use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::HashSet,
    marker::PhantomData,
    rc::Rc,
};

use crate::{
    SignalGet, SignalUpdate, SignalWith,
    effect::{EffectPriority, EffectTrait, observer_clean_up},
    id::Id,
    read::SignalTrack,
    runtime::{RUNTIME, Runtime},
    scope::Scope,
    signal::{ReadSignal, Signal, WriteSignal},
};

/// A lazily recomputed mapped signal backed by a local cached value.
///
/// This behaves like a [`Memo`](crate::Memo), but it always propagates updates
/// when its dependencies invalidate, even if the mapped value is equal.
pub struct MappedSignal<T: 'static, O: 'static, S, F>
where
    S: SignalWith<T> + Copy + 'static,
    F: Fn(&T) -> O + Clone + 'static,
{
    getter: ReadSignal<O>,
    mapped_id: Id,
    _phantom: PhantomData<(T, S, F)>,
}

impl<T: 'static, O: 'static, S, F> Copy for MappedSignal<T, O, S, F>
where
    S: SignalWith<T> + Copy + 'static,
    F: Fn(&T) -> O + Clone + 'static,
{
}

impl<T: 'static, O: 'static, S, F> Clone for MappedSignal<T, O, S, F>
where
    S: SignalWith<T> + Copy + 'static,
    F: Fn(&T) -> O + Clone + 'static,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static, O: Clone + 'static, S, F> SignalGet<O> for MappedSignal<T, O, S, F>
where
    S: SignalWith<T> + Copy + 'static,
    F: Fn(&T) -> O + Clone + 'static,
{
    fn id(&self) -> crate::id::Id {
        self.getter.id
    }

    fn get_untracked(&self) -> O
    where
        O: 'static,
    {
        self.ensure_fresh();
        self.getter.get_untracked()
    }

    fn get(&self) -> O
    where
        O: 'static,
    {
        self.ensure_fresh();
        self.getter.get()
    }

    fn try_get(&self) -> Option<O>
    where
        O: 'static,
    {
        self.ensure_fresh();
        self.getter.try_get()
    }

    fn try_get_untracked(&self) -> Option<O>
    where
        O: 'static,
    {
        self.ensure_fresh();
        self.getter.try_get_untracked()
    }
}

impl<T: 'static, O: 'static, S, F> SignalTrack<O> for MappedSignal<T, O, S, F>
where
    S: SignalWith<T> + Copy + 'static,
    F: Fn(&T) -> O + Clone + 'static,
{
    fn id(&self) -> crate::id::Id {
        self.getter.id
    }
}

impl<T: 'static, O: 'static, S, F> SignalWith<O> for MappedSignal<T, O, S, F>
where
    S: SignalWith<T> + Copy + 'static,
    F: Fn(&T) -> O + Clone + 'static,
{
    fn id(&self) -> crate::id::Id {
        self.getter.id
    }

    fn with<O2>(&self, f: impl FnOnce(&O) -> O2) -> O2
    where
        O: 'static,
    {
        self.ensure_fresh();
        self.getter.with(f)
    }

    fn with_untracked<O2>(&self, f: impl FnOnce(&O) -> O2) -> O2
    where
        O: 'static,
    {
        self.ensure_fresh();
        self.getter.with_untracked(f)
    }

    fn try_with<O2>(&self, f: impl FnOnce(Option<&O>) -> O2) -> O2
    where
        O: 'static,
    {
        self.ensure_fresh();
        self.getter.try_with(f)
    }

    fn try_with_untracked<O2>(&self, f: impl FnOnce(Option<&O>) -> O2) -> O2
    where
        O: 'static,
    {
        self.ensure_fresh();
        self.getter.try_with_untracked(f)
    }
}

impl<T: 'static, O: 'static, S, F> MappedSignal<T, O, S, F>
where
    S: SignalWith<T> + Copy + 'static,
    F: Fn(&T) -> O + Clone + 'static,
{
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn new(source: S, map: F) -> Self {
        Runtime::assert_ui_thread();

        let mapped_id = Id::next();
        let state = Rc::new(MappedSignalState::new(mapped_id, source, map));

        mapped_id.set_scope();
        let effect: Rc<dyn EffectTrait> = state.clone();
        RUNTIME.with(|runtime| runtime.register_effect(&effect));

        let initial = state.compute_initial();
        let (getter, setter) = Signal::new_split(initial);
        state.set_signal(getter, setter);
        state.mark_clean();

        Self { getter, mapped_id, _phantom: PhantomData }
    }

    fn ensure_fresh(&self) {
        self.with_state(|state| state.ensure_fresh());
    }

    fn with_state<R>(&self, f: impl FnOnce(&MappedSignalState<T, O, S, F>) -> R) -> Option<R> {
        RUNTIME.with(|runtime| {
            runtime.get_effect(self.mapped_id).and_then(|effect| {
                effect.as_any().downcast_ref::<MappedSignalState<T, O, S, F>>().map(f)
            })
        })
    }
}

struct MappedSignalState<T: 'static, O: 'static, S, F>
where
    S: SignalWith<T> + Copy + 'static,
    F: Fn(&T) -> O + Clone + 'static,
{
    id: Id,
    source: S,
    map: F,
    setter: RefCell<Option<WriteSignal<O>>>,
    dirty: Cell<bool>,
    observers: RefCell<HashSet<Id>>,
    _phantom: PhantomData<T>,
}

impl<T: 'static, O: 'static, S, F> MappedSignalState<T, O, S, F>
where
    S: SignalWith<T> + Copy + 'static,
    F: Fn(&T) -> O + Clone + 'static,
{
    fn new(id: Id, source: S, map: F) -> Self {
        Self {
            id,
            source,
            map,
            setter: RefCell::new(None),
            dirty: Cell::new(true),
            observers: RefCell::new(HashSet::new()),
            _phantom: PhantomData,
        }
    }

    fn compute_initial(&self) -> O {
        let effect = RUNTIME
            .with(|runtime| runtime.get_effect(self.id))
            .expect("mapped signal registered before initial compute");

        let prev_effect =
            RUNTIME.with(|runtime| runtime.current_effect.borrow_mut().replace(effect));
        let scope = Scope(self.id, PhantomData);
        let value = scope.enter(|| self.source.with(|value| (self.map)(value)));

        RUNTIME.with(|runtime| *runtime.current_effect.borrow_mut() = prev_effect);
        value
    }

    fn set_signal(&self, _: ReadSignal<O>, setter: WriteSignal<O>) {
        self.setter.replace(Some(setter));
    }

    fn mark_clean(&self) {
        self.dirty.set(false);
    }

    fn ensure_fresh(&self) {
        if !self.dirty.get() {
            return;
        }

        self.recompute();
    }

    fn recompute(&self) {
        Runtime::assert_ui_thread();
        let effect =
            RUNTIME.with(|runtime| runtime.get_effect(self.id)).expect("mapped signal registered");

        observer_clean_up(&effect);

        let prev_effect =
            RUNTIME.with(|runtime| runtime.current_effect.borrow_mut().replace(effect));
        let scope = Scope(self.id, PhantomData);
        let new_value = scope.enter(|| self.source.with(|value| (self.map)(value)));
        RUNTIME.with(|runtime| *runtime.current_effect.borrow_mut() = prev_effect);

        if let Some(setter) = self.setter.borrow().as_ref() {
            setter.set(new_value);
        }

        self.dirty.set(false);
    }
}

impl<T: 'static, O: 'static, S, F> Drop for MappedSignalState<T, O, S, F>
where
    S: SignalWith<T> + Copy + 'static,
    F: Fn(&T) -> O + Clone + 'static,
{
    fn drop(&mut self) {
        if RUNTIME.try_with(|runtime| runtime.remove_effect(self.id)).is_ok() {
            self.id.dispose();
        }
    }
}

impl<T: 'static, O: 'static, S, F> EffectTrait for MappedSignalState<T, O, S, F>
where
    S: SignalWith<T> + Copy + 'static,
    F: Fn(&T) -> O + Clone + 'static,
{
    fn id(&self) -> Id {
        self.id
    }

    fn run(&self) -> bool {
        self.dirty.set(true);
        self.ensure_fresh();
        true
    }

    fn add_observer(&self, id: Id) {
        self.observers.borrow_mut().insert(id);
    }

    fn clear_observers(&self) -> HashSet<Id> {
        std::mem::take(&mut *self.observers.borrow_mut())
    }

    fn priority(&self) -> EffectPriority {
        EffectPriority::High
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, rc::Rc};

    use crate::{
        Effect, Signal, SignalGet, SignalMapExt, SignalUpdate, runtime::Runtime, scope::Scope,
    };

    #[test]
    fn mapped_signal_recomputes_lazily() {
        let scope = Scope::new();
        let compute_count = Rc::new(Cell::new(0));

        scope.enter(|| {
            let source = Signal::new(2);
            let capture = Rc::new(String::from("mapped"));
            let capture_for_map = capture.clone();
            let compute_count_for_map = compute_count.clone();
            let mapped = source.map(move |value| {
                compute_count_for_map.set(compute_count_for_map.get() + 1);
                value + capture_for_map.len() as i32
            });

            assert_eq!(compute_count.get(), 1);
            assert_eq!(mapped.get(), 8);
            assert_eq!(compute_count.get(), 1);

            source.set(5);
            Runtime::drain_pending_work();
            assert_eq!(compute_count.get(), 2);

            assert_eq!(mapped.get(), 11);
            assert_eq!(compute_count.get(), 2);
        });
    }

    #[test]
    fn mapped_signal_notifies_even_when_value_is_equal() {
        let scope = Scope::new();

        scope.enter(|| {
            let source = Signal::new(1);
            let mapped = source.map(|value| value % 2);
            let run_count = Rc::new(Cell::new(0));

            let run_count_for_effect = run_count.clone();
            Effect::new(move |_| {
                mapped.get();
                run_count_for_effect.set(run_count_for_effect.get() + 1);
            });

            assert_eq!(run_count.get(), 1);

            source.set(3);
            Runtime::drain_pending_work();

            assert_eq!(run_count.get(), 2);
        });
    }
}

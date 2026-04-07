use std::{
    any::Any, cell::RefCell, collections::HashSet, fmt, marker::PhantomData, rc::Rc, sync::Arc,
};

use parking_lot::Mutex;

use crate::{
    Effect, UpdaterEffect,
    id::Id,
    memo::Memo,
    runtime::{RUNTIME, Runtime},
    signal::{ReadSignal, Signal, WriteSignal},
    state::{SignalState, SignalValue},
    sync_signal::{SyncReadSignal, SyncSignal, SyncWriteSignal},
};

/// You can manually control Signal's lifetime by using Scope.
///
/// Every Signal has a Scope created explicitly or implicitly,
/// and when you Dispose the Scope, it will clean up all the Signals
/// that belong to the Scope and all the child Scopes
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scope(pub(crate) Id, pub(crate) PhantomData<()>);

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Scope");
        s.field("id", &self.0);
        s.finish()
    }
}

impl Scope {
    /// Create a new Scope that isn't a child or parent of any scope
    pub fn new() -> Self {
        Self(Id::next(), PhantomData)
    }

    /// The current Scope in the Runtime. Any Signal/Effect/Memo created with
    /// implicitly Scope will be under this Scope
    pub fn current() -> Scope {
        RUNTIME.with(|runtime| Scope(*runtime.current_scope.borrow(), PhantomData))
    }

    /// Create a child Scope of this Scope
    pub fn create_child(&self) -> Scope {
        let child = Id::next();
        RUNTIME.with(|runtime| {
            runtime.children.borrow_mut().entry(self.0).or_default().insert(child);
            runtime.parents.borrow_mut().insert(child, self.0);
        });
        Scope(child, PhantomData)
    }

    /// Re-parent this scope to be a child of another scope.
    ///
    /// If this scope already has a parent, it will be removed from that parent first.
    /// This is useful when the scope hierarchy needs to be adjusted after construction
    /// to match the view hierarchy.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_reactive::Scope;
    /// let parent = Scope::new();
    /// let child = Scope::new(); // Initially has no parent
    ///
    /// child.set_parent(parent);
    /// // Now child is a child of parent, and will be disposed when parent is disposed
    /// ```
    pub fn set_parent(&self, new_parent: Scope) {
        RUNTIME.with(|runtime| {
            // Remove from old parent's children set (if any)
            if let Some(old_parent) = runtime.parents.borrow_mut().remove(&self.0)
                && let Some(children) = runtime.children.borrow_mut().get_mut(&old_parent)
            {
                children.remove(&self.0);
            }

            // Add to new parent's children set
            runtime.children.borrow_mut().entry(new_parent.0).or_default().insert(self.0);

            // Set new parent
            runtime.parents.borrow_mut().insert(self.0, new_parent.0);
        });
    }

    /// Returns the parent scope of this scope, if any.
    pub fn parent(&self) -> Option<Scope> {
        RUNTIME
            .with(|runtime| runtime.parents.borrow().get(&self.0).map(|id| Scope(*id, PhantomData)))
    }

    /// Create a new Signal under this Scope
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn create_split_signal<T>(self, value: T) -> (ReadSignal<T>, WriteSignal<T>)
    where
        T: Any + 'static,
    {
        self.enter(|| Signal::new_split(value))
    }

    /// Create a Signal under this Scope (local/unsync by default)
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn create_signal<T>(self, value: T) -> Signal<T>
    where
        T: Any + 'static,
    {
        self.enter(|| Signal::new(value))
    }

    /// Create a sync Signal under this Scope
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn create_sync_split_signal<T>(self, value: T) -> (SyncReadSignal<T>, SyncWriteSignal<T>)
    where
        T: Any + Send + Sync + 'static,
    {
        self.enter(|| SyncSignal::<T>::new_split(value))
    }

    /// Create a sync Signal under this Scope
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn create_sync_signal<T>(self, value: T) -> SyncSignal<T>
    where
        T: Any + Send + Sync + 'static,
    {
        self.enter(|| SyncSignal::<T>::new(value))
    }

    /// Create a local (unsync) Signal under this Scope
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn create_local_split_signal<T>(self, value: T) -> (ReadSignal<T>, WriteSignal<T>)
    where
        T: Any + 'static,
    {
        self.enter(|| Signal::new_split(value))
    }

    /// Create a local (unsync) Signal under this Scope
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn create_local_signal<T>(self, value: T) -> Signal<T>
    where
        T: Any + 'static,
    {
        self.enter(|| Signal::new(value))
    }

    /// Create a Memo under this Scope
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn create_memo<T>(self, f: impl Fn(Option<&T>) -> T + 'static) -> Memo<T>
    where
        T: PartialEq + 'static,
    {
        self.enter(|| Memo::new(f))
    }

    /// Create effect under this Scope
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn create_effect<T>(self, f: impl Fn(Option<T>) -> T + 'static)
    where
        T: Any + 'static,
    {
        self.enter(|| Effect::new(f))
    }

    /// Create updater under this Scope
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn create_updater<R>(
        self,
        compute: impl Fn() -> R + 'static,
        on_change: impl Fn(R) + 'static,
    ) -> R
    where
        R: 'static,
    {
        self.enter(|| UpdaterEffect::new(compute, on_change))
    }

    /// Runs the given closure within this scope.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn enter<T>(&self, f: impl FnOnce() -> T) -> T
    where
        T: 'static,
    {
        Runtime::assert_ui_thread();
        let prev_scope = RUNTIME.with(|runtime| {
            let mut current_scope = runtime.current_scope.borrow_mut();
            let prev_scope = *current_scope;
            *current_scope = self.0;
            prev_scope
        });

        let result = f();

        RUNTIME.with(|runtime| {
            *runtime.current_scope.borrow_mut() = prev_scope;
        });

        result
    }

    /// Wraps a closure so it runs under a new child scope of this scope.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn enter_child<T, U>(
        self,
        f: impl Fn(T) -> U + 'static,
    ) -> impl Fn(T) -> (U, Scope) + 'static
    where
        T: 'static,
    {
        Runtime::assert_ui_thread();
        let parent = self;
        move |t| {
            let scope = parent.create_child();
            let prev_scope = RUNTIME.with(|runtime| {
                let mut current_scope = runtime.current_scope.borrow_mut();
                let prev_scope = *current_scope;
                *current_scope = scope.0;
                prev_scope
            });

            let result = f(t);

            RUNTIME.with(|runtime| {
                *runtime.current_scope.borrow_mut() = prev_scope;
            });

            (result, scope)
        }
    }

    /// This is normally used by `Effect::new` and `Scope::create_effect`, and it binds the effect's lifetime
    /// to this scope
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn track(&self) {
        Runtime::assert_ui_thread();
        let signal = if let Some(signal) = self.0.signal() {
            signal
        } else {
            let signal = SignalState {
                id: self.0,
                subscribers: Arc::new(Mutex::new(HashSet::new())),
                value: SignalValue::Local(Rc::new(RefCell::new(()))),
            };
            self.0.add_signal(signal.clone());
            signal
        };
        signal.subscribe();
    }

    /// Dispose this Scope, and it will cleanup all the Signals and child Scope
    /// of this Scope.
    pub fn dispose(&self) {
        self.0.dispose();
    }
}

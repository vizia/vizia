use crate::prelude::*;
use crate::state::alloc_observer;
use std::collections::HashSet;
use std::marker::PhantomData;

pub trait DispatchState: 'static {
    type RegisterType;
    type LookupType;
    fn register(&mut self, entity: Entity, value: Self::RegisterType);
    fn lookup(
        &self,
        old: &Option<Self::LookupType>,
        new: &Option<Self::LookupType>,
    ) -> HashSet<Entity>;
}

pub struct DispatchView<L, T, S> {
    lens: L,
    state: S,
    old: Option<T>,
}

pub struct DispatchBinding<L> {
    content: Option<Box<dyn Fn(&mut Context, L)>>,
    lens: L,
}

#[derive(Debug)]
pub struct DispatchHandle<L, T, S>(Entity, PhantomData<DispatchView<L, T, S>>);

impl<L, T, S> Clone for DispatchHandle<L, T, S> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData::default())
    }
}

impl<L, T, S> Copy for DispatchHandle<L, T, S> {}

impl<L, T, S> DispatchView<L, T, S>
where
    L: Lens<Target = T>,
    S: DispatchState<LookupType = T> + Default,
    T: Data,
{
    pub fn new(
        cx: &mut Context,
        lens: L,
        content: impl FnOnce(&mut Context, DispatchHandle<L, T, S>),
    ) {
        let binding = Self { old: lens.get_fallible(cx), lens: lens.clone(), state: S::default() };
        let id = alloc_observer(cx, lens);
        cx.views.insert(id, Box::new(binding));

        cx.with_current(id, |cx| {
            content(cx, DispatchHandle(cx.current(), PhantomData::default()));
        });

        let _: Handle<Self> = Handle { entity: id, p: Default::default(), cx }.ignore();
    }
}

impl<L: Lens<Target = T>, T: Data, S: DispatchState<LookupType = T>> View
    for DispatchView<L, T, S>
{
    fn body(&mut self, cx: &mut Context) {
        let new = self.lens.get_fallible(cx);

        for observer in self.state.lookup(&self.old, &new) {
            if let Some(mut view) = cx.views.remove(&observer) {
                cx.with_current(observer, |cx| {
                    view.body(cx);
                });
                cx.views.insert(observer, view);
            }
        }

        self.old = new;
    }
}

impl<L: Lens> DispatchBinding<L> {
    pub fn new<F, T: Data, S: DispatchState<RegisterType = R>, R>(
        cx: &mut Context,
        register: R,
        handle: DispatchHandle<L, T, S>,
        content: F,
    ) where
        F: 'static + Fn(&mut Context, L),
    {
        let current = cx.current();
        let id = cx.entity_manager.create();
        cx.tree().add(id, current).expect("Failed to add to tree");
        cx.cache().add(id).expect("Failed to add to cache");
        cx.style().add(id);

        let lens = if let Some(view) = cx.views.get_mut(&handle.0) {
            if let Some(dview) = view.downcast_mut::<DispatchView<L, T, S>>() {
                dview.state.register(id, register);
                dview.lens.clone()
            } else {
                panic!("You seem to have manually constructed an invalid DispatchHandle.");
            }
        } else {
            panic!("Cannot find the view corresponding to a DispatchHandle");
        };

        let binding = Self { content: Some(Box::new(content)), lens };
        cx.views.insert(id, Box::new(binding));

        cx.with_current(id, |cx| {
            // Call the body of the binding
            if let Some(mut view_handler) = cx.views.remove(&id) {
                view_handler.body(cx);
                cx.views.insert(id, view_handler);
            }
        });

        let _: Handle<Self> = Handle { entity: id, p: Default::default(), cx }.ignore();
    }
}

impl<L: Lens> View for DispatchBinding<L> {
    fn body<'a>(&mut self, cx: &'a mut Context) {
        cx.remove_children(cx.current());
        if let Some(builder) = &self.content {
            (builder)(cx, self.lens.clone());
        }
    }
}

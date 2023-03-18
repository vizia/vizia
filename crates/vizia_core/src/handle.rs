use crate::prelude::*;
use std::marker::PhantomData;

/// A handle to a view which has been already built into the tree.
///
/// This type is part of the prelude.
pub struct Handle<'a, V> {
    pub entity: Entity,
    pub p: PhantomData<V>,
    pub cx: &'a mut Context,
}

impl<'a, V> Handle<'a, V> {
    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn ignore(self) -> Self {
        self.cx.tree.set_ignored(self.entity, true);
        self.focusable(false)
    }

    /// Stop the user from tabbing out of a subtree, which is useful for modal dialogs.
    pub fn lock_focus_to_within(self) -> Self {
        self.cx.tree.set_lock_focus_within(self.entity, true);
        self.cx.focus_stack.push(self.cx.focused);
        if !self.cx.focused.is_descendant_of(&self.cx.tree, self.entity) {
            let new_focus = vizia_storage::TreeIterator::subtree(&self.cx.tree, self.entity)
                .find(|node| crate::tree::is_focusable(self.cx, *node))
                .unwrap_or(self.cx.focus_stack.pop().unwrap());
            self.cx.with_current(new_focus, |cx| cx.focus());
        }
        self
    }

    pub fn modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
        V: 'static,
    {
        if let Some(view) = self
            .cx
            .views
            .get_mut(&self.entity)
            .and_then(|view_handler| view_handler.downcast_mut::<V>())
        {
            (f)(view);
        }

        self
    }

    /// Callback which is run when the view is built/rebuilt.
    pub fn on_build<F>(self, callback: F) -> Self
    where
        F: Fn(&mut EventContext),
    {
        let mut event_context = EventContext::new(self.cx);
        event_context.current = self.entity;
        (callback)(&mut event_context);

        self
    }

    pub fn bind<L, F>(self, lens: L, closure: F) -> Self
    where
        L: Lens,
        <L as Lens>::Target: Data,
        F: 'static + Fn(Handle<'_, V>, L),
    {
        let entity = self.entity();
        Binding::new(self.cx, lens, move |cx, data| {
            let new_handle = Handle { entity, p: Default::default(), cx };

            new_handle.cx.set_current(new_handle.entity);
            (closure)(new_handle, data);
        });
        self
    }
}

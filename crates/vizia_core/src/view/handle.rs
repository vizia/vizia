use crate::prelude::*;
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

/// A handle to a view which has been already built into the tree.
pub struct Handle<'a, V> {
    pub(crate) entity: Entity,
    pub(crate) p: PhantomData<V>,
    pub(crate) cx: &'a mut Context,
}

impl<'a, V> DataContext for Handle<'a, V> {
    fn data<T: 'static>(&self) -> Option<&T> {
        // Return data for the static model.
        if let Some(t) = <dyn Any>::downcast_ref::<T>(&()) {
            return Some(t);
        }

        for entity in self.entity.parent_iter(&self.cx.tree) {
            // Return any model data.
            if let Some(model_data_store) = self.cx.data.get(&entity) {
                if let Some(model) = model_data_store.models.get(&TypeId::of::<T>()) {
                    return model.downcast_ref::<T>();
                }
            }

            // Return any view data.
            if let Some(view_handler) = self.cx.views.get(&entity) {
                if let Some(data) = view_handler.downcast_ref::<T>() {
                    return Some(data);
                }
            }
        }

        None
    }
}

impl<'a, V> Handle<'a, V> {
    /// Returns the [`Entity`] id of the view.
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Returns a mutable reference to the context.
    pub fn context(&mut self) -> &mut Context {
        self.cx
    }

    pub fn parent(&self) -> Entity {
        self.cx.tree.get_parent(self.entity).unwrap_or(Entity::root())
    }

    /// Marks the view as being ignored.
    pub(crate) fn ignore(self) -> Self {
        self.cx.tree.set_ignored(self.entity, true);
        self.focusable(false)
    }

    /// Stop the user from tabbing out of a subtree, which is useful for modal dialogs.
    pub fn lock_focus_to_within(self) -> Self {
        self.cx.tree.set_lock_focus_within(self.entity, true);
        self.cx.focus_stack.push(self.cx.focused);
        if !self.cx.focused.is_descendant_of(&self.cx.tree, self.entity) {
            let new_focus = vizia_storage::TreeIterator::subtree(&self.cx.tree, self.entity)
                .find(|node| {
                    crate::tree::is_navigatable(
                        &self.cx.tree,
                        &self.cx.style,
                        *node,
                        Entity::root(),
                    )
                })
                .unwrap_or(self.cx.focus_stack.pop().unwrap());
            self.cx.with_current(new_focus, |cx| cx.focus());
        }
        self
    }

    /// Mody the internal data of the view.
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

    /// Marks the view as needing a relayout.
    pub fn needs_relayout(&mut self) {
        self.cx.needs_relayout();
    }

    /// Marks the view as needing a restyle.
    pub fn needs_restyle(&mut self) {
        self.cx.needs_restyle();
    }

    /// Marks the view as needing a redraw.
    pub fn needs_redraw(&mut self) {
        self.cx.needs_redraw();
    }

    /// Returns the bounding box of the view.
    pub fn bounds(&self) -> BoundingBox {
        self.cx.cache.get_bounds(self.entity)
    }

    /// Returns the scale factor of the device.
    pub fn scale_factor(&self) -> f32 {
        self.cx.scale_factor()
    }
}

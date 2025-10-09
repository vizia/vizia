use crate::context::LocalizationContext;
use crate::prelude::*;
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

/// A handle to a view which has been built into the tree.
pub struct Handle<'a, V> {
    pub(crate) current: Entity,
    pub(crate) entity: Entity,
    pub(crate) p: PhantomData<V>,
    pub(crate) cx: &'a mut Context,
}

impl<V> DataContext for Handle<'_, V> {
    fn data<T: 'static>(&self) -> Option<&T> {
        // Return data for the static model.
        if let Some(t) = <dyn Any>::downcast_ref::<T>(&()) {
            return Some(t);
        }

        for entity in self.entity.parent_iter(&self.cx.tree) {
            // Return any model data.
            if let Some(models) = self.cx.models.get(&entity) {
                if let Some(model) = models.get(&TypeId::of::<T>()) {
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

    fn localization_context(&self) -> Option<LocalizationContext<'_>> {
        Some(LocalizationContext::from_context(self.cx))
    }

    fn store(&self) -> &crate::recoil::Store {
        self.cx.data.get_store()
    }
}

impl<V> Handle<'_, V> {
    /// Returns the [`Entity`] id of the view.
    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub(crate) fn current(&self) -> Entity {
        self.current
    }

    /// Returns a mutable reference to the context.
    pub fn context(&mut self) -> &mut Context {
        self.cx
    }

    ///  Returns the entity id of the parent view.
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
    pub fn modify<F>(mut self, f: F) -> Self
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

        // Send an event to force the modification to happen within the same event loop.
        self.context().emit(WindowEvent::Redraw);

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

    /// Creates a binding to the given lens and provides a closure which can be used to mutate the view through a handle.
    pub fn bind<R, T, F>(self, res: R, closure: F) -> Self
    where
        R: Res<T>,
        F: 'static + Fn(Handle<'_, V>, R),
    {
        let entity = self.entity();
        let current = self.current();
        self.cx.with_current(current, |cx| {
            res.set_or_bind(cx, entity, move |cx, r| {
                let new_handle = Handle { entity, current: cx.current, p: Default::default(), cx };
                // new_handle.cx.set_current(new_handle.entity);
                (closure)(new_handle, r);
            });
        });
        self
    }

    /// Marks the view as needing a relayout.
    pub fn needs_relayout(&mut self) {
        self.cx.needs_relayout();
    }

    /// Marks the view as needing a restyle.
    pub fn needs_restyle(&mut self) {
        self.cx.needs_restyle(self.entity);
    }

    /// Marks the view as needing a redraw.
    pub fn needs_redraw(&mut self) {
        self.cx.needs_redraw(self.entity);
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

impl<V> AsMut<Context> for Handle<'_, V> {
    fn as_mut(&mut self) -> &mut Context {
        self.context()
    }
}

use super::internal;
use crate::prelude::*;

/// Modifiers for changing the accessibility properties of a view.
pub trait AccessibilityModifiers: internal::Modifiable {
    /// Sets the accessibility role of the view.
    fn role(mut self, role: Role) -> Self {
        let id = self.entity();

        self.context().style.role.insert(id, role);

        self.context().style.needs_access_update(id);

        self
    }

    /// Sets the accessibility name of the view.
    fn name<U>(mut self, name: impl Res<U> + 'static) -> Self
    where
        U: Clone + ToStringLocalized + 'static,
    {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, name, move |cx, value| {
            cx.style.name.insert(entity, value.to_string_local(cx));
            cx.style.needs_access_update(entity);
        });

        self
    }

    // /// Sets the accessibility default action for the view.
    // fn default_action_verb(mut self, action_verb: DefaultActionVerb) -> Self {
    //     let id = self.entity();

    //     self.context().style.default_action_verb.insert(id, action_verb);
    //     self.context().style.needs_access_update(id);

    //     self
    // }

    /// Sets whether the view should act as an accessibility live region.
    fn live(mut self, live: Live) -> Self {
        let id = self.entity();

        self.context().style.live.insert(id, live);
        self.context().style.needs_access_update(id);

        self
    }

    /// Sets whether the view should be hidden from accessibility.
    fn hidden(mut self, hidden: impl Res<bool> + 'static) -> Self {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, hidden, move |cx, value| {
            cx.style.hidden.insert(cx.current, *value);
            cx.style.needs_access_update(cx.current);
        });

        self
    }

    /// Sets the accessibility numeric value for the view.
    fn numeric_value<U>(mut self, value: impl Res<U> + 'static) -> Self
    where
        U: Clone + Into<f64> + 'static,
    {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, value, move |cx, val| {
            let v = val.clone().into();
            cx.style.numeric_value.insert(cx.current, v);
            cx.style.needs_access_update(cx.current);
        });

        self
    }

    /// Sets the accessibility text value for the view.
    fn text_value<U>(mut self, value: impl Res<U> + 'static) -> Self
    where
        U: Clone + ToStringLocalized + 'static,
    {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, value, move |cx, val| {
            cx.style.text_value.insert(cx.current, val.to_string_local(cx));
            cx.style.needs_access_update(cx.current);
        });

        self
    }
}

impl<V: View> AccessibilityModifiers for Handle<'_, V> {}

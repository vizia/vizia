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
    fn name<U: ToStringLocalized>(mut self, name: impl Res<U>) -> Self {
        let entity = self.entity();
        name.set_or_bind(self.context(), entity, |cx, name| {
            cx.style.name.insert(cx.current, name.to_string_local(cx));
            cx.style.needs_access_update(cx.current);
        });

        self
    }

    /// Sets the accessibility default action for the view.
    fn default_action_verb(mut self, action_verb: DefaultActionVerb) -> Self {
        let id = self.entity();

        self.context().style.default_action_verb.insert(id, action_verb);
        self.context().style.needs_access_update(id);

        self
    }

    /// Sets whether the view should act as an accessibility live region.
    fn live(mut self, live: Live) -> Self {
        let id = self.entity();

        self.context().style.live.insert(id, live);
        self.context().style.needs_access_update(id);

        self
    }

    /// Sets whether the view should be hidden from accessibility.
    fn hidden<U: Into<bool>>(mut self, hidden: impl Res<U>) -> Self {
        let entity = self.entity();
        hidden.set_or_bind(self.context(), entity, |cx, hidden| {
            cx.style.hidden.insert(cx.current, hidden.into());
            cx.style.needs_access_update(cx.current);
        });

        self
    }

    /// Sets the accessibility numeric value for the view.
    fn numeric_value<U: Into<f64>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, val| {
            let v = val.into();

            cx.style.numeric_value.insert(cx.current, v);
            cx.style.needs_access_update(cx.current);
        });

        self
    }

    /// Sets the accessibility text value for the view.
    fn text_value<U: ToString>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, val| {
            cx.style.text_value.insert(cx.current, val.to_string());
            cx.style.needs_access_update(cx.current);
        });

        self
    }
}

impl<'a, V: View> AccessibilityModifiers for Handle<'a, V> {}

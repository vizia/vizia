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
    fn name<U: ToString>(mut self, name: impl Res<U>) -> Self {
        let entity = self.entity();
        name.set_or_bind(self.context(), entity, |cx, id, name| {
            cx.style.name.insert(id, name.get_val(cx).to_string());
            cx.style.needs_access_update(id);
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
        hidden.set_or_bind(self.context(), entity, |cx, id, hidden| {
            cx.style.hidden.insert(id, hidden.get_val(cx).into());
            cx.style.needs_access_update(id);
        });

        self
    }

    /// Sets the accessibility numeric value for the view.
    fn numeric_value<U: Into<f64>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, id, val| {
            let v = val.get_val(cx).into();

            cx.style.numeric_value.insert(id, v);
            cx.style.needs_access_update(id);
        });

        self
    }

    /// Sets the accessibility text value for the view.
    fn text_value<U: ToString>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, id, val| {
            cx.style.text_value.insert(id, val.get_val(cx).to_string());
            cx.style.needs_access_update(id);
        });

        self
    }
}

impl<'a, V: View> AccessibilityModifiers for Handle<'a, V> {}

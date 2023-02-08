use super::internal;
use crate::prelude::*;

pub trait AccessibilityModifiers: internal::Modifiable {
    fn role(mut self, role: Role) -> Self {
        let id = self.entity();

        if let Some(node_builder) = self.context().style.accesskit_node_builders.get_mut(id) {
            node_builder.set_role(role);
        }

        self
    }

    fn name<U: ToString>(mut self, name: impl Res<U>) -> Self {
        let entity = self.entity();
        name.set_or_bind(self.context(), entity, |cx, id, name| {
            // println!("set name for: {} {}", id, name.to_string());
            if let Some(node_builder) = cx.style.accesskit_node_builders.get_mut(id) {
                node_builder.set_name(name.to_string().into_boxed_str());
            }
        });

        self
    }

    fn default_action_verb(mut self, action_verb: DefaultActionVerb) -> Self {
        let id = self.entity();
        if let Some(node_builder) = self.context().style.accesskit_node_builders.get_mut(id) {
            node_builder.set_default_action_verb(action_verb);
        }

        self
    }

    fn live(mut self, live: Live) -> Self {
        let id = self.entity();

        if let Some(node_builder) = self.context().style.accesskit_node_builders.get_mut(id) {
            node_builder.set_live(live);
        }
        self
    }

    fn hidden<U: Into<bool>>(mut self, hidden: impl Res<U>) -> Self {
        let entity = self.entity();
        hidden.set_or_bind(self.context(), entity, |cx, id, hidden| {
            if let Some(node_builder) = cx.style.accesskit_node_builders.get_mut(id) {
                if hidden.into() {
                    node_builder.set_hidden();
                } else {
                    node_builder.clear_hidden();
                }
            }
        });

        self
    }

    fn numeric_value<U: Into<f64>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, id, val| {
            let v = val.into();
            if let Some(node_builder) = cx.style.accesskit_node_builders.get_mut(id) {
                node_builder.set_numeric_value(v);
            }
        });

        self
    }

    fn text_value<U: ToString>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, id, val| {
            if let Some(node_builder) = cx.style.accesskit_node_builders.get_mut(id) {
                node_builder.set_value(val.to_string().into_boxed_str());
            }
        });

        self
    }
}

impl<'a, V: View> AccessibilityModifiers for Handle<'a, V> {}

use super::internal;
use crate::prelude::*;
use accesskit::SortDirection;

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
        let current = self.current();
        self.context().with_current(current, move |cx| {
            name.set_or_bind(cx, move |cx, name| {
                cx.style.name.insert(entity, name.get_value(cx).to_string_local(cx));
                cx.style.needs_access_update(entity);
            });
        });

        self
    }

    /// Sets the accessibility label relationship for the view using the ID of another view.
    fn labeled_by(mut self, id: impl Into<String>) -> Self {
        let entity = self.entity();
        let id = id.into();

        self.context().style.labelled_by.insert(entity, id);
        self.context().style.needs_access_update(entity);

        self
    }

    /// Sets the accessibility description relationship for the view using the ID of another view.
    fn described_by(mut self, id: impl Into<String>) -> Self {
        let entity = self.entity();
        let id = id.into();

        self.context().style.described_by.insert(entity, id);
        self.context().style.needs_access_update(entity);

        self
    }

    /// Sets the accessibility controls relationship for the view using the ID of another view.
    fn controls(mut self, id: impl Into<String>) -> Self {
        let entity = self.entity();
        let id = id.into();

        self.context().style.controls.insert(entity, id);
        self.context().style.needs_access_update(entity);

        self
    }

    /// Sets the accessibility active descendant relationship for the view.
    fn active_descendant<U: Into<String> + Clone + 'static>(
        mut self,
        id: impl Res<U> + 'static,
    ) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            id.set_or_bind(cx, move |cx, id| {
                cx.style.active_descendant.insert(entity, id.get_value(cx).into());
                cx.style.needs_access_update(entity);
            });
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
    fn hidden<U: Into<bool>>(mut self, hidden: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            hidden.set_or_bind(cx, move |cx, hidden| {
                cx.style.hidden.insert(entity, hidden.get_value(cx).into());
                cx.style.needs_access_update(entity);
            });
        });

        self
    }

    /// Sets whether the view should be announced as expanded (`true`) or collapsed (`false`).
    fn expanded<U: Into<bool>>(mut self, expanded: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            expanded.set_or_bind(cx, move |cx, expanded| {
                cx.style.expanded.insert(entity, expanded.get_value(cx).into());
                cx.style.needs_access_update(entity);
            });
        });

        self
    }

    /// Sets whether the view should be announced as selected (`true`) or not selected (`false`).
    fn accessibility_selected<U: Into<bool>>(mut self, selected: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            selected.set_or_bind(cx, move |cx, selected| {
                cx.style.selected.insert(entity, selected.get_value(cx).into());
                cx.style.needs_access_update(entity);
            });
        });

        self
    }

    /// Sets whether the view should be announced as selected (`true`) or not selected (`false`).
    ///
    /// This is an alias of [`AccessibilityModifiers::accessibility_selected`].
    fn selected<U: Into<bool>>(self, selected: impl Res<U>) -> Self {
        self.accessibility_selected(selected)
    }

    /// Sets whether the view allows multiple selected descendants.
    fn multiselectable<U: Into<bool>>(mut self, multiselectable: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            multiselectable.set_or_bind(cx, move |cx, multiselectable| {
                cx.style.multiselectable.insert(entity, multiselectable.get_value(cx).into());
                cx.style.needs_access_update(entity);
            });
        });

        self
    }

    /// Sets the accessibility sort direction for sortable headers.
    ///
    /// Use `None` when a header is not currently sorted.
    fn sort_direction(mut self, sort_direction: impl Res<Option<SortDirection>>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            sort_direction.set_or_bind(cx, move |cx, sort_direction| {
                if let Some(value) = sort_direction.get_value(cx) {
                    cx.style.sort_direction.insert(entity, value);
                } else {
                    cx.style.sort_direction.remove(entity);
                }
                cx.style.needs_access_update(entity);
            });
        });

        self
    }

    /// Sets the accessibility level for hierarchical items.
    fn level<U: Into<usize>>(mut self, level: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            level.set_or_bind(cx, move |cx, level| {
                cx.style.level.insert(entity, level.get_value(cx).into());
                cx.style.needs_access_update(entity);
            });
        });

        self
    }

    /// Sets the total number of sibling items in a set for this item.
    fn size_of_set<U: Into<usize>>(mut self, size_of_set: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            size_of_set.set_or_bind(cx, move |cx, size_of_set| {
                cx.style.size_of_set.insert(entity, size_of_set.get_value(cx).into());
                cx.style.needs_access_update(entity);
            });
        });

        self
    }

    /// Sets the 1-based position of this item within its sibling set.
    fn position_in_set<U: Into<usize>>(mut self, position_in_set: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            position_in_set.set_or_bind(cx, move |cx, position_in_set| {
                cx.style.position_in_set.insert(entity, position_in_set.get_value(cx).into());
                cx.style.needs_access_update(entity);
            });
        });

        self
    }

    /// Sets the accessibility orientation of the view.
    /// This does not affect the layout of the view, but is used to inform
    /// assistive technologies of the orientation of the view.
    fn orientation<U: Into<Orientation>>(mut self, orientation: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            orientation.set_or_bind(cx, move |cx, orientation| {
                let orientation_value = orientation.get_value(cx).into();

                if orientation_value == Orientation::Horizontal {
                    cx.with_current(entity, |cx| {
                        cx.toggle_class("horizontal", true);
                        cx.toggle_class("vertical", false);
                    });
                } else {
                    cx.with_current(entity, |cx| {
                        cx.toggle_class("horizontal", false);
                        cx.toggle_class("vertical", true);
                    });
                }
                cx.style.orientation.insert(entity, orientation_value);
                cx.style.needs_access_update(entity);
            });
        });

        self
    }

    /// Sets the accessibility numeric value for the view.
    fn numeric_value<U: Into<f64>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, move |cx, val| {
                let v = val.get_value(cx).into();

                cx.style.numeric_value.insert(entity, v);
                cx.style.needs_access_update(entity);
            });
        });

        self
    }

    /// Sets the accessibility text value for the view.
    fn text_value<U: ToStringLocalized>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, move |cx| {
            value.set_or_bind(cx, move |cx, val| {
                cx.style.text_value.insert(entity, val.get_value(cx).to_string_local(cx));
                cx.style.needs_access_update(entity);
            });
        });

        self
    }
}

impl<V: View> AccessibilityModifiers for Handle<'_, V> {}

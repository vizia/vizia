use super::internal;
use crate::prelude::*;
use crate::style::{PseudoClass, SystemFlags};

/// Modifiers for changing the style properties of a view.
pub trait StyleModifiers: internal::Modifiable {
    // Selectors

    /// Sets the ID name of the view.
    ///
    /// The ID name can be references by a CSS selector.
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let cx = &mut Context::default();
    /// Element::new(cx).id("foo");
    /// ```
    /// css
    /// ```css
    /// #foo {
    ///     background-color: red;
    /// }
    ///```
    fn id(mut self, id: impl Into<String>) -> Self {
        let id = id.into();
        let entity = self.entity();
        self.context().style.ids.insert(entity, id.clone()).expect("Could not insert id");
        self.context().needs_restyle();

        self.context().entity_identifiers.insert(id, entity);

        self
    }

    /// Adds a class name to the view.
    fn class(mut self, name: &str) -> Self {
        let entity = self.entity();
        if let Some(class_list) = self.context().style.classes.get_mut(entity) {
            class_list.insert(name.to_string());
        }

        self.context().needs_restyle();

        self
    }

    /// Sets whether a view should have the given class name.
    fn toggle_class(mut self, name: &str, applied: impl Res<bool>) -> Self {
        let name = name.to_owned();
        let entity = self.entity();
        applied.set_or_bind(self.context(), entity, move |cx, entity, applied| {
            if let Some(class_list) = cx.style.classes.get_mut(entity) {
                if applied {
                    class_list.insert(name.clone());
                } else {
                    class_list.remove(&name);
                }
            }

            cx.needs_restyle();
        });

        self
    }

    // Pseudoclass
    // TODO: Should these have their own modifiers trait?

    /// Sets the state of the view to checked.
    fn checked<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        state.set_or_bind(self.context(), entity, |cx, entity, val| {
            if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(entity) {
                pseudo_classes.set(PseudoClass::CHECKED, val.into());
            } else {
                let mut pseudoclass = PseudoClass::empty();
                pseudoclass.set(PseudoClass::CHECKED, val.into());
                cx.style.pseudo_classes.insert(entity, pseudoclass).unwrap();
            }

            cx.needs_restyle();
        });

        self
    }

    modifier!(
        /// Sets the view to be disabled.
        ///
        /// This property is inherited by the descendants of the view.
        disabled,
        bool,
        SystemFlags::RESTYLE
    );

    modifier!(
        /// Sets whether the view should be positioned and rendered.
        ///
        /// A display value of `Display::None` causes the view to be ignored by both layout and rendering.
        display,
        Display,
        SystemFlags::REHIDE | SystemFlags::RELAYOUT | SystemFlags::REDRAW
    );

    modifier!(
        /// Sets whether the view should be rendered.
        ///
        /// The layout system will still compute the size and position of an invisible view.
        visibility,
        Visibility,
        SystemFlags::REHIDE | SystemFlags::REDRAW
    );

    /// Sets the z-order index of the view.
    ///
    /// Views with a higher z-order will be rendered on top of those with a lower z-order.
    /// Views with the same z-order are rendered in tree order.
    fn z_order<U: Into<i32>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.tree.set_z_order(entity, value);
            cx.needs_redraw();
        });

        self
    }

    modifier!(
        /// Sets the overflow behavior of the view.
        ///
        /// The overflow behavior determines whether child views can render outside the bounds of their parent.
        overflow,
        Overflow,
        SystemFlags::RECLIP | SystemFlags::REDRAW
    );

    // Background Properties
    modifier!(
        /// Sets the background color of the view.
        background_color,
        Color,
        SystemFlags::REDRAW
    );
    modifier!(
        /// Sets the background image of the view.
        ///
        /// Background image will override any background gradient or color.
        background_image,
        String,
        SystemFlags::REDRAW
    );

    // TODO: Docs for this.
    fn image<U: ToString>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, val| {
            let val = val.to_string();
            if let Some(prev_data) = cx.style.image.get(entity) {
                if prev_data != &val {
                    cx.style.image.insert(entity, val);
                    cx.style.needs_text_layout.insert(entity, true).unwrap();
                    cx.needs_redraw();
                }
            } else {
                cx.style.image.insert(entity, val);
                cx.style.needs_text_layout.insert(entity, true).unwrap();
                cx.needs_redraw();
            }
        });

        self
    }

    // Border Properties
    modifier!(
        /// Sets the border width of the view.
        border_width,
        Units,
        SystemFlags::RELAYOUT | SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border color of the view.
        border_color,
        Color,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border radius for the top-left corner of the view.
        border_radius_top_left,
        Units,
        SystemFlags::REDRAW
    );
    modifier!(
        /// Sets the border radius for the top-right corner of the view.
        border_radius_top_right,
        Units,
        SystemFlags::REDRAW
    );
    modifier!(
        /// Sets the border radius for the bottom-left corner of the view.
        border_radius_bottom_left,
        Units,
        SystemFlags::REDRAW
    );
    modifier!(
        /// Sets the border radius for the bottom-right corner of the view.
        border_radius_bottom_right,
        Units,
        SystemFlags::REDRAW
    );

    /// Sets the border radius for all four corners of the view.
    fn border_radius<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.border_radius_top_left.insert(entity, value);
            cx.style.border_radius_top_right.insert(entity, value);
            cx.style.border_radius_bottom_left.insert(entity, value);
            cx.style.border_radius_bottom_right.insert(entity, value);

            cx.needs_redraw();
        });

        self
    }

    modifier!(
        /// Sets the border corner shape for the top-left corner of the view.
        border_shape_top_left,
        BorderCornerShape,
        SystemFlags::REDRAW
    );
    modifier!(
        /// Sets the border corner shape for the top-right corner of the view.
        border_shape_top_right,
        BorderCornerShape,
        SystemFlags::REDRAW
    );
    modifier!(
        /// Sets the border corner shape for the bottom-left corner of the view.
        border_shape_bottom_left,
        BorderCornerShape,
        SystemFlags::REDRAW
    );
    modifier!(
        /// Sets the border corner shape for the bottom-right corner of the view.
        border_shape_bottom_right,
        BorderCornerShape,
        SystemFlags::REDRAW
    );

    /// Sets the border corner shape for all four corners of the view.
    fn border_corner_shape<U: Into<BorderCornerShape>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.border_shape_top_left.insert(entity, value);
            cx.style.border_shape_top_right.insert(entity, value);
            cx.style.border_shape_bottom_left.insert(entity, value);
            cx.style.border_shape_bottom_right.insert(entity, value);

            cx.needs_redraw();
        });

        self
    }

    // Outine Properties
    modifier!(
        /// Sets the outline width of the view.
        outline_width,
        Units,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the outline color of the view.
        outline_color,
        Color,
        SystemFlags::REDRAW
    );
    modifier!(
        /// Sets the outline offset of the view.
        outline_offset,
        Units,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the mouse cursor used when the view is hovered.
        cursor,
        CursorIcon,
        SystemFlags::empty()
    );

    // Transform Properties
    modifier!(
        /// Sets the angle of rotation for the view.
        ///
        /// Rotation applies to the rendered view and does not affect layout.
        rotate,
        f32,
        SystemFlags::RETRANSFORM | SystemFlags::REDRAW
    );
    modifier!(
        /// Sets the translation offset of the view.
        ///
        /// Translation applies to the rendered view and does not affect layout.
        translate,
        (f32, f32),
        SystemFlags::RETRANSFORM | SystemFlags::REDRAW
    );
    modifier!(
        /// Sets the scale of the view.
        ///
        /// Scale applies to the rendered view and does not affect layout.
        scale,
        (f32, f32),
        SystemFlags::RETRANSFORM | SystemFlags::REDRAW
    );
}

impl<'a, V: View> StyleModifiers for Handle<'a, V> {}

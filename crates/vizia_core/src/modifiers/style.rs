use vizia_style::{BorderRadius, Rect};

use super::internal;
use crate::{entity, prelude::*};

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
        self.context().need_restyle();

        self.context().entity_identifiers.insert(id, entity);

        self
    }

    /// Adds a class name to the view.
    fn class(mut self, name: &str) -> Self {
        let entity = self.entity();
        if let Some(class_list) = self.context().style.classes.get_mut(entity) {
            class_list.insert(name.to_string());
        }

        self.context().need_restyle();

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

            cx.need_restyle();
        });

        self
    }

    // PseudoClassFlags
    // TODO: Should these have their own modifiers trait?

    /// Sets the state of the view to checked.
    fn checked<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        state.set_or_bind(self.context(), entity, |cx, entity, val| {
            if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(entity) {
                pseudo_classes.set(PseudoClassFlags::CHECKED, val.into());
            } else {
                let mut PseudoClassFlags = PseudoClassFlags::empty();
                PseudoClassFlags.set(PseudoClassFlags::CHECKED, val.into());
                cx.style.pseudo_classes.insert(entity, PseudoClassFlags).unwrap();
            }

            cx.need_restyle();
        });

        self
    }

    modifier!(
        /// Sets the view to be disabled.
        ///
        /// This property is inherited by the descendants of the view.
        disabled,
        bool
    );

    modifier!(
        /// Sets whether the view should be positioned and rendered.
        ///
        /// A display value of `Display::None` causes the view to be ignored by both layout and rendering.
        display,
        Display
    );

    modifier!(
        /// Sets whether the view should be rendered.
        ///
        /// The layout system will still compute the size and position of an invisible view.
        visibility,
        Visibility
    );

    modifier!(
        /// Sets the opacity of the view.
        opacity,
        Opacity
    );

    modifier!(
        /// Sets the z-order index of the view.
        ///
        /// Views with a higher z-order will be rendered on top of those with a lower z-order.
        /// Views with the same z-order are rendered in tree order.
        z_index,
        i32
    );

    modifier!(
        /// Sets the overflow behavior of the view.
        ///
        /// The overflow behavior determines whether child views can render outside the bounds of their parent.
        overflow,
        Overflow
    );

    // Background Properties
    modifier!(
        /// Sets the background color of the view.
        background_color,
        Color
    );

    fn background_image<'i, U: Into<Vec<BackgroundImage<'i>>>>(
        mut self,
        value: impl Res<U>,
    ) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, val| {
            let images = val.into();
            let gradients = images
                .into_iter()
                .filter_map(|img| match img {
                    BackgroundImage::Gradient(gradient) => Some(*gradient),
                    _ => None,
                })
                .collect::<Vec<_>>();
            cx.style.background_gradient.insert(entity, gradients);
            cx.need_redraw();
        });

        self
    }

    // TODO: Docs for this.
    fn image<U: ToString>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, val| {
            let val = val.to_string();
            if let Some(prev_data) = cx.style.image.get(entity) {
                if prev_data != &val {
                    cx.style.image.insert(entity, val);

                    cx.need_redraw();
                }
            } else {
                cx.style.image.insert(entity, val);

                cx.need_redraw();
            }
        });

        self
    }

    // Border Properties
    modifier!(
        /// Sets the border width of the view.
        border_width,
        LengthOrPercentage
    );

    modifier!(
        /// Sets the border color of the view.
        border_color,
        Color
    );

    modifier!(
        /// Sets the border radius for the top-left corner of the view.
        border_top_left_radius,
        LengthOrPercentage
    );
    modifier!(
        /// Sets the border radius for the top-right corner of the view.
        border_top_right_radius,
        LengthOrPercentage
    );
    modifier!(
        /// Sets the border radius for the bottom-left corner of the view.
        border_bottom_left_radius,
        LengthOrPercentage
    );
    modifier!(
        /// Sets the border radius for the bottom-right corner of the view.
        border_bottom_right_radius,
        LengthOrPercentage
    );

    /// Sets the border radius for all four corners of the view.
    fn border_radius<U: Into<BorderRadius>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.border_top_left_radius.insert(entity, value.top_left);
            cx.style.border_top_right_radius.insert(entity, value.top_right);
            cx.style.border_bottom_left_radius.insert(entity, value.bottom_left);
            cx.style.border_bottom_right_radius.insert(entity, value.bottom_right);

            cx.need_redraw();
        });

        self
    }

    modifier!(
        /// Sets the border corner shape for the top-left corner of the view.
        border_top_left_shape,
        BorderCornerShape
    );
    modifier!(
        /// Sets the border corner shape for the top-right corner of the view.
        border_top_right_shape,
        BorderCornerShape
    );
    modifier!(
        /// Sets the border corner shape for the bottom-left corner of the view.
        border_bottom_left_shape,
        BorderCornerShape
    );
    modifier!(
        /// Sets the border corner shape for the bottom-right corner of the view.
        border_bottom_right_shape,
        BorderCornerShape
    );

    /// Sets the border corner shape for all four corners of the view.
    fn border_corner_shape<U: Into<Rect<BorderCornerShape>>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.border_top_left_shape.insert(entity, value.0);
            cx.style.border_top_right_shape.insert(entity, value.1);
            cx.style.border_bottom_right_shape.insert(entity, value.2);
            cx.style.border_bottom_left_shape.insert(entity, value.3);

            cx.need_redraw();
        });

        self
    }

    // Outine Properties
    modifier!(
        /// Sets the outline width of the view.
        outline_width,
        LengthOrPercentage
    );

    modifier!(
        /// Sets the outline color of the view.
        outline_color,
        Color
    );
    modifier!(
        /// Sets the outline offset of the view.
        outline_offset,
        LengthOrPercentage
    );

    modifier!(
        /// Sets the mouse cursor used when the view is hovered.
        cursor,
        CursorIcon
    );

    // // Transform Properties
    // modifier!(
    //     /// Sets the angle of rotation for the view.
    //     ///
    //     /// Rotation applies to the rendered view and does not affect layout.
    //     rotate,
    //     f32
    // );
    // modifier!(
    //     /// Sets the translation offset of the view.
    //     ///
    //     /// Translation applies to the rendered view and does not affect layout.
    //     translate,
    //     (f32, f32)
    // );
    // modifier!(
    //     /// Sets the scale of the view.
    //     ///
    //     /// Scale applies to the rendered view and does not affect layout.
    //     scale,
    //     (f32, f32)
    // );
}

impl<'a, V: View> StyleModifiers for Handle<'a, V> {}

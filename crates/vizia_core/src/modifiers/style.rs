use vizia_style::{BorderRadius, Position, Rect, Scale, Transform, Translate};

use super::internal;
use crate::prelude::*;
use crate::style::{Abilities, ImageOrGradient, PseudoClassFlags, SystemFlags};

/// Modifiers for changing the style properties of a view.
pub trait StyleModifiers: internal::Modifiable {
    // Selectors

    /// Sets the ID name of the view.
    ///
    /// A view can have only one ID name and it must be unique.
    /// The ID name can be referenced by a CSS selector.
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
        // TODO - What should happen if the id already exists?
        let id = id.into();
        let entity = self.entity();
        self.context().style.ids.insert(entity, id.clone()).expect("Could not insert id");
        self.context().needs_restyle();

        self.context().entity_identifiers.insert(id, entity);

        self
    }

    /// Adds a class name to the view.
    ///
    /// A view can have multiple classes.
    /// The class name can be referenced by a CSS selector.
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let cx = &mut Context::default();
    /// Element::new(cx).class("foo");
    /// ```
    /// css
    /// ```css
    /// .foo {
    ///     background-color: red;
    /// }
    ///```
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

    // PseudoClassFlags
    // TODO: Should these have their own modifiers trait?

    /// Sets the state of the view to checked.
    fn checked<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        state.set_or_bind(self.context(), entity, |cx, entity, val| {
            let val = val.into();
            if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(entity) {
                pseudo_classes.set(PseudoClassFlags::CHECKED, val.into());
            }

            if val {
                // Setting a checked state should make it checkable... probably
                if let Some(abilities) = cx.style.abilities.get_mut(entity) {
                    abilities.set(Abilities::CHECKABLE, true);
                }
            }

            cx.needs_restyle();
        });

        self
    }

    fn read_only<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        state.set_or_bind(self.context(), entity, |cx, entity, val| {
            let val = val.into();
            if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(entity) {
                pseudo_classes.set(PseudoClassFlags::READ_ONLY, val.into());
            }

            cx.needs_restyle();
        });

        self
    }

    fn read_write<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        state.set_or_bind(self.context(), entity, |cx, entity, val| {
            let val = val.into();
            if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(entity) {
                pseudo_classes.set(PseudoClassFlags::READ_WRITE, val.into());
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
        SystemFlags::RELAYOUT | SystemFlags::REDRAW
    );

    modifier!(
        /// Sets whether the view should be rendered.
        ///
        /// The layout system will still compute the size and position of an invisible view.
        visibility,
        Visibility,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the opacity of the view.
        opacity,
        Opacity,
        SystemFlags::REDRAW
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

    fn overflow<U: Into<Overflow>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.overflowx.insert(entity, value);
            cx.style.overflowy.insert(entity, value);

            cx.needs_redraw();
        });

        self
    }

    modifier!(
        /// Sets the overflow behavior of the view in the horizontal direction.
        ///
        /// The overflow behavior determines whether child views can render outside the bounds of their parent.
        overflowx,
        Overflow,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the overflow behavior of the view in the vertical direction.
        ///
        /// The overflow behavior determines whether child views can render outside the bounds of their parent.
        overflowy,
        Overflow,
        SystemFlags::REDRAW
    );

    // Background Properties
    modifier!(
        /// Sets the background color of the view.
        background_color,
        Color,
        SystemFlags::REDRAW
    );

    fn background_image<'i, U: Into<Vec<BackgroundImage<'i>>>>(
        mut self,
        value: impl Res<U>,
    ) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, val| {
            let images = val.into();
            let images = images
                .into_iter()
                .filter_map(|img| match img {
                    BackgroundImage::Gradient(gradient) => {
                        Some(ImageOrGradient::Gradient(*gradient))
                    }
                    BackgroundImage::Url(url) => Some(ImageOrGradient::Image(url.url.to_string())),
                    _ => None,
                })
                .collect::<Vec<_>>();
            cx.style.background_image.insert(entity, images);
            cx.needs_redraw();
        });

        self
    }

    // Border Properties
    modifier!(
        /// Sets the border width of the view.
        border_width,
        LengthOrPercentage,
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
        border_top_left_radius,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border radius for the top-right corner of the view.
        border_top_right_radius,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border radius for the bottom-left corner of the view.
        border_bottom_left_radius,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border radius for the bottom-right corner of the view.
        border_bottom_right_radius,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    /// Sets the border radius for all four corners of the view.
    fn border_radius<U: std::fmt::Debug + Into<BorderRadius>>(
        mut self,
        value: impl Res<U>,
    ) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.border_top_left_radius.insert(entity, value.top_left);
            cx.style.border_top_right_radius.insert(entity, value.top_right);
            cx.style.border_bottom_left_radius.insert(entity, value.bottom_left);
            cx.style.border_bottom_right_radius.insert(entity, value.bottom_right);

            cx.needs_redraw();
        });

        self
    }

    modifier!(
        /// Sets the border corner shape for the top-left corner of the view.
        border_top_left_shape,
        BorderCornerShape,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border corner shape for the top-right corner of the view.
        border_top_right_shape,
        BorderCornerShape,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border corner shape for the bottom-left corner of the view.
        border_bottom_left_shape,
        BorderCornerShape,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border corner shape for the bottom-right corner of the view.
        border_bottom_right_shape,
        BorderCornerShape,
        SystemFlags::REDRAW
    );

    /// Sets the border corner shape for all four corners of the view.
    fn border_corner_shape<U: std::fmt::Debug + Into<Rect<BorderCornerShape>>>(
        mut self,
        value: impl Res<U>,
    ) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.border_top_left_shape.insert(entity, value.0);
            cx.style.border_top_right_shape.insert(entity, value.1);
            cx.style.border_bottom_right_shape.insert(entity, value.2);
            cx.style.border_bottom_left_shape.insert(entity, value.3);

            cx.needs_redraw();
        });

        self
    }

    // Outine Properties
    modifier!(
        /// Sets the outline width of the view.
        outline_width,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the outline color of the view.
        outline_color,
        Color,
        SystemFlags::REDRAW
    );

    // Outline Offset
    modifier!(
        /// Sets the outline offset of the view.
        outline_offset,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    // Cursor Icon
    modifier!(
        /// Sets the mouse cursor used when the view is hovered.
        cursor,
        CursorIcon,
        SystemFlags::empty()
    );

    /// Sets the transform of the view with a list of transform functions.
    fn transform<U: Into<Vec<Transform>>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.transform.insert(entity, value);
            cx.needs_redraw();
        });

        self
    }

    /// Sets the transform origin of the the view.
    fn transform_origin<U: Into<Position>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value: Position = v.into();
            let x = value.x.to_length_or_percentage();
            let y = value.y.to_length_or_percentage();
            cx.style.transform_origin.insert(entity, Translate { x, y });
            cx.needs_redraw();
        });

        self
    }

    // Translate
    modifier!(
        /// Sets the translation offset of the view.
        ///
        /// Translation applies to the rendered view and does not affect layout.
        translate,
        Translate,
        SystemFlags::REDRAW
    );

    // Rotate
    modifier!(
        /// Sets the angle of rotation for the view.
        ///
        /// Rotation applies to the rendered view and does not affect layout.
        rotate,
        Angle,
        SystemFlags::REDRAW
    );

    // Scale
    modifier!(
        /// Sets the scale of the view.
        ///
        /// Scale applies to the rendered view and does not affect layout.
        scale,
        Scale,
        SystemFlags::REDRAW
    );
}

impl<'a, V: View> StyleModifiers for Handle<'a, V> {}

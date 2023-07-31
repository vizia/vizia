use vizia_style::{
    BorderRadius, BoxShadow, ColorStop, Gradient, PointerEvents, Position, Rect, Scale, Translate,
};

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
        self.context().style.ids.insert(entity, id.clone());
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
        applied.set_or_bind(self.context(), entity, move |cx, applied| {
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

    /// Sets the checked state of the view.
    fn checked<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();

        // Setting a checked state should make it checkable
        if let Some(abilities) = self.context().style.abilities.get_mut(entity) {
            abilities.set(Abilities::CHECKABLE, true);
        }

        state.set_or_bind(self.context(), entity, |cx, val| {
            let val = val.into();
            if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(cx.current) {
                pseudo_classes.set(PseudoClassFlags::CHECKED, val);
            }

            cx.needs_restyle();
        });

        self
    }

    /// Sets the focused state of the view.
    ///
    /// Since only one view can have keyboard focus at a time, subsequent calls to this
    /// function on other views will cause those views to gain focus and this view to lose it.
    fn focused<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();

        state.set_or_bind(self.context(), entity, |cx, val| {
            let val = val.into();

            if val {
                cx.focus();
            }

            cx.needs_restyle();
        });

        self
    }

    fn read_only<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        state.set_or_bind(self.context(), entity, |cx, val| {
            let val = val.into();
            if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(cx.current) {
                pseudo_classes.set(PseudoClassFlags::READ_ONLY, val);
            }

            cx.needs_restyle();
        });

        self
    }

    fn read_write<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        state.set_or_bind(self.context(), entity, |cx, val| {
            let val = val.into();
            if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(cx.current) {
                pseudo_classes.set(PseudoClassFlags::READ_WRITE, val);
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
        /// The layout system will still compute the size and position of an invisible (hidden) view.
        visibility,
        Visibility,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the opacity of the view.
        ///
        /// Exects a value between 0.0 (transparent) and 1.0 (opaque).
        opacity,
        Opacity,
        SystemFlags::REDRAW
    );

    /// Sets the z-index of the view.
    ///
    /// Views with a higher z-index will be rendered on top of those with a lower z-order.
    /// Views with the same z-index are rendered in tree order.
    fn z_index<U: Into<i32>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        // value.set_or_bind(self.context(), entity, |cx, v| {
        let value = value.get_val(self.context()).into();
        self.context().tree.set_z_index(entity, value);
        self.context().needs_redraw();
        // });

        self
    }

    /// Sets the clip path for the the view.
    fn clip_path<U: Into<ClipPath>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
            cx.style.clip_path.insert(cx.current, value);

            cx.needs_redraw();
        });

        self
    }

    fn overflow<U: Into<Overflow>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
            cx.style.overflowx.insert(cx.current, value);
            cx.style.overflowy.insert(cx.current, value);

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

    /// Sets the backdrop filter for the view.
    fn backdrop_filter<U: Into<Filter>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
            cx.style.backdrop_filter.insert(cx.current, value);

            cx.needs_redraw();
        });

        self
    }

    /// Add a box-shadow to the view.
    fn box_shadow<U: Into<BoxShadow>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
            if let Some(box_shadows) = cx.style.box_shadow.get_inline_mut(cx.current) {
                box_shadows.push(value);
            } else {
                cx.style.box_shadow.insert(cx.current, vec![value]);
            }

            cx.needs_redraw();
        });

        self
    }

    fn background_gradient<U: Into<Gradient>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
            if let Some(background_images) = cx.style.background_image.get_inline_mut(cx.current) {
                background_images.push(ImageOrGradient::Gradient(value));
            } else {
                cx.style
                    .background_image
                    .insert(cx.current, vec![ImageOrGradient::Gradient(value)]);
            }

            cx.needs_redraw();
        });

        self
    }

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
        value.set_or_bind(self.context(), entity, |cx, val| {
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
            cx.style.background_image.insert(cx.current, images);
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
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
            cx.style.border_top_left_radius.insert(cx.current, value.top_left);
            cx.style.border_top_right_radius.insert(cx.current, value.top_right);
            cx.style.border_bottom_left_radius.insert(cx.current, value.bottom_left);
            cx.style.border_bottom_right_radius.insert(cx.current, value.bottom_right);

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
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
            cx.style.border_top_left_shape.insert(cx.current, value.0);
            cx.style.border_top_right_shape.insert(cx.current, value.1);
            cx.style.border_bottom_right_shape.insert(cx.current, value.2);
            cx.style.border_bottom_left_shape.insert(cx.current, value.3);

            cx.needs_redraw();
        });

        self
    }

    // Outline Properties
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

    /// Sets whether the view can be become the target of pointer events.
    fn pointer_events<U: Into<PointerEvents>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
            cx.style.pointer_events.insert(cx.current, value);
        });

        self
    }

    /// Sets the transform of the view with a list of transform functions.
    fn transform<U: Into<Vec<Transform>>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
            cx.style.transform.insert(cx.current, value);
            cx.needs_redraw();
        });

        self
    }

    /// Sets the transform origin of the the view.
    fn transform_origin<U: Into<Position>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value: Position = v.into();
            let x = value.x.to_length_or_percentage();
            let y = value.y.to_length_or_percentage();
            cx.style.transform_origin.insert(cx.current, Translate { x, y });
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

#[derive(Debug, Clone)]
pub struct LinearGradientBuilder {
    direction: LineDirection,
    stops: Vec<ColorStop<LengthOrPercentage>>,
}

impl Default for LinearGradientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LinearGradientBuilder {
    pub fn new() -> Self {
        LinearGradientBuilder { direction: LineDirection::default(), stops: Vec::new() }
    }

    pub fn with_direction(direction: impl Into<LineDirection>) -> Self {
        LinearGradientBuilder { direction: direction.into(), stops: Vec::new() }
    }

    fn build(self) -> Gradient {
        Gradient::Linear(LinearGradient { direction: self.direction, stops: self.stops })
    }

    pub fn add_stop(mut self, stop: impl Into<ColorStop<LengthOrPercentage>>) -> Self {
        self.stops.push(stop.into());

        self
    }
}

impl From<LinearGradientBuilder> for Gradient {
    fn from(value: LinearGradientBuilder) -> Self {
        value.build()
    }
}

#[derive(Debug, Clone)]
pub struct BoxShadowBuilder {
    box_shadow: BoxShadow,
}

impl Default for BoxShadowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BoxShadowBuilder {
    pub fn new() -> Self {
        Self { box_shadow: BoxShadow::default() }
    }

    fn build(self) -> BoxShadow {
        self.box_shadow
    }

    pub fn x_offset(mut self, offset: impl Into<Length>) -> Self {
        self.box_shadow.x_offset = offset.into();

        self
    }

    pub fn y_offset(mut self, offset: impl Into<Length>) -> Self {
        self.box_shadow.y_offset = offset.into();

        self
    }

    pub fn blur(mut self, radius: Length) -> Self {
        self.box_shadow.blur_radius = Some(radius);

        self
    }

    pub fn spread(mut self, radius: Length) -> Self {
        self.box_shadow.spread_radius = Some(radius);

        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.box_shadow.color = Some(color);

        self
    }

    pub fn inset(mut self) -> Self {
        self.box_shadow.inset = true;

        self
    }
}

impl From<BoxShadowBuilder> for BoxShadow {
    fn from(value: BoxShadowBuilder) -> Self {
        value.build()
    }
}

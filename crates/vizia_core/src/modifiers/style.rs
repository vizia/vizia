use vizia_style::{ColorStop, CornerRadius, Rect};

use super::internal;
use crate::prelude::*;

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
        self.context().needs_restyle(entity);

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

        self.context().needs_restyle(entity);

        self
    }

    /// Sets whether a view should have the given class name.
    fn toggle_class(mut self, name: &str, applied: impl Res<bool>) -> Self {
        let name = name.to_owned();
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            applied.set_or_bind(cx, entity, move |cx, applied| {
                let applied = applied.get(cx);
                if let Some(class_list) = cx.style.classes.get_mut(entity) {
                    if applied {
                        class_list.insert(name.clone());
                    } else {
                        class_list.remove(&name);
                    }
                }

                cx.needs_restyle(entity);
            });
        });

        self
    }

    // PseudoClassFlags
    // TODO: Should these have their own modifiers trait?

    /// Sets the checked state of the view.
    fn checked<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        // Setting a checked state should make it checkable
        if let Some(abilities) = self.context().style.abilities.get_mut(entity) {
            abilities.set(Abilities::CHECKABLE, true);
        }

        self.context().with_current(current, move |cx| {
            state.set_or_bind(cx, entity, move |cx, val| {
                let val = val.get(cx).into();
                if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(entity) {
                    pseudo_classes.set(PseudoClassFlags::CHECKED, val);
                }
                cx.needs_restyle(entity);
            });
        });

        self
    }

    /// Sets the focused state of the view.
    ///
    /// Since only one view can have keyboard focus at a time, subsequent calls to this
    /// function on other views will cause those views to gain focus and this view to lose it.
    fn focused<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();

        self.context().with_current(current, |cx| {
            state.set_or_bind(cx, entity, |cx, val| {
                let val = val.get(cx).into();

                if val {
                    cx.focus();
                    // cx.focus_with_visibility(true);
                }

                cx.needs_restyle(cx.current);
            });
        });

        self
    }

    /// Sets the focused state of the view as well as the focus visibility.
    fn focused_with_visibility<U: Into<bool>>(
        mut self,
        focus: impl Res<U> + Copy + 'static,
        visibility: impl Res<U> + Copy + 'static,
    ) -> Self {
        let entity = self.entity();
        let current = self.current();

        self.context().with_current(current, move |cx| {
            focus.set_or_bind(cx, entity, move |cx, f| {
                visibility.set_or_bind(cx, entity, move |cx, v| {
                    let focus = f.get(cx).into();
                    let visibility = v.get(cx).into();
                    if focus {
                        //cx.focus();
                        cx.focus_with_visibility(visibility);
                        cx.needs_restyle(cx.current);
                    }
                });
            });
        });

        self
    }

    /// Sets whether the view should be in a read-only state.
    fn read_only<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            state.set_or_bind(cx, entity, move |cx, val| {
                let val = val.get(cx).into();
                if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(cx.current) {
                    pseudo_classes.set(PseudoClassFlags::READ_ONLY, val);
                }

                cx.needs_restyle(cx.current);
            });
        });

        self
    }

    /// Sets whether the view should be in a read-write state.
    fn read_write<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            state.set_or_bind(cx, entity, move |cx, val| {
                let val = val.get(cx).into();
                if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(cx.current) {
                    pseudo_classes.set(PseudoClassFlags::READ_WRITE, val);
                }

                cx.needs_restyle(cx.current);
            });
        });

        self
    }

    /// Sets whether the view is showing a placeholder.
    fn placeholder_shown<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            state.set_or_bind(cx, entity, move |cx, val| {
                let val = val.get(cx).into();
                if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(cx.current) {
                    pseudo_classes.set(PseudoClassFlags::PLACEHOLDER_SHOWN, val);
                }

                cx.needs_restyle(cx.current);
            });
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
        let cx = self.context();
        let value = value.get(cx).into();
        cx.style.z_index.insert(entity, value);
        cx.needs_redraw(entity);
        // });

        self
    }

    /// Sets the clip path for the the view.
    fn clip_path<U: Into<ClipPath>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                cx.style.clip_path.insert(cx.current, value);

                cx.needs_redraw(entity);
            });
        });

        self
    }

    /// Sets the overflow behavior of the view in the horizontal and vertical directions simultaneously.
    fn overflow<U: Into<Overflow>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                cx.style.overflowx.insert(cx.current, value);
                cx.style.overflowy.insert(cx.current, value);

                cx.needs_redraw(entity);
            });
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
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                cx.style.backdrop_filter.insert(cx.current, value);

                cx.needs_redraw(entity);
            });
        });

        self
    }

    /// Add a shadow to the view.
    fn shadow<U: Into<Shadow>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                if let Some(shadows) = cx.style.shadow.get_inline_mut(cx.current) {
                    shadows.push(value);
                } else {
                    cx.style.shadow.insert(cx.current, vec![value]);
                }

                cx.needs_redraw(entity);
            });
        });

        self
    }

    /// Set the shadows of the view.
    fn shadows<U: Into<Vec<Shadow>>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();

                cx.style.shadow.insert(cx.current, value);

                cx.needs_redraw(entity);
            });
        });

        self
    }

    /// Set the background gradient of the view.
    fn background_gradient<U: Into<Gradient>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                if let Some(background_images) =
                    cx.style.background_image.get_inline_mut(cx.current)
                {
                    background_images.push(ImageOrGradient::Gradient(value));
                } else {
                    cx.style
                        .background_image
                        .insert(cx.current, vec![ImageOrGradient::Gradient(value)]);
                }

                cx.needs_redraw(entity);
            });
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

    /// Set the background image of the view.
    fn background_image<'i, U: Into<BackgroundImage<'i>>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, val| {
                let image = val.get(cx).into();
                let image = match image {
                    BackgroundImage::Gradient(gradient) => {
                        Some(ImageOrGradient::Gradient(*gradient))
                    }
                    BackgroundImage::Url(url) => Some(ImageOrGradient::Image(url.url.to_string())),
                    _ => None,
                };

                if let Some(image) = image {
                    cx.style.background_image.insert(cx.current, vec![image]);
                }

                cx.needs_redraw(entity);
            });
        });

        self
    }

    // Border Properties

    /// Sets the border width of the view.
    fn border_width<U: Into<LengthOrPercentage>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        value.set_or_bind(self.context(), current, move |cx, v| {
            cx.style.border_width.insert(entity, v.get(cx).into());
            cx.cache.path.remove(entity);
            cx.style.system_flags |= SystemFlags::RELAYOUT | SystemFlags::REDRAW;
            cx.set_system_flags(entity, SystemFlags::RELAYOUT | SystemFlags::REDRAW);
        });

        self
    }

    modifier!(
        /// Sets the border color of the view.
        border_color,
        Color,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border color of the view.
        border_style,
        BorderStyleKeyword,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the corner radius for the top-left corner of the view.
        corner_top_left_radius,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the corner radius for the top-right corner of the view.
        corner_top_right_radius,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the corner radius for the bottom-left corner of the view.
        corner_bottom_left_radius,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the corner radius for the bottom-right corner of the view.
        corner_bottom_right_radius,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    /// Sets the corner radius for all four corners of the view.
    fn corner_radius<U: std::fmt::Debug + Into<CornerRadius>>(
        mut self,
        value: impl Res<U>,
    ) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                cx.style.corner_top_left_radius.insert(cx.current, value.top_left);
                cx.style.corner_top_right_radius.insert(cx.current, value.top_right);
                cx.style.corner_bottom_left_radius.insert(cx.current, value.bottom_left);
                cx.style.corner_bottom_right_radius.insert(cx.current, value.bottom_right);

                cx.needs_redraw(entity);
            });
        });

        self
    }

    modifier!(
        /// Sets the corner corner shape for the top-left corner of the view.
        corner_top_left_shape,
        CornerShape,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the corner corner shape for the top-right corner of the view.
        corner_top_right_shape,
        CornerShape,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the corner corner shape for the bottom-left corner of the view.
        corner_bottom_left_shape,
        CornerShape,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the corner corner shape for the bottom-right corner of the view.
        corner_bottom_right_shape,
        CornerShape,
        SystemFlags::REDRAW
    );

    /// Sets the corner shape for all four corners of the view.
    fn corner_shape<U: std::fmt::Debug + Into<Rect<CornerShape>>>(
        mut self,
        value: impl Res<U>,
    ) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                cx.style.corner_top_left_shape.insert(cx.current, value.0);
                cx.style.corner_top_right_shape.insert(cx.current, value.1);
                cx.style.corner_bottom_right_shape.insert(cx.current, value.2);
                cx.style.corner_bottom_left_shape.insert(cx.current, value.3);

                cx.needs_redraw(entity);
            });
        });

        self
    }

    modifier!(
        /// Sets the corner smoothing for the top-left corner of the view.
        corner_top_left_smoothing,
        f32,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the corner smoothing for the top-right corner of the view.
        corner_top_right_smoothing,
        f32,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the corner smoothing for the bottom-left corner of the view.
        corner_bottom_left_smoothing,
        f32,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the corner smoothing for the bottom-right corner of the view.
        corner_bottom_right_smoothing,
        f32,
        SystemFlags::REDRAW
    );

    /// Sets the corner smoothing for all four corners of the view.
    fn corner_smoothing<U: std::fmt::Debug + Into<Rect<f32>>>(
        mut self,
        value: impl Res<U>,
    ) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                cx.style.corner_top_left_smoothing.insert(cx.current, value.0);
                cx.style.corner_top_right_smoothing.insert(cx.current, value.1);
                cx.style.corner_bottom_left_smoothing.insert(cx.current, value.2);
                cx.style.corner_bottom_right_smoothing.insert(cx.current, value.3);

                cx.needs_redraw(entity);
            });
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
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                cx.style.pointer_events.insert(cx.current, value);
            });
        });

        self
    }

    /// Sets the transform of the view with a list of transform functions.
    fn transform<U: Into<Vec<Transform>>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                cx.style.transform.insert(cx.current, value);
                cx.needs_redraw(entity);
            });
        });

        self
    }

    /// Sets the transform origin of the the view.
    fn transform_origin<U: Into<Position>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                let x = value.x.to_length_or_percentage();
                let y = value.y.to_length_or_percentage();
                cx.style.transform_origin.insert(cx.current, Translate { x, y });
                cx.needs_redraw(entity);
            });
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

impl<V: View> StyleModifiers for Handle<'_, V> {}

/// A builder for constructing linear gradients.
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
    /// Creates a new [LinearGradientBuilder].
    pub fn new() -> Self {
        LinearGradientBuilder { direction: LineDirection::default(), stops: Vec::new() }
    }

    /// Set the direction of the linear gradient.
    pub fn with_direction(direction: impl Into<LineDirection>) -> Self {
        LinearGradientBuilder { direction: direction.into(), stops: Vec::new() }
    }

    fn build(self) -> Gradient {
        Gradient::Linear(LinearGradient { direction: self.direction, stops: self.stops })
    }

    /// Add a color stop to the linear gradient.
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

/// A builder for constructing a shadow.
#[derive(Debug, Clone)]
pub struct ShadowBuilder {
    shadow: Shadow,
}

impl Default for ShadowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ShadowBuilder {
    /// Creates a new [ShadowBuilder].
    pub fn new() -> Self {
        Self { shadow: Shadow::default() }
    }

    fn build(self) -> Shadow {
        self.shadow
    }

    /// Sets the horizontal offset of the shadow.
    pub fn x_offset(mut self, offset: impl Into<Length>) -> Self {
        self.shadow.x_offset = offset.into();

        self
    }

    /// Set the vertical offset of the shadow.
    pub fn y_offset(mut self, offset: impl Into<Length>) -> Self {
        self.shadow.y_offset = offset.into();

        self
    }

    /// Sets the blur radius of the shadow.
    pub fn blur(mut self, radius: Length) -> Self {
        self.shadow.blur_radius = Some(radius);

        self
    }

    /// Sets the spread amount of the shadow.
    pub fn spread(mut self, radius: Length) -> Self {
        self.shadow.spread_radius = Some(radius);

        self
    }

    /// Sets the color of the shadow.
    pub fn color(mut self, color: Color) -> Self {
        self.shadow.color = Some(color);

        self
    }

    /// Sets whether the shadow should be inset.
    pub fn inset(mut self) -> Self {
        self.shadow.inset = true;

        self
    }
}

impl From<ShadowBuilder> for Shadow {
    fn from(value: ShadowBuilder) -> Self {
        value.build()
    }
}

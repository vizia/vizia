use crate::prelude::*;

use vizia_style::{BorderWidth, Property};

use super::TimingFunction;

/// A builder for constructing animations.
#[derive(Clone, Copy)]
pub struct Animation {
    pub(crate) duration: Duration,
    pub(crate) delay: Duration,
    pub(crate) fill_mode: AnimationFillMode,
    pub(crate) iteration_count: AnimationIterationCount,
    pub(crate) direction: AnimationDirection,
    pub(crate) easing_function: TimingFunction,
}

impl Default for Animation {
    fn default() -> Self {
        Self::new()
    }
}

impl Animation {
    /// Creates a new [AnimationBuilder].
    pub fn new() -> Self {
        Self {
            duration: Duration::new(0, 0),
            delay: Duration::new(0, 0),
            fill_mode: AnimationFillMode::None,
            iteration_count: AnimationIterationCount::Count(1),
            direction: AnimationDirection::Normal,
            easing_function: TimingFunction::linear(),
        }
    }

    /// Sets the duration of the animation.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;

        self
    }

    /// Sets the delay of the animation.
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;

        self
    }

    /// Sets the fill mode of the animation.
    pub fn fill_mode(mut self, fill_mode: AnimationFillMode) -> Self {
        self.fill_mode = fill_mode;

        self
    }

    /// Sets the iteration count of the animation.
    pub fn iteration_count(mut self, iteration_count: AnimationIterationCount) -> Self {
        self.iteration_count = iteration_count;

        self
    }

    /// Sets the direction of the animation.
    pub fn direction(mut self, direction: AnimationDirection) -> Self {
        self.direction = direction;

        self
    }

    /// Sets the easing function of the animation.
    pub fn easing_function(mut self, easing_function: TimingFunction) -> Self {
        self.easing_function = easing_function;

        self
    }
}

#[derive(Clone)]
pub struct Keyframes<'a> {
    pub(crate) keyframes: Vec<KeyframeBuilder<'a>>,
}

impl Default for Keyframes<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Keyframes<'_> {
    /// Creates a new [KeyframesBuilder].
    pub fn new() -> Self {
        Self { keyframes: Vec::new() }
    }

    /// Adds a new keyframe to the keyframes.
    pub fn keyframe(
        mut self,
        time: f32,
        keyframe: impl FnOnce(KeyframeBuilder) -> KeyframeBuilder,
    ) -> Self {
        let keyframe = (keyframe)(KeyframeBuilder::new(time));
        self.keyframes.push(keyframe);

        self
    }
}

/// A builder for constructing keyframes.
#[derive(Clone)]
pub struct KeyframeBuilder<'a> {
    pub(crate) time: f32,
    pub(crate) properties: Vec<Property<'a>>,
}

impl<'a> KeyframeBuilder<'a> {
    /// Creates a new [KeyframeBuilder].
    pub(crate) fn new(time: f32) -> Self {
        Self { time, properties: Vec::new() }
    }

    // DISPLAY

    /// Set the display value for the keyframe.
    pub fn display(mut self, val: impl Into<Display>) -> Self {
        self.properties.push(Property::Display(val.into()));

        self
    }

    /// Set the opacity value for the keyframe.
    pub fn opacity(mut self, val: impl Into<Opacity>) -> Self {
        self.properties.push(Property::Opacity(val.into()));

        self
    }

    /// Set the clip-path value for the keyframe.
    pub fn clip_path(mut self, val: impl Into<ClipPath>) -> Self {
        self.properties.push(Property::ClipPath(val.into()));

        self
    }

    // TRANSFORM

    /// Set the transform value for the keyframe.
    pub fn transform(mut self, val: impl Into<Vec<Transform>>) -> Self {
        self.properties.push(Property::Transform(val.into()));

        self
    }

    /// Set the transform origin value for the keyframe.
    pub fn transform_origin(mut self, val: impl Into<Position>) -> Self {
        self.properties.push(Property::TransformOrigin(val.into()));

        self
    }

    /// Set the translate value for the keyframe.
    pub fn translate(mut self, val: impl Into<Translate>) -> Self {
        self.properties.push(Property::Translate(val.into()));

        self
    }

    /// Set the rotate value for the keyframe.
    pub fn rotate(mut self, val: impl Into<Angle>) -> Self {
        self.properties.push(Property::Rotate(val.into()));

        self
    }

    /// Set the scale value for the keyframe.
    pub fn scale(mut self, val: impl Into<Scale>) -> Self {
        self.properties.push(Property::Scale(val.into()));

        self
    }

    // BORDER

    /// Set the border width value for the keyframe.
    pub fn border_width(mut self, val: impl Into<BorderWidth>) -> Self {
        self.properties.push(Property::BorderWidth(val.into()));

        self
    }

    /// Set the border color value for the keyframe.
    pub fn border_color(mut self, val: impl Into<Color>) -> Self {
        self.properties.push(Property::BorderColor(val.into()));

        self
    }

    // CORNERS

    pub fn corner_top_left_radius(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(Property::CornerTopLeftRadius(val.into()));

        self
    }

    pub fn corner_top_right_radius(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(Property::CornerTopRightRadius(val.into()));

        self
    }

    pub fn corner_bottom_left_radius(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(Property::CornerBottomLeftRadius(val.into()));

        self
    }

    pub fn corner_bottom_right_radius(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(Property::CornerBottomRightRadius(val.into()));

        self
    }

    // OUTLINE

    pub fn outline_width(mut self, val: impl Into<BorderWidth>) -> Self {
        self.properties.push(Property::OutlineWidth(val.into()));

        self
    }

    pub fn outline_color(mut self, val: impl Into<Color>) -> Self {
        self.properties.push(Property::OutlineColor(val.into()));

        self
    }

    pub fn outline_offset(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(Property::OutlineOffset(val.into()));

        self
    }

    // BACKGROUND

    pub fn background_color(mut self, val: impl Into<Color>) -> Self {
        self.properties.push(Property::BackgroundColor(val.into()));

        self
    }

    pub fn background_image(mut self, val: impl Into<Vec<BackgroundImage<'a>>>) -> Self {
        self.properties.push(Property::BackgroundImage(val.into()));

        self
    }

    pub fn background_size(mut self, val: impl Into<Vec<BackgroundSize>>) -> Self {
        self.properties.push(Property::BackgroundSize(val.into()));

        self
    }

    // BOX SHADOW

    pub fn shadow(mut self, val: impl Into<Vec<Shadow>>) -> Self {
        self.properties.push(Property::Shadow(val.into()));

        self
    }

    // TEXT

    pub fn color(mut self, val: impl Into<Color>) -> Self {
        self.properties.push(Property::FontColor(val.into()));

        self
    }

    pub fn font_size(mut self, val: impl Into<FontSize>) -> Self {
        self.properties.push(Property::FontSize(val.into()));

        self
    }

    pub fn caret_color(mut self, val: impl Into<Color>) -> Self {
        self.properties.push(Property::CaretColor(val.into()));

        self
    }

    pub fn selection_color(mut self, val: impl Into<Color>) -> Self {
        self.properties.push(Property::SelectionColor(val.into()));

        self
    }

    // SPACE

    pub fn left(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::Left(val.into()));

        self
    }

    pub fn right(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::Right(val.into()));

        self
    }

    pub fn top(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::Top(val.into()));

        self
    }

    pub fn bottom(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::Bottom(val.into()));

        self
    }

    // PADDING

    pub fn padding_left(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::PaddingLeft(val.into()));

        self
    }

    pub fn padding_right(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::PaddingRight(val.into()));

        self
    }

    pub fn padding_top(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::PaddingTop(val.into()));

        self
    }

    pub fn padding_bottom(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::PaddingBottom(val.into()));

        self
    }

    pub fn horizontal_gap(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::HorizontalGap(val.into()));

        self
    }

    pub fn vertical_gap(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::VerticalGap(val.into()));

        self
    }

    // SIZE

    pub fn width(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::Width(val.into()));

        self
    }

    pub fn height(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::Height(val.into()));

        self
    }

    // SIZE CONSTRAINTS
    pub fn min_width(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::MinWidth(val.into()));

        self
    }

    pub fn max_width(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::MaxWidth(val.into()));

        self
    }

    pub fn min_height(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::MinHeight(val.into()));

        self
    }

    pub fn max_height(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::MaxHeight(val.into()));

        self
    }
}

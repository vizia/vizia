use crate::prelude::*;
// use instant::Duration;
use morphorm::Units;
use vizia_style::{
    BackgroundSize, BorderWidth, BoxShadow, FontSize, Position, Property, Scale, Translate,
};

// pub struct AnimationDescription {
//     duration: Duration,
//     delay: Duration,
// }

pub struct AnimationBuilder<'a> {
    pub(crate) keyframes: Vec<KeyframeBuilder<'a>>,
}

impl<'a> Default for AnimationBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> AnimationBuilder<'a> {
    pub fn new() -> Self {
        Self { keyframes: Vec::new() }
    }

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

pub struct KeyframeBuilder<'a> {
    pub(crate) time: f32,
    pub(crate) properties: Vec<Property<'a>>,
}

// TODO: Make a macro for these
impl<'a> KeyframeBuilder<'a> {
    pub(crate) fn new(time: f32) -> Self {
        Self { time, properties: Vec::new() }
    }

    // DISPLAY

    pub fn display(mut self, val: impl Into<Display>) -> Self {
        self.properties.push(Property::Display(val.into()));

        self
    }

    pub fn opacity(mut self, val: impl Into<Opacity>) -> Self {
        self.properties.push(Property::Opacity(val.into()));

        self
    }

    pub fn clip_path(mut self, val: impl Into<ClipPath>) -> Self {
        self.properties.push(Property::ClipPath(val.into()));

        self
    }

    // TRANSFORM

    pub fn transform(mut self, val: impl Into<Vec<Transform>>) -> Self {
        self.properties.push(Property::Transform(val.into()));

        self
    }

    pub fn transform_origin(mut self, val: impl Into<Position>) -> Self {
        self.properties.push(Property::TransformOrigin(val.into()));

        self
    }

    pub fn translate(mut self, val: impl Into<Translate>) -> Self {
        self.properties.push(Property::Translate(val.into()));

        self
    }

    pub fn rotate(mut self, val: impl Into<Angle>) -> Self {
        self.properties.push(Property::Rotate(val.into()));

        self
    }

    pub fn scale(mut self, val: impl Into<Scale>) -> Self {
        self.properties.push(Property::Scale(val.into()));

        self
    }

    // BORDER

    pub fn border_width(mut self, val: impl Into<BorderWidth>) -> Self {
        self.properties.push(Property::BorderWidth(val.into()));

        self
    }

    pub fn border_color(mut self, val: impl Into<Color>) -> Self {
        self.properties.push(Property::BorderColor(val.into()));

        self
    }

    pub fn border_top_left_radius(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(Property::BorderTopLeftRadius(val.into()));

        self
    }

    pub fn border_top_right_radius(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(Property::BorderTopRightRadius(val.into()));

        self
    }

    pub fn border_bottom_left_radius(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(Property::BorderBottomLeftRadius(val.into()));

        self
    }

    pub fn border_bottom_right_radius(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(Property::BorderBottomRightRadius(val.into()));

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

    pub fn box_shadow(mut self, val: impl Into<Vec<BoxShadow>>) -> Self {
        self.properties.push(Property::BoxShadow(val.into()));

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

    // CHILD SPACE

    pub fn child_left(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::ChildLeft(val.into()));

        self
    }

    pub fn child_right(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::ChildRight(val.into()));

        self
    }

    pub fn child_top(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::ChildTop(val.into()));

        self
    }

    pub fn child_bottom(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::ChildBottom(val.into()));

        self
    }

    pub fn col_between(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::ColBetween(val.into()));

        self
    }

    pub fn row_between(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::RowBetween(val.into()));

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

    // SPACE CONSTRAINTS
    pub fn min_left(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::MinLeft(val.into()));

        self
    }

    pub fn max_left(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::MaxLeft(val.into()));

        self
    }

    pub fn min_right(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::MinRight(val.into()));

        self
    }

    pub fn max_right(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::MaxRight(val.into()));

        self
    }

    pub fn min_top(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::MinTop(val.into()));

        self
    }

    pub fn max_top(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::MaxTop(val.into()));

        self
    }

    pub fn min_bottom(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::MinBottom(val.into()));

        self
    }

    pub fn max_bottom(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(Property::MaxBottom(val.into()));

        self
    }
}

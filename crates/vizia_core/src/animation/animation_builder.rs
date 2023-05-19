use crate::{prelude::*, style::ImageOrGradient};
use morphorm::Units;
use vizia_style::{BackgroundSize, BoxShadow, FontSize, Scale, Translate};

pub(crate) enum AnimationProperty {
    // DISPLAY
    Display(Display),
    Opacity(Opacity),
    ClipPath(ClipPath),

    // TRANSFORM
    Transform(Vec<Transform>),
    TransformOrigin(Translate),
    Translate(Translate),
    Rotate(Angle),
    Scale(Scale),

    // BORDER
    BorderWidth(LengthOrPercentage),
    BorderColor(Color),

    BorderTopLeftRadius(LengthOrPercentage),
    BorderTopRightRadius(LengthOrPercentage),
    BorderBottomLeftRadius(LengthOrPercentage),
    BorderBottomRightRadius(LengthOrPercentage),

    // OUTLINE
    OutlineWidth(LengthOrPercentage),
    OutlineColor(Color),
    OutlineOffset(LengthOrPercentage),

    // BACKGROUND
    BackgroundColor(Color),
    BackgroundImage(Vec<ImageOrGradient>),
    BackgroundSize(Vec<BackgroundSize>),

    // BOX SHADOW
    BoxShadow(Vec<BoxShadow>),

    // TEXT
    FontColor(Color),
    FontSize(FontSize),
    CaretColor(Color),
    SelectionColor(Color),

    // SPACE
    Left(Units),
    Right(Units),
    Top(Units),
    Bottom(Units),

    // CHILD SPACE
    ChildLeft(Units),
    ChildRight(Units),
    ChildTop(Units),
    ChildBottom(Units),
    ColBetween(Units),
    RowBetween(Units),

    // SIZE
    Width(Units),
    Height(Units),

    // SIZE CONSTRAINTS
    MinWidth(Units),
    MaxWidth(Units),
    MinHeight(Units),
    MaxHeight(Units),

    // SPACE CONSTRAINTS
    MinLeft(Units),
    MaxLeft(Units),
    MinRight(Units),
    MaxRight(Units),
    MinTop(Units),
    MaxTop(Units),
    MinBottom(Units),
    MaxBottom(Units),
}

pub struct AnimationBuilder {
    pub(crate) keyframes: Vec<KeyframeBuilder>,
}

impl<'a> AnimationBuilder {
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

pub struct KeyframeBuilder {
    pub(crate) time: f32,
    pub(crate) properties: Vec<AnimationProperty>,
}

// TODO: Make a macro for these
impl KeyframeBuilder {
    pub(crate) fn new(time: f32) -> Self {
        Self { time, properties: Vec::new() }
    }

    // DISPLAY

    pub fn display(mut self, val: impl Into<Display>) -> Self {
        self.properties.push(AnimationProperty::Display(val.into()));

        self
    }

    pub fn opacity(mut self, val: impl Into<Opacity>) -> Self {
        self.properties.push(AnimationProperty::Opacity(val.into()));

        self
    }

    pub fn clip_path(mut self, val: impl Into<ClipPath>) -> Self {
        self.properties.push(AnimationProperty::ClipPath(val.into()));

        self
    }

    // TRANSFORM

    pub fn transform(mut self, val: impl Into<Vec<Transform>>) -> Self {
        self.properties.push(AnimationProperty::Transform(val.into()));

        self
    }

    pub fn transform_origin(mut self, val: impl Into<Translate>) -> Self {
        self.properties.push(AnimationProperty::TransformOrigin(val.into()));

        self
    }

    pub fn translate(mut self, val: impl Into<Translate>) -> Self {
        self.properties.push(AnimationProperty::Translate(val.into()));

        self
    }

    pub fn rotate(mut self, val: impl Into<Angle>) -> Self {
        self.properties.push(AnimationProperty::Rotate(val.into()));

        self
    }

    pub fn scale(mut self, val: impl Into<Scale>) -> Self {
        self.properties.push(AnimationProperty::Scale(val.into()));

        self
    }

    // BORDER

    pub fn border_width(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(AnimationProperty::BorderWidth(val.into()));

        self
    }

    pub fn border_color(mut self, val: impl Into<Color>) -> Self {
        self.properties.push(AnimationProperty::BorderColor(val.into()));

        self
    }

    pub fn border_top_left_radius(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(AnimationProperty::BorderTopLeftRadius(val.into()));

        self
    }

    pub fn border_top_right_radius(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(AnimationProperty::BorderTopRightRadius(val.into()));

        self
    }

    pub fn border_bottom_left_radius(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(AnimationProperty::BorderBottomLeftRadius(val.into()));

        self
    }

    pub fn border_bottom_right_radius(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(AnimationProperty::BorderBottomRightRadius(val.into()));

        self
    }

    // OUTLINE

    pub fn outline_width(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(AnimationProperty::OutlineWidth(val.into()));

        self
    }

    pub fn outline_color(mut self, val: impl Into<Color>) -> Self {
        self.properties.push(AnimationProperty::OutlineColor(val.into()));

        self
    }

    pub fn outline_offset(mut self, val: impl Into<LengthOrPercentage>) -> Self {
        self.properties.push(AnimationProperty::OutlineOffset(val.into()));

        self
    }

    // BACKGROUND

    pub fn background_color(mut self, val: impl Into<Color>) -> Self {
        self.properties.push(AnimationProperty::BackgroundColor(val.into()));

        self
    }

    pub fn background_image(mut self, val: impl Into<Vec<ImageOrGradient>>) -> Self {
        self.properties.push(AnimationProperty::BackgroundImage(val.into()));

        self
    }

    pub fn background_size(mut self, val: impl Into<Vec<BackgroundSize>>) -> Self {
        self.properties.push(AnimationProperty::BackgroundSize(val.into()));

        self
    }

    // BOX SHADOW

    pub fn box_shadow(mut self, val: impl Into<Vec<BoxShadow>>) -> Self {
        self.properties.push(AnimationProperty::BoxShadow(val.into()));

        self
    }

    // TEXT

    pub fn color(mut self, val: impl Into<Color>) -> Self {
        self.properties.push(AnimationProperty::FontColor(val.into()));

        self
    }

    pub fn font_size(mut self, val: impl Into<FontSize>) -> Self {
        self.properties.push(AnimationProperty::FontSize(val.into()));

        self
    }

    pub fn caret_color(mut self, val: impl Into<Color>) -> Self {
        self.properties.push(AnimationProperty::CaretColor(val.into()));

        self
    }

    pub fn selection_color(mut self, val: impl Into<Color>) -> Self {
        self.properties.push(AnimationProperty::SelectionColor(val.into()));

        self
    }

    // SPACE

    pub fn left(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::Left(val.into()));

        self
    }

    pub fn right(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::Right(val.into()));

        self
    }

    pub fn top(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::Top(val.into()));

        self
    }

    pub fn bottom(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::Bottom(val.into()));

        self
    }

    // CHILD SPACE

    pub fn child_left(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::ChildLeft(val.into()));

        self
    }

    pub fn child_right(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::ChildRight(val.into()));

        self
    }

    pub fn child_top(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::ChildTop(val.into()));

        self
    }

    pub fn child_bottom(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::ChildBottom(val.into()));

        self
    }

    pub fn col_between(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::ColBetween(val.into()));

        self
    }

    pub fn row_between(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::RowBetween(val.into()));

        self
    }

    // SIZE

    pub fn width(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::Width(val.into()));

        self
    }

    pub fn height(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::Height(val.into()));

        self
    }

    // SIZE CONSTRAINTS
    pub fn min_width(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::MinWidth(val.into()));

        self
    }

    pub fn max_width(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::MaxWidth(val.into()));

        self
    }

    pub fn min_height(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::MinHeight(val.into()));

        self
    }

    pub fn max_height(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::MaxHeight(val.into()));

        self
    }

    // SPACE CONSTRAINTS
    pub fn min_left(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::MinLeft(val.into()));

        self
    }

    pub fn max_left(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::MaxLeft(val.into()));

        self
    }

    pub fn min_right(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::MinRight(val.into()));

        self
    }

    pub fn max_right(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::MaxRight(val.into()));

        self
    }

    pub fn min_top(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::MinTop(val.into()));

        self
    }

    pub fn max_top(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::MaxTop(val.into()));

        self
    }

    pub fn min_bottom(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::MinBottom(val.into()));

        self
    }

    pub fn max_bottom(mut self, val: impl Into<Units>) -> Self {
        self.properties.push(AnimationProperty::MaxBottom(val.into()));

        self
    }
}

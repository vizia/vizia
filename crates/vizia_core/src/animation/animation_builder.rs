use crate::prelude::*;
use morphorm::Units;
use vizia_style::{Scale, Translate};

pub(crate) enum AnimationProperty {
    Left(Units),
    Right(Units),
    Top(Units),
    Bottom(Units),

    Translate(Translate),
    Rotate(Angle),
    Scale(Scale),
    Opacity(Opacity),
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

impl KeyframeBuilder {
    pub(crate) fn new(time: f32) -> Self {
        Self { time, properties: Vec::new() }
    }

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

    pub fn opacity(mut self, val: impl Into<Opacity>) -> Self {
        self.properties.push(AnimationProperty::Opacity(val.into()));

        self
    }
}

#[cfg(test)]
mod tests {
    use instant::Duration;
    use morphorm::Units;

    use super::AnimationBuilder;

    #[test]
    fn anim_builder() {
        let anim_builder = AnimationBuilder::new()
            .keyframe(0.0, |key| key.top(Units::Pixels(50.0)))
            .keyframe(1.0, |key| key.top(Units::Pixels(100.0)));
    }
}

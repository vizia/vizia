use morphorm::Units;

use crate::{style::Opacity, Animation, AnimationState, Color, Context};

pub(crate) struct AnimationDescription {
    duration: instant::Duration,
    delay: instant::Duration,
    persistent: bool,
}

/// A builder for constructing animations.
///
/// Returned from `cx.create_animation(duration)`.
///
/// # Example
/// ```ignore
/// let animation_id = cx.create_animation(std::time::Duration::from_secs(1))
///     .add_keyframe(0.0, |keyframe|
///         keyframe
///             .set_background_color(Color::red())
///             .set_border_color(Color::blue())
///     )
///     .add_keyframe(1.0, |keyframe|
///         keyframe
///             .set_background_color(Color::blue()))
///             .set_border_color(Color::red())
///     .build();
/// ```
pub struct AnimationBuilder<'a> {
    id: Animation,
    cx: &'a mut Context,
    animation_description: AnimationDescription,
}

impl<'a> AnimationBuilder<'a> {
    pub fn new(id: Animation, cx: &'a mut Context, duration: std::time::Duration) -> Self {
        Self {
            id,
            cx,
            animation_description: AnimationDescription {
                duration,
                delay: instant::Duration::from_secs(0),
                persistent: false,
            },
        }
    }

    /// Sets the delay before the animation will play.
    ///
    /// Needs to be called before setting keyframes.
    pub fn with_delay(mut self, delay: instant::Duration) -> Self {
        self.animation_description.delay = delay;

        self
    }

    /// Sets the animation to persist after completion.
    ///
    /// Normally, after an animation is finished, the animated property will return to the the previous value
    /// before the animation was played. Setting an animation to persistent causes the property to be set to the last
    /// value of the animation.
    pub fn persistent(mut self) -> Self {
        self.animation_description.persistent = true;

        self
    }

    /// Adds a keyframe to the animation.
    ///
    ///
    pub fn add_keyframe<F>(self, time: f32, keyframe: F) -> KeyframeBuilder<'a>
    where
        F: FnOnce(KeyframeBuilder<'a>) -> KeyframeBuilder<'a>,
    {
        (keyframe)(KeyframeBuilder::new(self.id, self.cx, time, self.animation_description))
    }
}

/// A builder for constructing keyframes.
///
/// # Example
/// ```ignore
/// let animation_id = cx.create_animation(std::time::Duration::from_secs(1))
///     .add_keyframe(0.0, |keyframe|
///         keyframe
///             .set_background_color(Color::red())
///             .set_border_color(Color::blue())
///     )
///     .add_keyframe(1.0, |keyframe|
///         keyframe
///             .set_background_color(Color::blue()))
///             .set_border_color(Color::red())
///     .build();
/// ```
pub struct KeyframeBuilder<'a> {
    id: Animation,
    cx: &'a mut Context,
    time: f32,
    animation_description: AnimationDescription,
}

impl<'a> KeyframeBuilder<'a> {
    pub(crate) fn new(
        id: Animation,
        cx: &'a mut Context,
        time: f32,
        animation_description: AnimationDescription,
    ) -> Self {
        Self { id, cx, time, animation_description }
    }

    /// Finish building the animation, returning an [Animation] id.
    pub fn build(self) -> Animation {
        self.id
    }

    /// Add another keyframe to the animation.
    pub fn add_keyframe<F>(self, time: f32, keyframe: F) -> Self
    where
        F: FnOnce(KeyframeBuilder<'a>) -> KeyframeBuilder<'a>,
    {
        (keyframe)(KeyframeBuilder::new(self.id, self.cx, time, self.animation_description))
    }

    /// Adds a background-color property to the keyframe.
    ///
    /// # Example
    /// ```ignore
    /// .add_keyframe(0.0, |keyframe| keyframe.set_background_color(Color::red()))
    /// ```
    pub fn set_background_color(self, color: Color) -> Self {
        if let Some(anim_cx) = self.cx.style.background_color.get_animation_mut(self.id) {
            anim_cx.keyframes.push((self.time, color));
        } else {
            let anim_cx = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, color));

            self.cx.style.background_color.insert_animation(self.id, anim_cx);
        }

        self
    }

    /// Adds a left property to the keyframe.
    ///
    /// # Example
    /// ```ignore
    /// .add_keyframe(0.0, |keyframe| keyframe.set_left(Pixels(50.0)))
    /// ```
    pub fn set_left(self, value: Units) -> Self {
        if let Some(anim_cx) = self.cx.style.left.get_animation_mut(self.id) {
            anim_cx.keyframes.push((self.time, value));
        } else {
            let anim_cx = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));

            self.cx.style.left.insert_animation(self.id, anim_cx);
        }

        self
    }

    /// Adds a right property to the keyframe.
    ///
    /// # Example
    /// ```ignore
    /// .add_keyframe(0.0, |keyframe| keyframe.set_right(Pixels(50.0)))
    /// ```
    pub fn set_right(self, value: Units) -> Self {
        if let Some(anim_cx) = self.cx.style.right.get_animation_mut(self.id) {
            anim_cx.keyframes.push((self.time, value));
        } else {
            let anim_cx = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));

            self.cx.style.right.insert_animation(self.id, anim_cx);
        }

        self
    }

    /// Adds a top property to the keyframe.
    ///
    /// # Example
    /// ```ignore
    /// .add_keyframe(0.0, |keyframe| keyframe.set_top(Pixels(50.0)))
    /// ```
    pub fn set_top(self, value: Units) -> Self {
        if let Some(anim_cx) = self.cx.style.top.get_animation_mut(self.id) {
            anim_cx.keyframes.push((self.time, value));
        } else {
            let anim_cx = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));

            self.cx.style.top.insert_animation(self.id, anim_cx);
        }

        self
    }

    /// Adds a bottom property to the keyframe.
    ///
    /// # Example
    /// ```ignore
    /// .add_keyframe(0.0, |keyframe| keyframe.set_bottom(Pixels(50.0)))
    /// ```
    pub fn set_bottom(self, value: Units) -> Self {
        if let Some(anim_cx) = self.cx.style.bottom.get_animation_mut(self.id) {
            anim_cx.keyframes.push((self.time, value));
        } else {
            let anim_cx = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));

            self.cx.style.bottom.insert_animation(self.id, anim_cx);
        }

        self
    }

    /// Adds a width property to the keyframe.
    ///
    /// # Example
    /// ```ignore
    /// .add_keyframe(0.0, |keyframe| keyframe.set_width(Pixels(50.0)))
    /// ```
    pub fn set_width(self, value: Units) -> Self {
        if let Some(anim_cx) = self.cx.style.width.get_animation_mut(self.id) {
            anim_cx.keyframes.push((self.time, value));
        } else {
            let anim_cx = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));

            self.cx.style.width.insert_animation(self.id, anim_cx);
        }

        self
    }

    /// Adds a height property to the keyframe.
    ///
    /// # Example
    /// ```ignore
    /// .add_keyframe(0.0, |keyframe| keyframe.set_height(Pixels(50.0)))
    /// ```
    pub fn set_height(self, value: Units) -> Self {
        if let Some(anim_cx) = self.cx.style.height.get_animation_mut(self.id) {
            anim_cx.keyframes.push((self.time, value));
        } else {
            let anim_cx = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));

            self.cx.style.height.insert_animation(self.id, anim_cx);
        }

        self
    }

    /// Adds a child-left property to the keyframe.
    ///
    /// # Example
    /// ```ignore
    /// .add_keyframe(0.0, |keyframe| keyframe.set_child_left(Pixels(50.0)))
    /// ```
    pub fn set_child_left(self, value: Units) -> Self {
        if let Some(anim_cx) = self.cx.style.child_left.get_animation_mut(self.id) {
            anim_cx.keyframes.push((self.time, value));
        } else {
            let anim_cx = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));

            self.cx.style.child_left.insert_animation(self.id, anim_cx);
        }

        self
    }

    /// Adds a child-right property to the keyframe.
    ///
    /// # Example
    /// ```ignore
    /// .add_keyframe(0.0, |keyframe| keyframe.set_child_right(Pixels(50.0)))
    /// ```
    pub fn set_child_right(self, value: Units) -> Self {
        if let Some(anim_cx) = self.cx.style.child_right.get_animation_mut(self.id) {
            anim_cx.keyframes.push((self.time, value));
        } else {
            let anim_cx = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));

            self.cx.style.child_right.insert_animation(self.id, anim_cx);
        }

        self
    }

    /// Adds a child-top property to the keyframe.
    ///
    /// # Example
    /// ```ignore
    /// .add_keyframe(0.0, |keyframe| keyframe.set_child_top(Pixels(50.0)))
    /// ```
    pub fn set_child_top(self, value: Units) -> Self {
        if let Some(anim_cx) = self.cx.style.child_top.get_animation_mut(self.id) {
            anim_cx.keyframes.push((self.time, value));
        } else {
            let anim_cx = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));

            self.cx.style.child_top.insert_animation(self.id, anim_cx);
        }

        self
    }

    /// Adds a child-bottom property to the keyframe.
    ///
    /// # Example
    /// ```ignore
    /// .add_keyframe(0.0, |keyframe| keyframe.set_child_bottom(Pixels(50.0)))
    /// ```
    pub fn set_child_bottom(self, value: Units) -> Self {
        if let Some(anim_cx) = self.cx.style.child_bottom.get_animation_mut(self.id) {
            anim_cx.keyframes.push((self.time, value));
        } else {
            let anim_cx = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));

            self.cx.style.child_bottom.insert_animation(self.id, anim_cx);
        }

        self
    }

    /// Adds a rotate transform property to the keyframe.
    ///
    /// # Example
    /// ```ignore
    /// .add_keyframe(0.0, |keyframe| keyframe.set_rotate(Pixels(50.0)))
    /// ```
    pub fn set_rotate(self, value: f32) -> Self {
        if let Some(anim_cx) = self.cx.style.rotate.get_animation_mut(self.id) {
            anim_cx.keyframes.push((self.time, value));
        } else {
            let anim_cx = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));

            self.cx.style.rotate.insert_animation(self.id, anim_cx);
        }

        self
    }

    pub fn set_opacity(self, value: f32) -> Self {
        if let Some(anim_cx) = self.cx.style.opacity.get_animation_mut(self.id) {
            anim_cx.keyframes.push((self.time, Opacity(value)));
        } else {
            let anim_cx = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, Opacity(value)));

            self.cx.style.opacity.insert_animation(self.id, anim_cx);
        }

        self
    }
}

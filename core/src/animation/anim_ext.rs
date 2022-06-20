use crate::prelude::*;

/// Trait which provides methods for entities to manipulate linked animations
///
/// This trait is part of the prelude.
pub trait AnimExt: Copy + Sized {
    fn entity(self) -> Entity;

    /// Play an animation on the entity.
    ///
    /// Internally this generates an active animation and links the entity to it for each animated property.
    ///
    /// # Example
    /// Create an animation which animates the `left` property from 0 to 100 pixels in 5 seconds
    /// and play the animation on an entity:
    /// ```ignore
    /// let animation_id = cx.add_animation(instant::Duration::from_secs(5))
    ///     .add_keyframe(0.0, |keyframe| keyframe.set_left(Pixels(0.0)))
    ///     .add_keyframe(1.0, |keyframe| keyframe.set_left(Pixels(100.0)))
    ///     .build();
    ///
    /// entity.play_animation(cx, animation_id);
    /// ```
    fn play_animation(self, cx: &mut Context, animation: Animation) -> Self {
        // Background
        cx.style_mut().background_color.play_animation(self.entity(), animation);

        // Space
        cx.style_mut().left.play_animation(self.entity(), animation);
        cx.style_mut().right.play_animation(self.entity(), animation);
        cx.style_mut().top.play_animation(self.entity(), animation);
        cx.style_mut().bottom.play_animation(self.entity(), animation);

        // Min/Max Space
        cx.style_mut().min_left.play_animation(self.entity(), animation);
        cx.style_mut().min_right.play_animation(self.entity(), animation);
        cx.style_mut().min_top.play_animation(self.entity(), animation);
        cx.style_mut().min_bottom.play_animation(self.entity(), animation);
        cx.style_mut().max_left.play_animation(self.entity(), animation);
        cx.style_mut().max_right.play_animation(self.entity(), animation);
        cx.style_mut().max_top.play_animation(self.entity(), animation);
        cx.style_mut().max_bottom.play_animation(self.entity(), animation);

        // Child Space
        cx.style_mut().child_left.play_animation(self.entity(), animation);
        cx.style_mut().child_right.play_animation(self.entity(), animation);
        cx.style_mut().child_top.play_animation(self.entity(), animation);
        cx.style_mut().child_bottom.play_animation(self.entity(), animation);

        // Size
        cx.style_mut().width.play_animation(self.entity(), animation);
        cx.style_mut().height.play_animation(self.entity(), animation);

        // Min/Max Size
        cx.style_mut().min_width.play_animation(self.entity(), animation);
        cx.style_mut().min_height.play_animation(self.entity(), animation);
        cx.style_mut().max_width.play_animation(self.entity(), animation);
        cx.style_mut().max_height.play_animation(self.entity(), animation);

        // Border
        cx.style_mut().border_color.play_animation(self.entity(), animation);
        cx.style_mut().border_width.play_animation(self.entity(), animation);
        cx.style_mut().border_bottom_left_radius.play_animation(self.entity(), animation);
        cx.style_mut().border_top_left_radius.play_animation(self.entity(), animation);
        cx.style_mut().border_bottom_right_radius.play_animation(self.entity(), animation);
        cx.style_mut().border_top_right_radius.play_animation(self.entity(), animation);

        // Transform
        cx.style_mut().rotate.play_animation(self.entity(), animation);
        // cx.style.translate.play_animation(self.entity(), animation);
        cx.style_mut().scale.play_animation(self.entity(), animation);

        // Display
        cx.style_mut().opacity.play_animation(self.entity(), animation);

        self
    }

    /// Returns true if there is an active animation with the given id.
    ///
    /// # Example
    /// ```ignore
    /// let test = entity.is_animating(animation_id);
    /// ```
    fn is_animating(self, cx: &mut Context, animation: Animation) -> bool {
        cx.style_mut().height.is_animating(self.entity(), animation)
            || cx.style_mut().width.is_animating(self.entity(), animation)
    }
}

impl AnimExt for Entity {
    fn entity(self) -> Entity {
        self
    }
}

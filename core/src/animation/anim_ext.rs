use crate::{Animation, AsEntity, Context};

/// Trait which provides methods for entities to manipulate linked animations
pub trait AnimExt: AsEntity + Sized {
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
        cx.style.background_color.play_animation(self.entity(), animation);

        // Space
        cx.style.left.play_animation(self.entity(), animation);
        cx.style.right.play_animation(self.entity(), animation);
        cx.style.top.play_animation(self.entity(), animation);
        cx.style.bottom.play_animation(self.entity(), animation);

        // Min/Max Space
        cx.style.min_left.play_animation(self.entity(), animation);
        cx.style.min_right.play_animation(self.entity(), animation);
        cx.style.min_top.play_animation(self.entity(), animation);
        cx.style.min_bottom.play_animation(self.entity(), animation);
        cx.style.max_left.play_animation(self.entity(), animation);
        cx.style.max_right.play_animation(self.entity(), animation);
        cx.style.max_top.play_animation(self.entity(), animation);
        cx.style.max_bottom.play_animation(self.entity(), animation);

        // Child Space
        cx.style.child_left.play_animation(self.entity(), animation);
        cx.style.child_right.play_animation(self.entity(), animation);
        cx.style.child_top.play_animation(self.entity(), animation);
        cx.style.child_bottom.play_animation(self.entity(), animation);

        // Size
        cx.style.width.play_animation(self.entity(), animation);
        cx.style.height.play_animation(self.entity(), animation);

        // Min/Max Size
        cx.style.min_width.play_animation(self.entity(), animation);
        cx.style.min_height.play_animation(self.entity(), animation);
        cx.style.max_width.play_animation(self.entity(), animation);
        cx.style.max_height.play_animation(self.entity(), animation);

        // Border
        cx.style.border_color.play_animation(self.entity(), animation);
        cx.style.border_width.play_animation(self.entity(), animation);
        cx.style.border_radius_bottom_left.play_animation(self.entity(), animation);
        cx.style.border_radius_top_left.play_animation(self.entity(), animation);
        cx.style.border_radius_bottom_right.play_animation(self.entity(), animation);
        cx.style.border_radius_top_right.play_animation(self.entity(), animation);

        // Transform
        cx.style.rotate.play_animation(self.entity(), animation);
        // cx.style.translate.play_animation(self.entity(), animation);
        cx.style.scale.play_animation(self.entity(), animation);

        // Display
        cx.style.opacity.play_animation(self.entity(), animation);

        self
    }

    /// Returns true if there is an active animation with the given id.
    ///
    /// # Example
    /// ```ignore
    /// let test = entity.is_animating(animation_id);
    /// ```
    fn is_animating(self, cx: &mut Context, animation: Animation) -> bool {
        cx.style.height.is_animating(self.entity(), animation)
            || cx.style.width.is_animating(self.entity(), animation)
    }
}

impl<T: AsEntity> AnimExt for T {}

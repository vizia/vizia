use crate::{Animation, AsEntity, State};

/// Trait which provides methods for entities to manipulate linked animations
pub trait AnimExt: AsEntity + Sized {

    /// Play an animation on the entity.
    ///
    /// Internally this generates an active animation and links the entity to it for each animated property.
    ///
    /// # Example
    /// Create an animation which animates the `left` property from 0 to 100 pixels in 5 seconds 
    /// and play the animation on an entity:
    /// ```
    /// let animation_id = state.create_animation(std::time::Duration::from_secs(5))
    ///     .add_keyframe(0.0, |keyframe| keyframe.set_left(Pixels(0.0)))
    ///     .add_keyframe(1.0, |keyframe| keyframe.set_left(Pixels(100.0)))
    ///     .build(); 
    ///
    /// entity.play_animation(state, animation_id);
    /// ```
    fn play_animation(self, state: &mut State, animation: Animation) -> Self {

        // Background
        state.style.background_color.play_animation(self.entity(), animation);

        // Space
        state.style.left.play_animation(self.entity(), animation);
        state.style.right.play_animation(self.entity(), animation);
        state.style.top.play_animation(self.entity(), animation);
        state.style.bottom.play_animation(self.entity(), animation);

        // Min/Max Space
        state.style.min_left.play_animation(self.entity(), animation);
        state.style.min_right.play_animation(self.entity(), animation);
        state.style.min_top.play_animation(self.entity(), animation);
        state.style.min_bottom.play_animation(self.entity(), animation);
        state.style.max_left.play_animation(self.entity(), animation);
        state.style.max_right.play_animation(self.entity(), animation);
        state.style.max_top.play_animation(self.entity(), animation);
        state.style.max_bottom.play_animation(self.entity(), animation);

        // Child Space
        state.style.child_left.play_animation(self.entity(), animation);
        state.style.child_right.play_animation(self.entity(), animation);
        state.style.child_top.play_animation(self.entity(), animation);
        state.style.child_bottom.play_animation(self.entity(), animation);

        // Size
        state.style.width.play_animation(self.entity(), animation);
        state.style.height.play_animation(self.entity(), animation);

        // Min/Max Size
        state.style.min_width.play_animation(self.entity(), animation);
        state.style.min_height.play_animation(self.entity(), animation);
        state.style.max_width.play_animation(self.entity(), animation);
        state.style.max_height.play_animation(self.entity(), animation);

        // Border
        state.style.border_color.play_animation(self.entity(), animation);
        state.style.border_width.play_animation(self.entity(), animation);
        state.style.border_radius_bottom_left.play_animation(self.entity(), animation);
        state.style.border_radius_top_left.play_animation(self.entity(), animation);
        state.style.border_radius_bottom_right.play_animation(self.entity(), animation);
        state.style.border_radius_top_right.play_animation(self.entity(), animation);

        // Transform
        state.style.rotate.play_animation(self.entity(), animation);
        // state.style.translate.play_animation(self.entity(), animation);
        state.style.scale.play_animation(self.entity(), animation);

        // Display
        state.style.opacity.play_animation(self.entity(), animation);


        self
    }

    /// Returns true if there is an active animation with the given id.
    /// 
    /// # Example
    /// ```
    /// let test = entity.is_animation(animation_id);
    /// ```
    fn is_animating(self, state: &mut State, animation: Animation) -> bool {
        state.style.height.is_animating(self.entity(), animation) ||
        state.style.width.is_animating(self.entity(), animation)
    }
}

impl<T: AsEntity> AnimExt for T {

}
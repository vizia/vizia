//! # Animations
//!
//! Nearly every style property can be animated.
//!
//! # Creating Anaimtions
//! To create an animation, call `state.create_animation(duration)`, where the animation `duration` is a `std::time::Duration` type.
//! This function returns an [AnimationBuilder], which can be used to construct the animation of several properties at once.
// !
//! ## Example
//! The following code creates an animation which will animate the `background-color` property from red to blue over 1 second:
//! ```compile_fail
//! let animation = state.create_animation(instant::Duration::from_secs(1))
//!     .add_keyframe(0.0, |keyframe| keyframe.set_background_color(Color::red()))
//!     .add_keyframe(1.0, |keyframe| keyframe.set_background_color(Color::blue()))
//!     .build();
//! ```
//!
//! The `add_keyframe()` method on the [AnimationBuilder] takes two parameters: a value between 0.0 and 1.0, representing a fractional time from
//! between the start of the anmation and the end (start + duration), and a closure which provides a [KeyframeBuilder], which allows multiple different properties to be keyframed.
//! For example, the above animation can be modified to animate both background color and border color simultaneously over the 1 second duration:
//!
//! ## Example
//! ```compile_fail
//! let animation_id = state.create_animation(instant::Duration::from_secs(1))
//!     .add_keyframe(0.0, |keyframe|
//!         keyframe
//!             .set_background_color(Color::red())
//!             .set_border_color(Color::blue())
//!     )
//!     .add_keyframe(1.0, |keyframe|
//!         keyframe
//!             .set_background_color(Color::blue()))
//!             .set_border_color(Color::red())
//!     .build();
//! ```
//! Calling `build()` finishes the animation and returns an [Animation] id. This id can then be used to modify the animation and to link it entities.
//!
//! # Controlling Animations
//! Animations are linked to entities and controlled using methods from the [AnimExt] trait. For example,
//! the following code links an entity to an animation. This causes the animation to become active and play until completion.
//! An animation can be played on muliple entities by calling `.play_animation()` on each of the entities.
//! ```compile_fail
//! entity.play_animation(animation_id);
//! ```
mod animation;
pub use animation::Animation;

mod animation_state;
pub(crate) use animation_state::AnimationState;

mod interpolator;
pub(crate) use interpolator::Interpolator;

mod transition;
pub(crate) use transition::Transition;

mod animation_builder;
pub use animation_builder::*;

mod anim_ext;
pub use anim_ext::AnimExt;

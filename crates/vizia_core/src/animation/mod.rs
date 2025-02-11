//! Animation module.

mod animation_id;
pub use animation_id::{AnimId, Animation};

mod animation_state;
pub(crate) use animation_state::{AnimationState, Keyframe};

mod interpolator;
pub(crate) use interpolator::Interpolator;

mod timing_function;
pub(crate) use timing_function::TimingFunction;

mod animation_builder;
pub use animation_builder::*;

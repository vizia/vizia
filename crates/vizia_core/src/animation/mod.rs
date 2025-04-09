mod animation_id;
pub use animation_id::{AnimId, AnimationId};

mod animation_state;
pub(crate) use animation_state::{AnimationState, Keyframe};

mod interpolator;
pub(crate) use interpolator::Interpolator;

mod timing_function;
pub use timing_function::TimingFunction;

mod animation_builder;
pub use animation_builder::*;

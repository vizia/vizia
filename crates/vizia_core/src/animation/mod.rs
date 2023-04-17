mod animation;
pub(crate) use animation::Animation;

mod animation_state;
pub(crate) use animation_state::{AnimationState, Keyframe};

mod interpolator;
pub(crate) use interpolator::Interpolator;

mod timing_function;
pub(crate) use timing_function::TimingFunction;

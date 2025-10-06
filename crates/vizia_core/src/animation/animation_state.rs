use crate::animation::Interpolator;
use hashbrown::HashSet;
use vizia_style::{AnimationDirection, AnimationFillMode, AnimationIterationCount};

use crate::prelude::*;

use super::TimingFunction;

/// A keyframe in an animation state.
#[derive(Debug, Clone)]
pub(crate) struct Keyframe<T: Interpolator> {
    pub time: f32,
    pub value: T,
    pub timing_function: Option<TimingFunction>,
}

/// Represents an animation of a property with type `T`.
#[derive(Clone, Debug)]
pub(crate) struct AnimationState<T: Interpolator> {
    /// ID of the animation description.
    pub id: AnimationId,
    /// The start time of the animation.
    pub start_time: Instant,
    /// The duration of the animation.
    pub duration: Duration,
    /// The delay before the animation starts.
    pub delay: Duration,
    /// List of animation keyframes as (normalized time, value).
    pub keyframes: Vec<Keyframe<T>>,
    /// The output of value of the animation.
    pub output: Option<T>,
    /// Whether the animation should persist after finishing.
    pub fill_mode: AnimationFillMode,
    /// The number of times the animation should repeat.
    pub iteration_count: AnimationIterationCount,
    /// The current iteration of the animation.
    pub current_iteration: u32,
    /// The easing function to use for the animation.
    pub easing_function: TimingFunction,
    /// The direction of the animation.
    pub direction: AnimationDirection,
    /// How far through the animation between 0.0 and 1.0.
    pub t: f32,

    pub dt: f32,

    pub active: bool,

    /// For transitions. The starting rule for this transition.
    pub from_rule: usize,
    /// For tansitions. The ending rule for this transition.
    pub to_rule: usize,

    /// List of entities connected to this animation (used when animation is removed from active list)
    pub entities: HashSet<Entity>,
}

impl<T> AnimationState<T>
where
    T: Interpolator,
{
    /// Create a new animation state with the given [Animation] id.
    pub(crate) fn new(id: AnimationId) -> Self {
        AnimationState {
            id,
            start_time: Instant::now(),
            duration: Duration::new(0, 0),
            delay: Duration::new(0, 0),
            keyframes: Vec::new(),
            output: None,
            fill_mode: AnimationFillMode::None,
            iteration_count: AnimationIterationCount::Count(1),
            current_iteration: 0,
            easing_function: TimingFunction::linear(),
            direction: AnimationDirection::Normal,
            t: 0.0,
            dt: 0.0,
            active: false,
            entities: HashSet::new(),
            from_rule: usize::MAX,
            to_rule: usize::MAX,
        }
    }

    pub(crate) fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;

        self
    }

    pub(crate) fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;

        self
    }

    pub(crate) fn with_keyframe(mut self, key: Keyframe<T>) -> Self {
        self.keyframes.push(key);

        self
    }

    pub(crate) fn get_output(&self) -> Option<&T> {
        self.output.as_ref()
    }

    pub(crate) fn play(&mut self, entity: Entity) {
        self.active = true;
        self.t = 0.0;
        self.start_time = Instant::now();
        self.entities.insert(entity);
    }

    pub(crate) fn should_persist(&self) -> bool {
        match self.fill_mode {
            AnimationFillMode::None => false,
            AnimationFillMode::Forwards => true,
            AnimationFillMode::Backwards => false,
            AnimationFillMode::Both => true,
        }
    }

    pub(crate) fn is_transition(&self) -> bool {
        !(self.from_rule == usize::MAX && self.to_rule == usize::MAX)
    }
}

impl<Prop> Default for AnimationState<Prop>
where
    Prop: Interpolator,
{
    fn default() -> Self {
        AnimationState {
            id: AnimationId::null(),
            start_time: Instant::now(),
            duration: Duration::new(0, 0),
            delay: Duration::new(0, 0),
            keyframes: Vec::new(),
            output: None,
            fill_mode: AnimationFillMode::None,
            iteration_count: AnimationIterationCount::Count(1),
            current_iteration: 0,
            easing_function: TimingFunction::linear(),
            direction: AnimationDirection::Normal,
            t: 0.0,
            dt: 0.0,
            active: false,
            entities: HashSet::new(),
            from_rule: usize::MAX,
            to_rule: usize::MAX,
        }
    }
}

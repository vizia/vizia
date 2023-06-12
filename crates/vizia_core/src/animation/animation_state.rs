use crate::animation::Interpolator;
use instant::{Duration, Instant};
use std::collections::HashSet;
use vizia_id::GenerationalId;

use crate::prelude::*;

use super::{Animation, TimingFunction};

/// A keyframe in an animation state.
#[derive(Debug, Clone)]
pub(crate) struct Keyframe<T: Interpolator> {
    #[allow(dead_code)] // FIXME
    pub time: f32,
    pub value: T,
    pub timing_function: TimingFunction,
}

/// Represents an animation of a property with type `T`.
#[derive(Clone, Debug)]
pub(crate) struct AnimationState<T: Interpolator> {
    /// ID of the animation description.
    pub id: Animation,
    /// The start time of the animation.
    pub start_time: Instant,
    /// The duration of the animation.
    pub duration: Duration,
    /// The delay before the animation starts.
    pub delay: f32,
    /// List of animation keyframes as (normalized time, value).
    pub keyframes: Vec<Keyframe<T>>,
    /// The output of value of the animation.
    pub output: Option<T>,
    /// Whether the animation should persist after finishing.
    pub persistent: bool,

    // pub t0: f32,
    /// How far through the animation between 0.0 and 1.0.
    pub t: f32,

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
    pub(crate) fn new(id: Animation) -> Self {
        AnimationState {
            id,
            start_time: Instant::now(),
            duration: Duration::new(0, 0),
            delay: 0.0,
            keyframes: Vec::new(),
            output: None,
            persistent: false,
            t: 0.0,
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

    pub(crate) fn with_delay(mut self, delay: Option<Duration>) -> Self {
        if let Some(delay) = delay {
            self.delay = delay.as_secs_f32() / self.duration.as_secs_f32();
        }

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
        self.start_time = instant::Instant::now();
        self.entities.insert(entity);
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
            id: Animation::null(),
            start_time: Instant::now(),
            duration: Duration::new(0, 0),
            delay: 0.0,
            keyframes: Vec::new(),
            output: None,
            persistent: true,
            t: 0.0,
            active: false,
            entities: HashSet::new(),
            from_rule: std::usize::MAX,
            to_rule: std::usize::MAX,
        }
    }
}

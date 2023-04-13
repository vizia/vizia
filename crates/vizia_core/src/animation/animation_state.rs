use crate::animation::Interpolator;
use instant::{Duration, Instant};
use std::collections::HashSet;
use vizia_id::GenerationalId;

use crate::prelude::*;

use super::{Animation, TimingFunction};

#[derive(Debug, Clone)]
pub(crate) struct Keyframe<T: Interpolator> {
    pub time: f32,
    pub value: T,
    pub timing_function: TimingFunction,
}

#[derive(Clone, Debug)]
pub(crate) struct AnimationState<T: Interpolator> {
    /// ID of the animation description.
    pub id: Animation,
    /// List of property indices that this animation applies to.
    pub indices: Vec<usize>,
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

    pub t0: f32,
    /// How far through the animation between 0.0 and 1.0.
    pub t: f32,

    pub active: bool,

    /// For transitions. The starting rule for this transition.
    pub from_rule: usize,
    /// For tansitions. The ending rule for this transition.
    pub to_rule: usize,

    /// The number of entities linked to this animation when playing
    pub count: usize,

    /// List of entities connected to this animation (used when animation is removed from active list)
    pub entities: HashSet<Entity>,
}

impl<T> AnimationState<T>
where
    T: Interpolator,
{
    pub(crate) fn new(id: Animation) -> Self {
        AnimationState {
            id,
            indices: Vec::new(),
            start_time: Instant::now(),
            duration: Duration::new(0, 0),
            delay: 0.0,
            keyframes: Vec::new(),
            output: None,
            persistent: false,
            t0: 0.0,
            t: 0.0,
            active: false,
            entities: HashSet::new(),
            from_rule: usize::MAX,
            to_rule: usize::MAX,
            count: 0,
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

    pub(crate) fn set_delay(&mut self, delay: Duration) -> &mut Self {
        self.delay = delay.as_secs_f32() / self.duration.as_secs_f32();

        self
    }

    pub(crate) fn with_keyframe(mut self, key: Keyframe<T>) -> Self {
        self.keyframes.push(key);

        self
    }

    pub(crate) fn interpolate(&mut self, current_time: Instant) -> bool {
        if current_time > self.start_time + self.duration {
            return false;
        }
        // println!("Animating");

        //let point = self.start_time.elapsed().as_secs_f32() / self.duration.as_secs_f32();

        //let value = Prop::interpolate((0.0,1.0), (&self.keyframes[0].1, &self.keyframes[1].1), point);
        // use the keyframes to interpolate the value and store the result in output.
        //let mut pos = Positioning::default();

        //let i = Prop::interpolate(0.0, Prop::default(), 1.0, Prop::default())

        //let i = pos.interpolate();

        true
    }

    pub(crate) fn set_persistent(mut self, flag: bool) -> Self {
        self.persistent = flag;

        self
    }

    pub(crate) fn get_output(&self) -> Option<&T> {
        self.output.as_ref()
    }

    pub(crate) fn play(&mut self, entity: Entity) {
        self.t0 = 0.0;
        self.active = true;
        self.t = 0.0;
        self.start_time = instant::Instant::now();
        self.entities.insert(entity);
    }
}

impl<Prop> Default for AnimationState<Prop>
where
    Prop: Interpolator,
{
    fn default() -> Self {
        AnimationState {
            id: Animation::null(),
            indices: Vec::new(),
            start_time: Instant::now(),
            duration: Duration::new(0, 0),
            delay: 0.0,
            keyframes: Vec::new(),
            output: None,
            persistent: true,
            t0: 0.0,
            t: 0.0,
            active: false,
            entities: HashSet::new(),
            from_rule: std::usize::MAX,
            to_rule: std::usize::MAX,
            count: 0,
        }
    }
}

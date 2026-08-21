use crate::animation::Interpolator;
use hashbrown::HashSet;
use vizia_style::AnimationComposition;

use crate::prelude::*;

use super::{CssAnimationClock, CssAnimationTiming, TimingFunction};

/// A keyframe in an animation state.
#[derive(Debug, Clone)]
pub(crate) struct Keyframe<T: Interpolator> {
    pub time: f32,
    pub value: T,
    pub timing_function: TimingFunction,
}

/// Represents an animation of a property with type `T`.
#[derive(Clone, Debug)]
pub(crate) struct AnimationState<T: Interpolator> {
    pub id: Animation,
    pub start_time: Instant,
    pub duration: Duration,
    pub delay: Duration,
    pub keyframes: Vec<Keyframe<T>>,
    pub output: Option<T>,
    pub composed_output: Option<T>,
    pub persistent: bool,
    pub t: f32,
    pub dt: f32,
    pub active: bool,
    pub from_rule: usize,
    pub to_rule: usize,
    pub entities: HashSet<Entity>,
    pub css_clock: Option<CssAnimationClock>,
    pub css_default_timing: TimingFunction,
    pub css_instance_id: Option<u64>,
    pub css_order: usize,
    pub css_composition: AnimationComposition,
    pub css_timeline_driven: bool,
    pub css_timeline_progress: Option<f32>,
}

impl<T> AnimationState<T>
where
    T: Interpolator,
{
    pub(crate) fn new(id: Animation) -> Self {
        AnimationState {
            id,
            start_time: Instant::now(),
            duration: Duration::ZERO,
            delay: Duration::ZERO,
            keyframes: Vec::new(),
            output: None,
            composed_output: None,
            persistent: false,
            t: 0.0,
            dt: 0.0,
            active: false,
            entities: HashSet::new(),
            from_rule: usize::MAX,
            to_rule: usize::MAX,
            css_clock: None,
            css_default_timing: TimingFunction::ease(),
            css_instance_id: None,
            css_order: 0,
            css_composition: AnimationComposition::Replace,
            css_timeline_driven: false,
            css_timeline_progress: None,
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

    pub(crate) fn configure_css(
        &mut self,
        timing: CssAnimationTiming,
        default_timing: TimingFunction,
        start_time: Instant,
    ) {
        self.start_time = start_time;
        self.duration = Duration::from_secs_f32(timing.duration.max(0.0));
        self.delay = Duration::ZERO;
        self.dt = 0.0;
        self.css_clock = Some(CssAnimationClock::new(timing, start_time));
        self.css_default_timing = default_timing;
        self.persistent = matches!(
            timing.fill_mode,
            vizia_style::AnimationFillMode::Forwards | vizia_style::AnimationFillMode::Both
        );
        self.active = true;
        self.t = 0.0;
    }

    pub(crate) fn update_css_timing(
        &mut self,
        timing: CssAnimationTiming,
        default_timing: TimingFunction,
        now: Instant,
    ) {
        if let Some(clock) = &mut self.css_clock {
            clock.update_timing(timing, now);
        }
        self.css_default_timing = default_timing;
        self.persistent = matches!(
            timing.fill_mode,
            vizia_style::AnimationFillMode::Forwards | vizia_style::AnimationFillMode::Both
        );
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
        Self {
            id: Animation::null(),
            start_time: Instant::now(),
            duration: Duration::ZERO,
            delay: Duration::ZERO,
            keyframes: Vec::new(),
            output: None,
            composed_output: None,
            persistent: true,
            t: 0.0,
            dt: 0.0,
            active: false,
            entities: HashSet::new(),
            from_rule: usize::MAX,
            to_rule: usize::MAX,
            css_clock: None,
            css_default_timing: TimingFunction::ease(),
            css_instance_id: None,
            css_order: 0,
            css_composition: AnimationComposition::Replace,
            css_timeline_driven: false,
            css_timeline_progress: None,
        }
    }
}

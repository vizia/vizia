use std::{cmp::Ordering, rc::Rc};

use web_time::{Duration, Instant};

use crate::{context::EventContext, entity::Entity};

/// Enum which can be used to determine the reason a timer callback was called.
///
/// When a timer is ticked (called after some interval), the frame rate may not be in sync with the timer rate.
/// This will affect the accuracy of the timer. To account for this, the `Tick` variant provides a `delta` duration,
/// which is the time difference between the current frame time and the timer time. Typically this will be 0-2 ms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerAction {
    // The timer was started.
    Start,
    // The timer was ticked, i.e. called after an interval. The `delta` represents the time difference between the current frame time and the timer time.
    Tick(Duration),
    // The timer was stopped.
    Stop,
}

#[derive(Clone)]
pub struct TimerState {
    pub(crate) entity: Entity,
    pub(crate) id: Timer,
    pub(crate) time: Instant,
    pub(crate) interval: Duration,
    pub(crate) duration: Option<Duration>,
    pub(crate) start_time: Instant,
    pub(crate) callback: Rc<dyn Fn(&mut EventContext, TimerAction)>,
    pub(crate) ticking: bool,
    pub(crate) stopping: bool,
}

impl TimerState {
    pub fn start_time(&self) -> Instant {
        self.start_time
    }

    pub fn end_time(&self) -> Option<Instant> {
        self.duration.map(|duration| self.start_time + duration)
    }

    /// Sets the tick interval of the timer.
    pub fn set_interval(&mut self, interval: Duration) -> &mut Self {
        self.interval = interval;

        self
    }

    /// Sets the duration of the timer. Pass `None` for a timer which ticks forever.
    pub fn set_duration(&mut self, duration: Option<Duration>) -> &mut Self {
        self.duration = duration;

        self
    }

    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }

    pub fn interval(&self) -> Duration {
        self.interval
    }

    pub fn progress(&self) -> Option<f32> {
        self.duration.map(|duration| {
            ((self.time - self.start_time).as_secs_f32() / duration.as_secs_f32()).clamp(0.0, 1.0)
        })
    }
}

impl PartialEq<Self> for TimerState {
    fn eq(&self, other: &Self) -> bool {
        self.time.eq(&other.time)
    }
}

impl Eq for TimerState {}

impl PartialOrd for TimerState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimerState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time).reverse()
    }
}

/// A handle used to start, stop, and check the running status of a timer added with `cx.add_timer()`.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Timer(pub usize);

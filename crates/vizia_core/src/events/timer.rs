use std::{cmp::Ordering, rc::Rc};

use instant::{Duration, Instant};

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
pub(crate) struct TimerState {
    pub entity: Entity,
    pub id: Timer,
    pub time: Instant,
    pub interval: Duration,
    pub duration: Option<instant::Duration>,
    pub start_time: instant::Instant,
    pub callback: Rc<dyn Fn(&mut EventContext, TimerAction)>,
}

impl TimerState {
    pub fn end_time(&self) -> Option<instant::Instant> {
        self.duration.map(|duration| self.start_time + duration)
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
        self.time.partial_cmp(&other.time).map(|ord| ord.reverse())
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

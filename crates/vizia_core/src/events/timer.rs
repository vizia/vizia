use std::{cmp::Ordering, rc::Rc};

use instant::{Duration, Instant};

use crate::{context::EventContext, entity::Entity};

pub struct TimerBuilder {
    interval: instant::Duration,
    duration: Option<instant::Duration>,
    callback: Option<Rc<dyn Fn(&mut EventContext)>>,
}

impl TimerBuilder {
    pub fn new(interval: Duration, callback: impl Fn(&mut EventContext) + 'static) -> TimerBuilder {
        Self { interval, callback: Some(Rc::new(callback)), duration: None }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);

        self
    }

    pub(crate) fn build(self, entity: Entity, id: Timer) -> TimerState {
        TimerState {
            entity,
            id,
            time: Instant::now(),
            interval: self.interval,
            duration: self.duration,
            start_time: Instant::now(),
            callback: self.callback,
        }
    }
}

#[derive(Clone)]
pub(crate) struct TimerState {
    pub entity: Entity,
    pub id: Timer,
    pub time: instant::Instant,
    pub interval: instant::Duration,
    pub duration: Option<instant::Duration>,
    pub start_time: instant::Instant,
    pub callback: Option<Rc<dyn Fn(&mut EventContext)>>,
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Timer(pub usize);

use std::{cmp::Ordering, rc::Rc};

use instant::{Duration, Instant};

use crate::{context::EventContext, entity::Entity};

pub enum TimerAction {
    Start,
    Tick(Duration),
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Timer(pub usize);

use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::sync::Mutex;
use vizia_core::context::EventProxy;
use vizia_core::events::Event;

lazy_static! {
    pub(crate) static ref PROXY_QUEUE: Mutex<VecDeque<Event>> = Mutex::new(VecDeque::new());
}

pub(crate) fn queue_put(event: Event) {
    PROXY_QUEUE.lock().unwrap().push_back(event)
}

pub(crate) fn queue_get() -> Option<Event> {
    PROXY_QUEUE.lock().unwrap().pop_front()
}

#[derive(Clone)]
pub(crate) struct BaseviewProxy();

impl EventProxy for BaseviewProxy {
    fn send(&self, event: Event) -> Result<(), ()> {
        queue_put(event);
        Ok(())
    }

    fn make_clone(&self) -> Box<dyn EventProxy> {
        Box::new(self.clone())
    }
}

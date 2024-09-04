use std::collections::VecDeque;
use std::sync::{LazyLock, Mutex};
use vizia_core::context::EventProxy;
use vizia_core::events::Event;

pub(crate) static PROXY_QUEUE: LazyLock<Mutex<VecDeque<Event>>> = LazyLock::new(Mutex::default);

pub(crate) fn queue_put(event: Event) {
    PROXY_QUEUE.lock().unwrap().push_back(event)
}

pub(crate) fn queue_get() -> Option<Event> {
    PROXY_QUEUE.lock().unwrap().pop_front()
}

#[derive(Clone)]
pub(crate) struct BaseviewProxy;

impl EventProxy for BaseviewProxy {
    fn send(&self, event: Event) -> Result<(), ()> {
        queue_put(event);
        Ok(())
    }

    fn make_clone(&self) -> Box<dyn EventProxy> {
        Box::new(self.clone())
    }
}

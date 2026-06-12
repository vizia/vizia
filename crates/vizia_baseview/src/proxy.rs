use std::collections::VecDeque;
use std::sync::{LazyLock, Mutex};
use vizia_core::context::EventProxy;
use vizia_core::events::ProxyEvent;

pub(crate) static PROXY_QUEUE: LazyLock<Mutex<VecDeque<ProxyEvent>>> =
    LazyLock::new(Mutex::default);

pub(crate) fn queue_put(event: ProxyEvent) {
    PROXY_QUEUE.lock().unwrap().push_back(event)
}

pub(crate) fn queue_get() -> Option<ProxyEvent> {
    PROXY_QUEUE.lock().unwrap().pop_front()
}

#[derive(Clone)]
pub(crate) struct BaseviewProxy;

impl EventProxy for BaseviewProxy {
    fn send(&self, event: ProxyEvent) -> Result<(), ()> {
        queue_put(event);
        Ok(())
    }

    fn make_clone(&self) -> Box<dyn EventProxy> {
        Box::new(self.clone())
    }
}

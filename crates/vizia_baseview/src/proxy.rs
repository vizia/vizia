use baseview::WindowWaker;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use vizia_core::context::EventProxy;
use vizia_core::events::ProxyEvent;

/// Proxy for sending events from background threads back to the UI thread.
#[derive(Clone)]
pub(crate) struct BaseviewProxy {
    queue: Arc<Mutex<VecDeque<ProxyEvent>>>,
    waker: WindowWaker,
}

impl BaseviewProxy {
    pub(crate) fn new(waker: WindowWaker) -> Self {
        Self { queue: Arc::new(Mutex::new(VecDeque::new())), waker }
    }

    /// Remove and return all events currently waiting in this window's queue.
    pub(crate) fn take_events(&self) -> Vec<ProxyEvent> {
        let mut queue = self.queue.lock().unwrap();
        queue.drain(..).collect()
    }
}

impl EventProxy for BaseviewProxy {
    fn send(&self, event: ProxyEvent) -> Result<(), ()> {
        self.queue.lock().unwrap().push_back(event);
        self.waker.request_redraw();
        Ok(())
    }

    fn make_clone(&self) -> Box<dyn EventProxy> {
        Box::new(self.clone())
    }
}

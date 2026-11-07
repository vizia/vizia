use baseview::WindowWaker;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Weak};
use vizia_core::context::EventProxy;
use vizia_core::events::ProxyEvent;

/// Only the window holds a strong reference, so proxies held by background threads can tell
/// when the window has closed instead of filling a queue nobody drains.
#[derive(Default)]
pub(crate) struct ProxyQueue(Arc<Mutex<VecDeque<ProxyEvent>>>);

impl ProxyQueue {
    pub(crate) fn proxy(&self, waker: WindowWaker) -> BaseviewProxy {
        BaseviewProxy { queue: Arc::downgrade(&self.0), waker }
    }

    /// Remove and return all events currently waiting in this window's queue.
    pub(crate) fn take_events(&self) -> Vec<ProxyEvent> {
        let mut queue = self.0.lock().unwrap();
        queue.drain(..).collect()
    }
}

/// Proxy for sending events from background threads back to the UI thread.
#[derive(Clone)]
pub(crate) struct BaseviewProxy {
    queue: Weak<Mutex<VecDeque<ProxyEvent>>>,
    waker: WindowWaker,
}

impl EventProxy for BaseviewProxy {
    fn send(&self, event: ProxyEvent) -> Result<(), ()> {
        let queue = self.queue.upgrade().ok_or(())?;
        queue.lock().unwrap().push_back(event);
        // Note: when called from main thread during `draw`, baseview will only
        // request a redraw if the frame does. With `FramePolicy::OnDemand` the event
        // then waits for the next wake-up. Checking for pending events in
        // `schedule_next_frame` would cover it.
        self.waker.request_redraw();
        Ok(())
    }

    fn make_clone(&self) -> Box<dyn EventProxy> {
        Box::new(self.clone())
    }
}

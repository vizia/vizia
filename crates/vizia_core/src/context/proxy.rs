use std::any::Any;
use std::fmt::Formatter;
use std::sync::Mutex;

use super::InternalEvent;

use crate::prelude::*;
use crate::resource::ImageRetentionPolicy;

/// A bundle of data representing a snapshot of the context when a thread was spawned.
///
/// It supports a small subset of context operations. You will get one of these passed to you when
/// you create a new thread with the [`spawn`](crate::context::Context::spawn) method on [`Context`].
pub struct ContextProxy {
    pub current: Entity,
    pub event_proxy: Option<Box<dyn EventProxy>>,
}

/// Errors that might occur when emitting an event via a ContextProxy.
#[derive(Debug)]
pub enum ProxyEmitError {
    /// The current runtime does not support proxying events.
    Unsupported,
    /// The event loop has been closed; the application is exiting.
    EventLoopClosed,
}

impl std::fmt::Display for ProxyEmitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProxyEmitError::Unsupported => {
                f.write_str("The current runtime does not support proxying events")
            }
            ProxyEmitError::EventLoopClosed => {
                f.write_str("Sending an event to an event loop which has been closed")
            }
        }
    }
}

impl std::error::Error for ProxyEmitError {}

impl ContextProxy {
    pub fn emit<M: Any + Send>(&mut self, message: M) -> Result<(), ProxyEmitError> {
        if let Some(proxy) = &self.event_proxy {
            let event = Event::new(message)
                .target(self.current)
                .origin(self.current)
                .propagate(Propagation::Up);

            proxy.send(event).map_err(|_| ProxyEmitError::EventLoopClosed)
        } else {
            Err(ProxyEmitError::Unsupported)
        }
    }

    pub fn emit_to<M: Any + Send>(
        &mut self,
        target: Entity,
        message: M,
    ) -> Result<(), ProxyEmitError> {
        if let Some(proxy) = &self.event_proxy {
            let event = Event::new(message)
                .target(target)
                .origin(self.current)
                .propagate(Propagation::Direct);

            proxy.send(event).map_err(|_| ProxyEmitError::EventLoopClosed)
        } else {
            Err(ProxyEmitError::Unsupported)
        }
    }

    pub fn redraw(&mut self) -> Result<(), ProxyEmitError> {
        self.emit(InternalEvent::Redraw)
    }

    pub fn load_image(
        &mut self,
        path: String,
        image: image::DynamicImage,
        policy: ImageRetentionPolicy,
    ) -> Result<(), ProxyEmitError> {
        self.emit(InternalEvent::LoadImage { path, image: Mutex::new(Some(image)), policy })
    }

    pub fn spawn<F>(&self, target: F)
    where
        F: 'static + Send + FnOnce(&mut ContextProxy),
    {
        let mut cxp = self.clone();
        std::thread::spawn(move || target(&mut cxp));
    }
}

impl Clone for ContextProxy {
    fn clone(&self) -> Self {
        Self {
            current: self.current,
            event_proxy: self.event_proxy.as_ref().map(|p| p.make_clone()),
        }
    }
}

pub trait EventProxy: Send {
    #[allow(clippy::result_unit_err)]
    fn send(&self, event: Event) -> Result<(), ()>;
    fn make_clone(&self) -> Box<dyn EventProxy>;
}

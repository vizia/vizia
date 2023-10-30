use crate::entity::Entity;
use instant::Instant;
use std::{any::Any, cmp::Ordering, fmt::Debug};
use vizia_id::GenerationalId;

/// Determines how an event propagates through the tree.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Propagation {
    // /// Events propagate down the tree to the target entity, e.g. from grand-parent to parent to child (target)
    // Down,
    /// Events propagate up the tree from the target entity from ancestor to ancestor, e.g. from child (target) to parent to grand-parent etc.
    Up,
    // /// Events propagate down the tree to the target entity and then back up to the root
    // DownUp,
    /// Events propagate starting at the target entity and visiting every entity that is a descendent of the target.
    Subtree,
    /// Events propagate directly to the target entity and to no others.
    Direct,
}

/// A wrapper around a message, providing metadata on how the event travels through the view tree.
pub struct Event {
    /// The meta data of the event
    pub(crate) meta: EventMeta,
    /// The message of the event
    pub(crate) message: Option<Box<dyn Any + Send>>,
}

impl Debug for Event {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Event {
    /// Creates a new event with a specified message.
    pub fn new<M>(message: M) -> Self
    where
        M: Any + Send,
    {
        Event { meta: Default::default(), message: Some(Box::new(message)) }
    }

    /// Sets the target of the event.
    pub fn target(mut self, entity: Entity) -> Self {
        self.meta.target = entity;
        self
    }

    /// Sets the origin of the event.
    pub fn origin(mut self, entity: Entity) -> Self {
        self.meta.origin = entity;
        self
    }

    /// Sets the propagation of the event.
    pub fn propagate(mut self, propagation: Propagation) -> Self {
        self.meta.propagation = propagation;
        self
    }

    /// Sets the propagation to directly target the `entity`.
    pub fn direct(mut self, entity: Entity) -> Self {
        self.meta.propagation = Propagation::Direct;
        self.meta.target = entity;
        self
    }

    /// Consumes the event to prevent it from continuing on its propagation path.
    pub fn consume(&mut self) {
        self.meta.consume();
    }

    /// Tries to downcast the event message to the specified type. If the downcast was successful,
    /// the message and the event metadata get passed into `f`.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # let cx = &mut Context::default();
    /// # use vizia_winit::application::Application;
    /// # pub struct AppData {
    /// #     count: i32,
    /// # }
    /// # pub enum AppEvent {
    /// #     Increment,
    /// #     Decrement,    
    /// # }
    /// # impl Model for AppData {
    /// #     fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
    /// event.map(|app_event, _| match app_event {
    ///     AppEvent::Increment => {
    ///         self.count += 1;
    ///     }
    ///
    ///     AppEvent::Decrement => {
    ///         self.count -= 1;
    ///     }
    /// });
    /// #     }
    /// # }
    /// ```
    pub fn map<M, F>(&mut self, f: F)
    where
        M: Any + Send,
        F: FnOnce(&M, &mut EventMeta),
    {
        if let Some(message) = &self.message {
            if let Some(message) = message.as_ref().downcast_ref() {
                (f)(message, &mut self.meta);
            }
        }
    }

    /// Tries to downcast the event message to the specified type. If the downcast was successful,
    /// return the message by value and consume the event. Otherwise, do nothing.
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let cx = &mut Context::default();
    /// # use vizia_winit::application::Application;
    /// # pub struct AppData {
    /// #     count: i32,
    /// # }
    /// # pub enum AppEvent {
    /// #     Increment,
    /// #     Decrement,
    /// # }
    /// # impl Model for AppData {
    /// #     fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
    /// if let Some(app_event) = event.take() {
    ///     match app_event {
    ///         AppEvent::Increment => {
    ///             self.count += 1;
    ///         }
    ///
    ///         AppEvent::Decrement => {
    ///             self.count -= 1;
    ///         }
    ///     }
    /// }
    /// #     }
    /// # }
    /// ```
    pub fn take<M: Any + Send>(&mut self) -> Option<M> {
        if let Some(message) = &self.message {
            if message.as_ref().is::<M>() {
                // Safe to unwrap because we already checked it exists
                let m = self.message.take().unwrap();
                // Safe to unwrap because we already checked it can be cast to M
                let v = m.downcast().unwrap();
                self.meta.consume();
                return Some(*v);
            }
        }
        None
    }
}

/// The metadata of an [`Event`].
pub struct EventMeta {
    /// The entity that produced the event. Entity::null() for OS events or unspecified.
    pub origin: Entity,
    /// The entity the event should be sent to (or from in the case of subtree propagation).
    pub target: Entity,
    /// How the event propagates through the tree.
    pub propagation: Propagation,
    /// Determines whether the event should continue to be propagated.
    pub(crate) consumed: bool,
}

impl EventMeta {
    /// Consumes the event to prevent it from continuing on its propagation path.
    pub fn consume(&mut self) {
        self.consumed = true;
    }
}

impl Default for EventMeta {
    fn default() -> Self {
        Self {
            origin: Entity::null(),
            target: Entity::root(),
            propagation: Propagation::Up,
            consumed: false,
        }
    }
}

/// A handle used to cancel a scheduled event before it is sent with `cx.cancel_scheduled`.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct TimedEventHandle(pub usize);

#[derive(Debug)]
pub(crate) struct TimedEvent {
    pub ident: TimedEventHandle,
    pub event: Event,
    pub time: Instant,
}
impl PartialEq<Self> for TimedEvent {
    fn eq(&self, other: &Self) -> bool {
        self.time.eq(&other.time)
    }
}
impl Eq for TimedEvent {}
impl PartialOrd for TimedEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for TimedEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time).reverse()
    }
}

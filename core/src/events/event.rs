use crate::entity::Entity;
use std::{any::Any, fmt::Debug};

/// Determines how an event propagates through the tree.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Propagation {
    // /// Events propagate down the tree to the target entity, e.g. from grand-parent to parent to child (target)
    // Down,
    /// Events propagate up the tree from the target entity from ancestor to ancestor, e.g. from child (target) to parent to grand-parent etc...
    Up,
    // /// Events propagate down the tree to the target entity and then back up to the root
    // DownUp,
    /// Events propagate starting at the target entity and visiting every entity that is a descendent of the target
    Subtree,
    /// Events propagate directly to the target entity and to no others
    Direct,
}

/// The content of an event.
///
/// A message can be any static type.
pub trait Message: Any + Send {
    /// A `&dyn Any` can be cast to a reference to a concrete type.
    fn as_any(&self) -> &dyn Any;
}

impl dyn Message {
    /// Casts a message to the specified type if the message is of that type.
    pub fn downcast<T: Message>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }
}

// Implements message for any static type that implements Send
impl<S: 'static + Send> Message for S {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A wrapper around a message, providing metadata on how the event travels through the tree.
pub struct Event {
    /// The meta data of the event
    pub meta: EventMeta,
    /// The message of the event
    message: Box<dyn Message>,
}

impl Debug for Event {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Event {
    /// Creates a new event with a specified message
    pub fn new<M>(message: M) -> Self
    where
        M: Message,
    {
        Event { meta: Default::default(), message: Box::new(message) }
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

    // I don't think there's a situation where this is needed
    // /// Consumes the event to prevent it from continuing on its propagation path.
    // pub fn consume(&mut self) {
    //     self.meta.consume();
    // }

    /// Tries to downcast the event message to the specified type. If the downcast was successful,
    /// the downcasted message and the event meta data get passed into `f`.
    pub fn map<M, F>(&mut self, f: F)
    where
        M: Message,
        F: FnOnce(&M, &mut EventMeta),
    {
        if let Some(message) = self.message.downcast() {
            (f)(message, &mut self.meta);
        }
    }
}

/// The meta data of an [`Event`].
pub struct EventMeta {
    /// The entity that produced the event. Entity::null() for OS events or unspecified.
    pub origin: Entity,
    /// The entity the event should be sent to. Entity::null() to send to all entities.
    pub target: Entity,
    /// How the event propagates through the tree.
    pub propagation: Propagation,
    /// Whether the event can be consumed
    pub consumable: bool,
    /// Determines whether the event should continue to be propagated
    pub(crate) consumed: bool,
    /// Specifies an order index which is used to sort the event queue
    pub order: i32,
}

impl EventMeta {
    /// Creates a new event meta.
    pub fn new() -> Self {
        Self::default()
    }
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
            consumable: true,
            consumed: false,
            order: 0,
        }
    }
}

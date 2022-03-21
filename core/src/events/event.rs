use crate::Entity;

use std::any::{Any, TypeId};
use std::fmt::Debug;

/// Determines how the event propagates through the tree
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

/// A message can be any static type.
pub trait Message: Any + Send {
    // An &Any can be cast to a reference to a concrete type.
    fn as_any(&self) -> &dyn Any;
}

impl dyn Message {
    // Check if a message is a certain type
    pub fn is<T: Message>(&self) -> bool {
        // Get TypeId of the type this function is instantiated with
        let t = TypeId::of::<T>();

        // Get TypeId of the type in the trait object
        let concrete = self.type_id();

        // Compare both TypeIds on equality
        t == concrete
    }

    // Casts a message to the specified type if the message is of that type
    pub fn downcast<T>(&mut self) -> Option<&mut T>
    where
        T: Message,
    {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn Message as *mut T)) }
        } else {
            None
        }
    }
}

// Implements message for any static type that implements Clone
impl<S: 'static + Send> Message for S {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// An event is a wrapper around a message and provides metadata on how the event should be propagated through the tree
pub struct Event {
    // The entity that produced the event. Entity::null() for OS events or unspecified.
    pub origin: Entity,
    // The entity the event should be sent to. Entity::null() to send to all entities.
    pub target: Entity,
    // How the event propagates through the tree.
    pub propagation: Propagation,
    // Whether the event can be consumed
    pub consumable: bool,
    // Determines whether the event should continue to be propagated
    pub(crate) consumed: bool,
    // Specifies an order index which is used to sort the event queue
    pub order: i32,
    // The event message
    pub message: Box<dyn Message>,
}

impl Debug for Event {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

// // Allows events to be compared for equality
// impl PartialEq for Event {
//     fn eq(&self, other: &Event) -> bool {
//         self.message.equals_a(&*other.message)
//             //&& self.origin == other.origin
//             && self.target == other.target
//     }
// }

impl Event {
    /// Creates a new event with a specified message
    pub fn new<M>(message: M) -> Self
    where
        M: Message,
    {
        Event {
            origin: Entity::null(),
            target: Entity::root(),
            propagation: Propagation::Up,
            consumable: true,
            consumed: false,
            order: 0,
            message: Box::new(message),
        }
    }

    /// Sets the target of the event
    pub fn target(mut self, entity: Entity) -> Self {
        self.target = entity;
        self
    }

    /// Sets the origin of the event
    pub fn origin(mut self, entity: Entity) -> Self {
        self.origin = entity;
        self
    }

    /// Sets the propagation of the event
    pub fn propagate(mut self, propagation: Propagation) -> Self {
        self.propagation = propagation;

        self
    }

    pub fn direct(mut self, entity: Entity) -> Self {
        self.propagation = Propagation::Direct;
        self.target = entity;
        self
    }

    /// Consumes the event
    /// (prevents the event from continuing on its propagation path)
    pub fn consume(&mut self) {
        self.consumed = true;
    }

    pub fn try_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Message,
    {
        self.message.downcast::<T>()
    }
}

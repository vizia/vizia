use crate::{
    entity::Entity,
    events::{Event, Message, Propagation},
};

use super::{Context, DrawContext, EventContext};

/// A macro for implementing methods on multiple contexts. Adapted from Druid.
///
/// There are a lot of methods defined on multiple contexts; this lets us only
/// have to write them out once.
macro_rules! impl_context_method {
    ($ty:ty,  { $($method:item)+ } ) => {
        impl $ty { $($method)+ }
    };
    ( $ty:ty, $($more:ty),+, { $($method:item)+ } ) => {
        impl_context_method!($ty, { $($method)+ });
        impl_context_method!($($more),+, { $($method)+ });
    };
}

impl_context_method!(EventContext<'_>, DrawContext<'_>, {
    /// Returns the entity id of the current view.
    pub fn current(&self) -> Entity {
        self.current
    }

    /// Returns the entity id of the hovered view.
    pub fn hovered(&self) -> Entity {
        *self.hovered
    }
});

impl_context_method!(Context, EventContext<'_>, {
    /// Send an event containing a message up the tree from the current entity.
    pub fn emit<M: Message>(&mut self, message: M) {
        self.event_queue.push_back(
            Event::new(message)
                .target(self.current)
                .origin(self.current)
                .propagate(Propagation::Up),
        );
    }

    /// Send an event containing a message directly to a specified entity.
    pub fn emit_to<M: Message>(&mut self, target: Entity, message: M) {
        self.event_queue.push_back(
            Event::new(message).target(target).origin(self.current).propagate(Propagation::Direct),
        );
    }

    /// Send an event with custom origin and propagation information.
    pub fn send_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }
});

use crate::id::impl_generational_id;

/// An entity is an identifier used to reference a view; to get/set properties in the context.
///
/// Rather than having widgets own their data, all state is stored in a single database and
/// is stored and loaded using entities.
///
/// This type is part of the prelude.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(u32);

impl Entity {
    /// Creates a new root entity.
    ///
    /// The root entity represents the main window and is always valid. It can be used to set
    /// properties on the primary window, such as background color, as well as sending events
    /// to the window such as [`Restyle`](crate::prelude::WindowEvent::Restyle) and
    /// [`Redraw`](crate::prelude::WindowEvent::Redraw) events.
    pub fn root() -> Self {
        Self(0)
    }
}

impl_generational_id!(Entity);

/// An entity identifier is an identifier that points to an entity (which may change).
///
/// This type is part of the prelude.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityIdentifier(u32);

impl_generational_id!(EntityIdentifier);

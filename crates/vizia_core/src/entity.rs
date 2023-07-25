use vizia_id::{
    impl_generational_id, GenerationalId, GENERATIONAL_ID_GENERATION_MASK,
    GENERATIONAL_ID_INDEX_BITS, GENERATIONAL_ID_INDEX_MASK,
};

/// An entity is an identifier used to reference a view; to get/set properties in the context.
///
/// Rather than having widgets own their data, all state is stored in a single database and
/// is stored and loaded using entities.
///
/// The [root entity](GenerationalId::root) represents the main window and is always valid. It can be used to set
/// properties on the primary window, such as background color, as well as sending events
/// to the window such as [`Restyle`] and [`Redraw`] events.
///
/// [root entity]: GenerationalId::root()
/// [`Restyle`]: crate::prelude::WindowEvent::Restyle
/// [`Redraw`]: crate::prelude::WindowEvent::Redraw
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(u64);

impl_generational_id!(Entity);

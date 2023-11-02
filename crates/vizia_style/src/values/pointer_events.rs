use crate::{macros::define_enum, Parse};

define_enum! {
    /// Determines whether an entity will receive pointer events.
    pub enum PointerEvents {
        /// The entity will receive pointer events unless its parent does not.
        "auto": Auto,
        /// The entity will not receive pointer events.
        "none": None,
    }
}

impl From<bool> for PointerEvents {
    fn from(boolean: bool) -> Self {
        if boolean {
            PointerEvents::Auto
        } else {
            PointerEvents::None
        }
    }
}

impl From<PointerEvents> for bool {
    fn from(pointer_events: PointerEvents) -> Self {
        match pointer_events {
            PointerEvents::Auto => true,
            PointerEvents::None => false,
        }
    }
}

impl Default for PointerEvents {
    fn default() -> Self {
        PointerEvents::Auto
    }
}

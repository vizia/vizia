use crate::{macros::define_enum, Parse};

define_enum! {
    /// Determines whether an entity will be rendered and acted on by the layout system.
    /// To make an entity invisible to rendering but still visible to layout, see [Visibility].
    pub enum Direction {
        /// The entity will be rendered and acted on by the layout system.
        "ltr": Ltr,
        /// The entity will not be rendered and acted on by the layout system.
        "rtl": Rtl,
    }
}

impl From<bool> for Direction {
    fn from(boolean: bool) -> Self {
        if boolean {
            Direction::Ltr
        } else {
            Direction::Rtl
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Ltr
    }
}

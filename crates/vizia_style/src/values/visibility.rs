use crate::{macros::define_enum, Parse};

define_enum! {
    /// Determines whether an entity will be rendered.
    ///
    /// An invisible entity will still be acted upon by the layout system.
    /// Use [`Display`](crate::values::Display) to hide an entity from both rendering and layout.
    pub enum Visibility {
        /// The entity will be rendered.
        "visible": Visible,
        /// The entity will not rendered, but will still be acted upon by the layout system.
        "invisible": Invisible,
    }
}

impl From<bool> for Visibility {
    fn from(boolean: bool) -> Self {
        if boolean {
            Visibility::Visible
        } else {
            Visibility::Invisible
        }
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Visible
    }
}

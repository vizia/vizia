use crate::{define_enum, Parse};

define_enum! {
    /// A border width keyword corresponding to a specific pixel width.
    pub enum BorderWidthKeyword {
        /// Corresponds to the border width 1px.
        "thin": Thin,
        /// Corresponds to the border width 3px.
        "medium": Medium,
        /// Corresponds to the border width 5px.
        "thick": Thick,
    }
}

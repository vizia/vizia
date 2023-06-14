use crate::{define_enum, Parse};

define_enum! {
    /// A vertical position keyword.
    pub enum VerticalPositionKeyword {
        /// The 'top' horizontal position keyword.
        "top": Top,
        /// The 'bottom' horizontal position keyword.
        "bottom": Bottom,
    }
}

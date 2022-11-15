use crate::{define_enum_value, Parse};

define_enum_value! {
    /// A vertical position keyword.
    pub enum VerticalPositionKeyword {
        /// The 'top' horizontal position keyword.
        "top": Top,
        /// The 'bottom' horizontal position keyword.
        "bottom": Bottom,
    }
}

use crate::{Parse, define_enum};

define_enum! {
    /// A vertical position keyword.
    pub enum VerticalPositionKeyword {
        /// The 'top' horizontal position keyword.
        "top": Top,
        /// The 'bottom' horizontal position keyword.
        "bottom": Bottom,
    }
}

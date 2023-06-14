use crate::{define_enum, Parse};

define_enum! {
    /// A horizontal position keyword.
    pub enum HorizontalPositionKeyword {
        /// The 'left' horizontal position keyword.
        "left": Left,
        /// The 'right' horizontal position keyword.
        "right": Right,
    }
}

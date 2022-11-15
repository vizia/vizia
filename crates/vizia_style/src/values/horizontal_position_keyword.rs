use crate::{define_enum_value, Parse};

define_enum_value! {
    /// A horizontal position keyword.
    pub enum HorizontalPositionKeyword {
        /// The 'left' horizontal position keyword.
        "left": Left,
        /// The 'right' horizontal position keyword.
        "right": Right,
    }
}

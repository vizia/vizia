use crate::{define_enum, Parse};

define_enum! {
    /// Determines how overflowed content that is not displayed should be signaled to the user.
    pub enum TextOverflow {
        /// The text is clipped and not accessible.
        "clip": Clip,
        /// Renders an ellipsis ("...") to represent the clipped text.
        "ellipsis": Ellipsis,
    }
}

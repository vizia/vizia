use crate::{define_enum, Parse};

define_enum! {
    /// Determines how to deal with content that overflows the bounding box of the element.
    #[derive(Default)]
    pub enum Overflow {
        /// The overflow is not clipped and renders outside of the bounding box of the element.
        #[default]
        "visible": Visible,
        /// The overflow is clipped and the content can only be scrolled programmatically.
        "hidden": Hidden,
        // /// The overflow is clipped and the content can't be scrolled at all.
        // "clip": Clip,
        // /// The overflow is clipped and scrollbars appears to view the rest of the content.
        // "scroll": Scroll,
        // /// The overflow is clipped and a scrollbar is automatically added on the axis where the overflow happens.
        // "auto": Auto,
    }
}

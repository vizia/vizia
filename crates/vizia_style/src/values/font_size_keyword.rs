use crate::{macros::define_enum, Parse};

define_enum! {
    /// A font size keyword corresponding to a specific font size.
    pub enum FontSizeKeyword {
        /// Corresponds to the font size 8.
        "xx-small": XXSmall,
        /// Corresponds to the font size 10.
        "x-small": XSmall,
        /// Corresponds to the font size 12.
        "small": Small,
        /// Corresponds to the font size 14.
        "medium": Medium,
        /// Corresponds to the font size 16.
        "large": Large,
        /// Corresponds to the font size 18.
        "x-large": XLarge,
        /// Corresponds to the font size 20.
        "xx-large": XXLarge,
    }
}

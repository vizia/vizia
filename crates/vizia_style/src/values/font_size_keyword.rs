use crate::{macros::define_enum, Parse};

define_enum! {
    /// A font size keyword corresponding to a specific font size.
    pub enum FontSizeKeyword {
        /// Corresponds to 60% of medium.
        "xx-small": XXSmall,
        /// Corresponds to 75% of medium.
        "x-small": XSmall,
        /// Corresponds to 89% of medium.
        "small": Small,
        /// Corresponds to the font size 14.
        "medium": Medium,
        /// Corresponds to 20% more than medium.
        "large": Large,
        /// Corresponds to 50% more than medium.
        "x-large": XLarge,
        /// Corresponds to 100% more than medium.
        "xx-large": XXLarge,
        /// Corresponds to 200% more than medium.
        "xxx-large": XXXLarge,
    }
}

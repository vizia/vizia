use crate::{macros::define_enum, Parse};

define_enum! {
    /// A font style.
    pub enum FontStyle {
        "normal": Normal,
        "italic": Italic,
        "oblique": Oblique,
    }
}

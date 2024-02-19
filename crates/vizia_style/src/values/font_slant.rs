use skia_safe::font_style::Slant;

use crate::{macros::define_enum, Parse};

define_enum! {
    /// A font style.
    pub enum FontSlant {
        "normal": Normal,
        "italic": Italic,
        "oblique": Oblique,
    }
}

impl Default for FontSlant {
    fn default() -> Self {
        FontSlant::Normal
    }
}

impl From<FontSlant> for Slant {
    fn from(value: FontSlant) -> Self {
        match value {
            FontSlant::Normal => Slant::Upright,
            FontSlant::Italic => Slant::Italic,
            FontSlant::Oblique => Slant::Oblique,
        }
    }
}

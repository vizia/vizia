use skia_safe::font_style::Slant;

use crate::{macros::define_enum, Parse};

define_enum! {
    #[derive(Default)]
    pub enum FontSlant {
        #[default]
        "normal": Normal,
        "italic": Italic,
        "oblique": Oblique,
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

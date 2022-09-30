//! Lists of names for icon unicode code points.

pub(crate) const ROBOTO_REGULAR: &'static [u8] =
    include_bytes!("../resources/fonts/Roboto-Regular.ttf");
pub(crate) const ROBOTO_BOLD: &'static [u8] = include_bytes!("../resources/fonts/Roboto-Bold.ttf");
pub(crate) const ENTYPO: &'static [u8] = include_bytes!("../resources/fonts/entypo.ttf");
pub(crate) const OPEN_SANS_EMOJI: &'static [u8] =
    include_bytes!("../resources/fonts/OpenSansEmoji.ttf");
pub(crate) const AMIRI_REGULAR: &'static [u8] =
    include_bytes!("../resources/fonts/amiri-regular.ttf");
pub(crate) const MATERIAL_ICONS_REGULAR: &'static [u8] =
    include_bytes!("../resources/fonts/MaterialIcons-Regular.ttf");

pub mod icons_names {
    pub const DOWN: &str = "\u{e75c}";
    pub const LEFT: &str = "\u{e75d}";
    pub const RIGHT: &str = "\u{e75e}";
    pub const UP: &str = "\u{e75f}";
}

pub mod material_names {
    pub const DOWN: &str = "\u{e313}";
    pub const LEFT: &str = "\u{e314}";
    pub const RIGHT: &str = "\u{E315}";
    pub const UP: &str = "\u{e316}";
}

pub mod unicode_names {
    pub const PLUS: &str = "\u{2b}";
    pub const CHECK: &str = "\u{2713}";
    pub const CANCEL: &str = "\u{2715}";
}

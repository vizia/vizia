pub const ROBOTO_REGULAR: &[u8] = include_bytes!("../resources/fonts/Roboto-Regular.ttf");
pub const ROBOTO_BOLD: &[u8] = include_bytes!("../resources/fonts/Roboto-Bold.ttf");
pub const ROBOTO_ITALIC: &[u8] = include_bytes!("../resources/fonts/Roboto-Italic.ttf");
pub const ENTYPO: &[u8] = include_bytes!("../resources/fonts/entypo.ttf");
pub const OPEN_SANS_EMOJI: &[u8] = include_bytes!("../resources/fonts/OpenSansEmoji.ttf");
pub const AMIRI_REGULAR: &[u8] = include_bytes!("../resources/fonts/amiri-regular.ttf");
pub const MATERIAL_ICONS_REGULAR: &[u8] =
    include_bytes!("../resources/fonts/MaterialIcons-Regular.ttf");

pub mod icons_names {
    pub const PLUS: &str = "\u{2b}";
    pub const MINUS: &str = "\u{2d}";
    pub const DOWN: &str = "\u{e75c}";
    pub const LEFT: &str = "\u{e75d}";
    pub const RIGHT: &str = "\u{e75e}";
    pub const UP: &str = "\u{e75f}";
    pub const CHECK: &str = "\u{2713}";
    pub const CANCEL: &str = "\u{2715}";
}

pub mod material_names {
    pub const DOWN: &str = "\u{e313}";
    pub const LEFT: &str = "\u{e314}";
    pub const RIGHT: &str = "\u{E315}";
    pub const UP: &str = "\u{e316}";
    pub const PENCIL: &str = "\u{e150}";
}

pub mod unicode_names {
    pub const PLUS: &str = "\u{2b}";
    pub const MINUS: &str = "\u{2d}";
}

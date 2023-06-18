#[cfg(feature = "embedded_fonts")]
pub const ROBOTO_REGULAR: &[u8] = include_bytes!("../resources/fonts/Roboto-Regular.ttf");
#[cfg(feature = "embedded_fonts")]
pub const ROBOTO_BOLD: &[u8] = include_bytes!("../resources/fonts/Roboto-Bold.ttf");
#[cfg(feature = "embedded_fonts")]
pub const ROBOTO_ITALIC: &[u8] = include_bytes!("../resources/fonts/Roboto-Italic.ttf");

pub const TABLER_ICONS: &[u8] = include_bytes!("../resources/fonts/tabler-icons.ttf");

pub mod icons {

    // Numbers
    pub const ICON_123: &str = "\u{f554}";
    pub const ICON_BOX_MULTIPLE_0: &str = "\u{ee0d}";
    pub const ICON_BOX_MULTIPLE_1: &str = "\u{ee0d}";
    pub const ICON_BOX_MULTIPLE_2: &str = "\u{ee0f}";
    pub const ICON_BOX_MULTIPLE_3: &str = "\u{ee10}";
    pub const ICON_BOX_MULTIPLE_4: &str = "\u{ee11}";
    pub const ICON_BOX_MULTIPLE_5: &str = "\u{ee12}";
    pub const ICON_BOX_MULTIPLE_6: &str = "\u{ee13}";
    pub const ICON_BOX_MULTIPLE_7: &str = "\u{ee14}";
    pub const ICON_BOX_MULTIPLE_8: &str = "\u{ee15}";
    pub const ICON_BOX_MULTIPLE_9: &str = "\u{ee16}";
    pub const ICON_CIRCLE_NUMBER_0: &str = "\u{ee34}";
    pub const ICON_CIRCLE_NUMBER_1: &str = "\u{ee35}";
    pub const ICON_CIRCLE_NUMBER_2: &str = "\u{ee36}";
    pub const ICON_CIRCLE_NUMBER_3: &str = "\u{ee37}";
    pub const ICON_CIRCLE_NUMBER_4: &str = "\u{ee38}";
    pub const ICON_CIRCLE_NUMBER_5: &str = "\u{ee39}";
    pub const ICON_CIRCLE_NUMBER_6: &str = "\u{ee3a}";
    pub const ICON_CIRCLE_NUMBER_7: &str = "\u{ee3b}";
    pub const ICON_CIRCLE_NUMBER_8: &str = "\u{ee3c}";
    pub const ICON_CIRCLE_NUMBER_9: &str = "\u{ee3d}";
    pub const ICON_HEXAGON_NUMBER_0: &str = "\u{f459}";
    pub const ICON_HEXAGON_NUMBER_1: &str = "\u{f45a}";
    pub const ICON_HEXAGON_NUMBER_2: &str = "\u{f45b}";
    pub const ICON_HEXAGON_NUMBER_3: &str = "\u{f45c}";
    pub const ICON_HEXAGON_NUMBER_4: &str = "\u{f45d}";
    pub const ICON_HEXAGON_NUMBER_5: &str = "\u{f45e}";
    pub const ICON_HEXAGON_NUMBER_6: &str = "\u{f45f}";
    pub const ICON_HEXAGON_NUMBER_7: &str = "\u{f460}";
    pub const ICON_HEXAGON_NUMBER_8: &str = "\u{f461}";
    pub const ICON_HEXAGON_NUMBER_9: &str = "\u{f462}";

    // Animals
    pub const ICON_BAT: &str = "\u{f284}";
    pub const ICON_CAT: &str = "\u{f65b}";
    pub const ICON_DEER: &str = "\u{f4c5}";
    pub const ICON_DOG: &str = "\u{f660}";
    pub const ICON_FISH_BONE: &str = "\u{f287}";
    pub const ICON_FISH_OFF: &str = "\u{f12b}";
    pub const ICON_FISH: &str = "\u{ef2b}";
    pub const ICON_PIG_MONEY: &str = "\u{f38c}";
    pub const ICON_PIG_OFF: &str = "\u{f177}";
    pub const ICON_PIG: &str = "\u{ef52}";
    pub const ICON_SPIDER: &str = "\u{f293}";

    // Arrows
    pub const ICON_ARROW_AUTOFIT_CONTENT: &str = "\u{ef31}";
    pub const ICON_ARROW_AUTOFIT_DOWN: &str = "\u{ef32}";
    pub const ICON_ARROW_AUTOFIT_HEIGHT: &str = "\u{ef33}";
    pub const ICON_ARROW_AUTOFIT_LEFT: &str = "\u{ef34}";
    pub const ICON_ARROW_AUTOFIT_RIGHT: &str = "\u{ef35}";
    pub const ICON_ARROW_AUTOFIT_UP: &str = "\u{ef36}";
    pub const ICON_ARROW_AUTOFIT_WIDTH: &str = "\u{ef37}";
    pub const ICON_ARROW_BACK_UP_DOUBLE: &str = "\u{f9ec}";
    pub const ICON_ARROW_BACK_UP: &str = "\u{eb77}";
    pub const ICON_ARROW_BACK: &str = "\u{ea0c}";
    pub const ICON_ARROW_BADGE_DOWN: &str = "\u{f60b}";
    pub const ICON_ARROW_BADGE_LEFT: &str = "\u{f60c}";
    pub const ICON_ARROW_BADGE_RIGHT: &str = "\u{f60d}";
    pub const ICON_ARROW_BADGE_UP: &str = "\u{f60e}";
    pub const ICON_ARROW_BAR_DOWN: &str = "\u{ea0d}";
    pub const ICON_ARROW_BAR_LEFT: &str = "\u{ea0e}";
    pub const ICON_ARROW_BAR_RIGHT: &str = "\u{ea0f}";
    pub const ICON_ARROW_BAR_TO_DOWN: &str = "\u{ec88}";
    pub const ICON_ARROW_BAR_TO_LEFT: &str = "\u{ec89}";
    pub const ICON_ARROW_BAR_TO_RIGHT: &str = "\u{ec8a}";
    pub const ICON_ARROW_BAR_TO_UP: &str = "\u{ec8b}";
    pub const ICON_ARROW_BAR_UP: &str = "\u{ea10}";
    pub const ICON_ARROW_BEAR_LEFT_2: &str = "\u{f044}";
    pub const ICON_ARROW_BEAR_LEFT: &str = "\u{f045}";
    pub const ICON_ARROW_BEAR_RIGHT_2: &str = "\u{f046}";
    pub const ICON_ARROW_BEAR_RIGHT: &str = "\u{f047}";
    pub const ICON_ARROW_BIG_DOWN_LINE: &str = "\u{efe8}";
    pub const ICON_ARROW_BIG_DOWN_LINES: &str = "\u{efe9}";
    pub const ICON_ARROW_BIG_DOWN: &str = "\u{edda}";
    pub const ICON_ARROW_BIG_LEFT_LINE: &str = "\u{efea}";
    pub const ICON_ARROW_BIG_LEFT_LINES: &str = "\u{efeb}";
    pub const ICON_ARROW_BIG_LEFT: &str = "\u{eddb}";
    pub const ICON_ARROW_BIG_RIGHT_LINE: &str = "\u{efec}";
    pub const ICON_ARROW_BIG_RIGHT_LINES: &str = "\u{efed}";
    pub const ICON_ARROW_BIG_RIGHT: &str = "\u{eddc}";
    pub const ICON_ARROW_BIG_UP_LINE: &str = "\u{efee}";
    pub const ICON_ARROW_BIG_UP_LINES: &str = "\u{efef}";
    pub const ICON_ARROW_BIG_UP: &str = "\u{eddd}";

    pub const ICON_CHEVRON_DOWN_LEFT: &str = "\u{ed09}";
    pub const ICON_CHEVRON_DOWN_RIGHT: &str = "\u{ed0a}";
    pub const ICON_CHEVRON_DOWN: &str = "\u{ea5f}";
    pub const ICON_CHEVRON_LEFT: &str = "\u{ea60}";
    pub const ICON_CHEVRON_RIGHT: &str = "\u{ea61}";
    pub const ICON_CHEVRON_UP_LEFT: &str = "\u{ed0b}";
    pub const ICON_CHEVRON_UP_RIGHT: &str = "\u{ed0c}";
    pub const ICON_CHEVRON_UP: &str = "\u{ea62}";

    pub const ICON_COMMAND: &str = "\u{ea78}";

    pub const ICON_CUT: &str = "\u{ea86}";
    pub const ICON_COPY: &str = "\u{ea7a}";
    pub const ICON_CLIPBOARD: &str = "\u{ea6f}";

    pub const ICON_SHARE: &str = "\u{eb21}";
    pub const ICON_SHARE_2: &str = "\u{f799}";
    pub const ICON_SHARE_3: &str = "\u{f7bd}";

    pub const ICON_CURSOR_TEXT: &str = "\u{ee6d}";

    pub const ICON_TRASH: &str = "\u{eb41}";

    pub const ICON_PLUS: &str = "\u{eb0b}";
    pub const ICON_MINUS: &str = "\u{eaf2}";
    pub const ICON_CHECK: &str = "\u{ea5e}";
    pub const ICON_X: &str = "\u{eb55}";

    pub const ICON_MOON: &str = "\u{eaf8}";
    pub const ICON_SUN: &str = "\u{eb30}";

    pub const ICON_EYE: &str = "\u{ea9a}";
    pub const ICON_EYE_OFF: &str = "\u{ecf0}";

    pub const ICON_STAR_FILLED: &str = "\u{f6a6}";

    pub const ICON_EDIT_CIRCLE_OFF: &str = "\u{f11d}";
    pub const ICON_EDIT_CIRCLE: &str = "\u{ee85}";
    pub const ICON_EDIT_OFF: &str = "\u{f11e}";
    pub const ICON_EDIT: &str = "\u{ea98}";
    pub const ICON_FILE_PENCIL: &str = "\u{f039}";
    pub const ICON_PENCIL_MINUS: &str = "\u{f1eb}";
    pub const ICON_PENCIL_OFF: &str = "\u{f173}";
    pub const ICON_PENCIL_PLUS: &str = "\u{f1ec}";
    pub const ICON_PENCIL: &str = "\u{eb04}";

    pub const ICON_PLAYER_PAUSE: &str = "\u{ed45}";
    pub const ICON_PLAYER_PAUSE_FILLED: &str = "\u{f690}";
    pub const ICON_PLAYER_PLAY: &str = "\u{ed46}";
    pub const ICON_PLAYER_PLAY_FILLED: &str = "\u{f691}";
    pub const ICON_PLAYER_RECORD: &str = "\u{ed47}";
    pub const ICON_PLAYER_RECORD_FILLED: &str = "\u{f692}";
    pub const ICON_PLAYER_SKIP_BACK: &str = "\u{ed48}";
    pub const ICON_PLAYER_SKIP_BACK_FILLED: &str = "\u{f693}";
    pub const ICON_PLAYER_SKIP_FORWARD: &str = "\u{ed49}";
    pub const ICON_PLAYER_SKIP_FORWARD_FILLED: &str = "\u{f694}";
    pub const ICON_PLAYER_STOP: &str = "\u{ed4a}";
    pub const ICON_PLAYER_STOP_FILLED: &str = "\u{f695}";
    pub const ICON_REPEAT: &str = "\u{eb72}";
    pub const ICON_REPEAT_OFF: &str = "\u{f18e}";

    pub const ICON_SETTINGS_AUTOMATION: &str = "\u{eb72}";
    pub const ICON_ADJUSTMENTS_ALT: &str = "\u{ec37}";

    pub const ICON_POINTER: &str = "\u{f265}";
    pub const ICON_SEARCH: &str = "\u{eb1c}";
    pub const ICON_SLICE: &str = "\u{ebdb}";
}

use vizia_core::prelude::CursorIcon as ViziaCursorIcon;
use vizia_input::Code as ViziaCode;
use vizia_input::Key as ViziaKey;
use winit::event::VirtualKeyCode as WinitVirtualKeyCode;
use winit::window::CursorIcon as WinitCursorIcon;

pub fn cursor_icon_to_cursor_icon(cursor_icon: ViziaCursorIcon) -> Option<WinitCursorIcon> {
    match cursor_icon {
        ViziaCursorIcon::Default => Some(WinitCursorIcon::Default),
        ViziaCursorIcon::Crosshair => Some(WinitCursorIcon::Crosshair),
        ViziaCursorIcon::Hand => Some(WinitCursorIcon::Hand),
        ViziaCursorIcon::Arrow => Some(WinitCursorIcon::Arrow),
        ViziaCursorIcon::Move => Some(WinitCursorIcon::Move),
        ViziaCursorIcon::Text => Some(WinitCursorIcon::Text),
        ViziaCursorIcon::Wait => Some(WinitCursorIcon::Wait),
        ViziaCursorIcon::Help => Some(WinitCursorIcon::Help),
        ViziaCursorIcon::Progress => Some(WinitCursorIcon::Progress),
        ViziaCursorIcon::NotAllowed => Some(WinitCursorIcon::NotAllowed),
        ViziaCursorIcon::ContextMenu => Some(WinitCursorIcon::ContextMenu),
        ViziaCursorIcon::Cell => Some(WinitCursorIcon::Cell),
        ViziaCursorIcon::VerticalText => Some(WinitCursorIcon::VerticalText),
        ViziaCursorIcon::Alias => Some(WinitCursorIcon::Alias),
        ViziaCursorIcon::Copy => Some(WinitCursorIcon::Copy),
        ViziaCursorIcon::NoDrop => Some(WinitCursorIcon::NoDrop),
        ViziaCursorIcon::Grab => Some(WinitCursorIcon::Grab),
        ViziaCursorIcon::Grabbing => Some(WinitCursorIcon::Grabbing),
        ViziaCursorIcon::AllScroll => Some(WinitCursorIcon::AllScroll),
        ViziaCursorIcon::ZoomIn => Some(WinitCursorIcon::ZoomIn),
        ViziaCursorIcon::ZoomOut => Some(WinitCursorIcon::ZoomOut),
        ViziaCursorIcon::EResize => Some(WinitCursorIcon::EResize),
        ViziaCursorIcon::NResize => Some(WinitCursorIcon::NResize),
        ViziaCursorIcon::NeResize => Some(WinitCursorIcon::NeResize),
        ViziaCursorIcon::NwResize => Some(WinitCursorIcon::NwResize),
        ViziaCursorIcon::SResize => Some(WinitCursorIcon::SResize),
        ViziaCursorIcon::SeResize => Some(WinitCursorIcon::SeResize),
        ViziaCursorIcon::SwResize => Some(WinitCursorIcon::SwResize),
        ViziaCursorIcon::WResize => Some(WinitCursorIcon::WResize),
        ViziaCursorIcon::EwResize => Some(WinitCursorIcon::EwResize),
        ViziaCursorIcon::NsResize => Some(WinitCursorIcon::NsResize),
        ViziaCursorIcon::NeswResize => Some(WinitCursorIcon::NeswResize),
        ViziaCursorIcon::NwseResize => Some(WinitCursorIcon::NwseResize),
        ViziaCursorIcon::ColResize => Some(WinitCursorIcon::ColResize),
        ViziaCursorIcon::RowResize => Some(WinitCursorIcon::RowResize),
        ViziaCursorIcon::None => None,
    }
}

pub fn virtual_key_code_to_code(virtual_key_code: WinitVirtualKeyCode) -> ViziaCode {
    match virtual_key_code {
        WinitVirtualKeyCode::Key1 => ViziaCode::Digit1,
        WinitVirtualKeyCode::Key2 => ViziaCode::Digit2,
        WinitVirtualKeyCode::Key3 => ViziaCode::Digit3,
        WinitVirtualKeyCode::Key4 => ViziaCode::Digit4,
        WinitVirtualKeyCode::Key5 => ViziaCode::Digit5,
        WinitVirtualKeyCode::Key6 => ViziaCode::Digit6,
        WinitVirtualKeyCode::Key7 => ViziaCode::Digit7,
        WinitVirtualKeyCode::Key8 => ViziaCode::Digit8,
        WinitVirtualKeyCode::Key9 => ViziaCode::Digit9,
        WinitVirtualKeyCode::Key0 => ViziaCode::Digit0,
        WinitVirtualKeyCode::A => ViziaCode::KeyA,
        WinitVirtualKeyCode::B => ViziaCode::KeyB,
        WinitVirtualKeyCode::C => ViziaCode::KeyC,
        WinitVirtualKeyCode::D => ViziaCode::KeyD,
        WinitVirtualKeyCode::E => ViziaCode::KeyE,
        WinitVirtualKeyCode::F => ViziaCode::KeyF,
        WinitVirtualKeyCode::G => ViziaCode::KeyG,
        WinitVirtualKeyCode::H => ViziaCode::KeyH,
        WinitVirtualKeyCode::I => ViziaCode::KeyI,
        WinitVirtualKeyCode::J => ViziaCode::KeyJ,
        WinitVirtualKeyCode::K => ViziaCode::KeyK,
        WinitVirtualKeyCode::L => ViziaCode::KeyL,
        WinitVirtualKeyCode::M => ViziaCode::KeyM,
        WinitVirtualKeyCode::N => ViziaCode::KeyN,
        WinitVirtualKeyCode::O => ViziaCode::KeyO,
        WinitVirtualKeyCode::P => ViziaCode::KeyP,
        WinitVirtualKeyCode::Q => ViziaCode::KeyQ,
        WinitVirtualKeyCode::R => ViziaCode::KeyR,
        WinitVirtualKeyCode::S => ViziaCode::KeyS,
        WinitVirtualKeyCode::T => ViziaCode::KeyT,
        WinitVirtualKeyCode::U => ViziaCode::KeyU,
        WinitVirtualKeyCode::V => ViziaCode::KeyV,
        WinitVirtualKeyCode::W => ViziaCode::KeyW,
        WinitVirtualKeyCode::X => ViziaCode::KeyX,
        WinitVirtualKeyCode::Y => ViziaCode::KeyY,
        WinitVirtualKeyCode::Z => ViziaCode::KeyZ,
        WinitVirtualKeyCode::Escape => ViziaCode::Escape,
        WinitVirtualKeyCode::F1 => ViziaCode::F1,
        WinitVirtualKeyCode::F2 => ViziaCode::F2,
        WinitVirtualKeyCode::F3 => ViziaCode::F3,
        WinitVirtualKeyCode::F4 => ViziaCode::F4,
        WinitVirtualKeyCode::F5 => ViziaCode::F5,
        WinitVirtualKeyCode::F6 => ViziaCode::F6,
        WinitVirtualKeyCode::F7 => ViziaCode::F7,
        WinitVirtualKeyCode::F8 => ViziaCode::F8,
        WinitVirtualKeyCode::F9 => ViziaCode::F9,
        WinitVirtualKeyCode::F10 => ViziaCode::F10,
        WinitVirtualKeyCode::F11 => ViziaCode::F11,
        WinitVirtualKeyCode::F12 => ViziaCode::F12,
        WinitVirtualKeyCode::Insert => ViziaCode::Insert,
        WinitVirtualKeyCode::Home => ViziaCode::Home,
        WinitVirtualKeyCode::Delete => ViziaCode::Delete,
        WinitVirtualKeyCode::End => ViziaCode::End,
        WinitVirtualKeyCode::PageDown => ViziaCode::PageDown,
        WinitVirtualKeyCode::PageUp => ViziaCode::PageUp,
        WinitVirtualKeyCode::Left => ViziaCode::ArrowLeft,
        WinitVirtualKeyCode::Up => ViziaCode::ArrowUp,
        WinitVirtualKeyCode::Right => ViziaCode::ArrowRight,
        WinitVirtualKeyCode::Down => ViziaCode::ArrowDown,
        WinitVirtualKeyCode::Back => ViziaCode::Backspace,
        WinitVirtualKeyCode::Return => ViziaCode::Enter,
        WinitVirtualKeyCode::Space => ViziaCode::Space,
        WinitVirtualKeyCode::Numpad0 => ViziaCode::Numpad0,
        WinitVirtualKeyCode::Numpad1 => ViziaCode::Numpad1,
        WinitVirtualKeyCode::Numpad2 => ViziaCode::Numpad2,
        WinitVirtualKeyCode::Numpad3 => ViziaCode::Numpad3,
        WinitVirtualKeyCode::Numpad4 => ViziaCode::Numpad4,
        WinitVirtualKeyCode::Numpad5 => ViziaCode::Numpad5,
        WinitVirtualKeyCode::Numpad6 => ViziaCode::Numpad6,
        WinitVirtualKeyCode::Numpad7 => ViziaCode::Numpad7,
        WinitVirtualKeyCode::Numpad8 => ViziaCode::Numpad8,
        WinitVirtualKeyCode::Numpad9 => ViziaCode::Numpad9,
        WinitVirtualKeyCode::NumpadAdd => ViziaCode::NumpadAdd,
        WinitVirtualKeyCode::NumpadDivide => ViziaCode::NumpadDivide,
        WinitVirtualKeyCode::NumpadDecimal => ViziaCode::NumpadDecimal,
        WinitVirtualKeyCode::NumpadComma => ViziaCode::NumpadComma,
        WinitVirtualKeyCode::NumpadEnter => ViziaCode::NumpadEnter,
        WinitVirtualKeyCode::NumpadEquals => ViziaCode::NumpadEqual,
        WinitVirtualKeyCode::NumpadMultiply => ViziaCode::NumpadMultiply,
        WinitVirtualKeyCode::NumpadSubtract => ViziaCode::NumpadSubtract,
        WinitVirtualKeyCode::Comma => ViziaCode::Comma,
        WinitVirtualKeyCode::Equals => ViziaCode::Equal,
        WinitVirtualKeyCode::Grave => ViziaCode::Backquote,
        WinitVirtualKeyCode::LAlt => ViziaCode::AltLeft,
        WinitVirtualKeyCode::LBracket => ViziaCode::BracketLeft,
        WinitVirtualKeyCode::LControl => ViziaCode::ControlLeft,
        WinitVirtualKeyCode::LShift => ViziaCode::ShiftLeft,
        WinitVirtualKeyCode::Minus => ViziaCode::Minus,
        WinitVirtualKeyCode::Period => ViziaCode::Period,
        WinitVirtualKeyCode::Semicolon => ViziaCode::Semicolon,
        WinitVirtualKeyCode::Slash => ViziaCode::Slash,
        WinitVirtualKeyCode::Tab => ViziaCode::Tab,
        _ => ViziaCode::NonConvert,
    }
}

pub fn scan_code_to_code(scan_code: u32) -> ViziaCode {
    match scan_code {
        0x001 => ViziaCode::Escape,
        0x002 => ViziaCode::Digit1,
        0x003 => ViziaCode::Digit2,
        0x004 => ViziaCode::Digit3,
        0x005 => ViziaCode::Digit4,
        0x006 => ViziaCode::Digit5,
        0x007 => ViziaCode::Digit6,
        0x008 => ViziaCode::Digit7,
        0x009 => ViziaCode::Digit8,
        0x00A => ViziaCode::Digit9,
        0x00B => ViziaCode::Digit0,
        0x00C => ViziaCode::Minus,
        0x00D => ViziaCode::Equal,
        0x00E => ViziaCode::Backspace,
        0x00F => ViziaCode::Tab,
        0x010 => ViziaCode::KeyQ,
        0x011 => ViziaCode::KeyW,
        0x012 => ViziaCode::KeyE,
        0x013 => ViziaCode::KeyR,
        0x014 => ViziaCode::KeyT,
        0x015 => ViziaCode::KeyY,
        0x016 => ViziaCode::KeyU,
        0x017 => ViziaCode::KeyI,
        0x018 => ViziaCode::KeyO,
        0x019 => ViziaCode::KeyP,
        0x01A => ViziaCode::BracketLeft,
        0x01B => ViziaCode::BracketRight,
        0x01C => ViziaCode::Enter,
        0x01D => ViziaCode::ControlLeft,
        0x01E => ViziaCode::KeyA,
        0x01F => ViziaCode::KeyS,
        0x020 => ViziaCode::KeyD,
        0x021 => ViziaCode::KeyF,
        0x022 => ViziaCode::KeyG,
        0x023 => ViziaCode::KeyH,
        0x024 => ViziaCode::KeyJ,
        0x025 => ViziaCode::KeyK,
        0x026 => ViziaCode::KeyL,
        0x027 => ViziaCode::Semicolon,
        0x028 => ViziaCode::Quote,
        0x029 => ViziaCode::Backquote,
        0x02A => ViziaCode::ShiftLeft,
        0x02B => ViziaCode::Backslash,
        0x02C => ViziaCode::KeyZ,
        0x02D => ViziaCode::KeyX,
        0x02E => ViziaCode::KeyC,
        0x02F => ViziaCode::KeyV,
        0x030 => ViziaCode::KeyB,
        0x031 => ViziaCode::KeyN,
        0x032 => ViziaCode::KeyM,
        0x033 => ViziaCode::Comma,
        0x034 => ViziaCode::Period,
        0x035 => ViziaCode::Slash,
        0x036 => ViziaCode::ShiftRight,
        0x037 => ViziaCode::NumpadMultiply,
        0x038 => ViziaCode::AltLeft,
        0x039 => ViziaCode::Space,
        0x03A => ViziaCode::CapsLock,
        0x03B => ViziaCode::F1,
        0x03C => ViziaCode::F2,
        0x03D => ViziaCode::F3,
        0x03E => ViziaCode::F4,
        0x03F => ViziaCode::F5,
        0x040 => ViziaCode::F6,
        0x041 => ViziaCode::F7,
        0x042 => ViziaCode::F8,
        0x043 => ViziaCode::F9,
        0x044 => ViziaCode::F10,
        0x045 => ViziaCode::Pause,
        0x046 => ViziaCode::ScrollLock,
        0x047 => ViziaCode::Numpad7,
        0x048 => ViziaCode::Numpad8,
        0x049 => ViziaCode::Numpad9,
        0x04A => ViziaCode::NumpadSubtract,
        0x04B => ViziaCode::Numpad4,
        0x04C => ViziaCode::Numpad5,
        0x04D => ViziaCode::Numpad6,
        0x04E => ViziaCode::NumpadAdd,
        0x04F => ViziaCode::Numpad1,
        0x050 => ViziaCode::Numpad2,
        0x051 => ViziaCode::Numpad3,
        0x052 => ViziaCode::Numpad0,
        0x053 => ViziaCode::NumpadDecimal,
        0x054 => ViziaCode::PrintScreen,
        0x056 => ViziaCode::IntlBackslash,
        0x057 => ViziaCode::F11,
        0x058 => ViziaCode::F12,
        0x059 => ViziaCode::NumpadEqual,
        0x070 => ViziaCode::KanaMode,
        0x071 => ViziaCode::Lang2,
        0x072 => ViziaCode::Lang1,
        0x073 => ViziaCode::IntlRo,
        0x079 => ViziaCode::Convert,
        0x07B => ViziaCode::NonConvert,
        0x07D => ViziaCode::IntlYen,
        0x07E => ViziaCode::NumpadComma,
        0x110 => ViziaCode::MediaTrackPrevious,
        0x119 => ViziaCode::MediaTrackNext,
        0x11C => ViziaCode::NumpadEnter,
        0x11D => ViziaCode::ControlRight,
        0x120 => ViziaCode::AudioVolumeMute,
        0x121 => ViziaCode::LaunchApp2,
        0x122 => ViziaCode::MediaPlayPause,
        0x124 => ViziaCode::MediaStop,
        0x12E => ViziaCode::AudioVolumeDown,
        0x130 => ViziaCode::AudioVolumeUp,
        0x132 => ViziaCode::BrowserHome,
        0x135 => ViziaCode::NumpadDivide,
        0x137 => ViziaCode::PrintScreen,
        0x138 => ViziaCode::AltRight,
        0x145 => ViziaCode::NumLock,
        0x147 => ViziaCode::Home,
        0x148 => ViziaCode::ArrowUp,
        0x149 => ViziaCode::PageUp,
        0x14B => ViziaCode::ArrowLeft,
        0x14D => ViziaCode::ArrowRight,
        0x14F => ViziaCode::End,
        0x150 => ViziaCode::ArrowDown,
        0x151 => ViziaCode::PageDown,
        0x152 => ViziaCode::Insert,
        0x153 => ViziaCode::Delete,
        0x15B => ViziaCode::MetaLeft,
        0x15C => ViziaCode::MetaRight,
        0x15D => ViziaCode::ContextMenu,
        0x15E => ViziaCode::Power,
        0x165 => ViziaCode::BrowserSearch,
        0x166 => ViziaCode::BrowserFavorites,
        0x167 => ViziaCode::BrowserRefresh,
        0x168 => ViziaCode::BrowserStop,
        0x169 => ViziaCode::BrowserForward,
        0x16A => ViziaCode::BrowserBack,
        0x16B => ViziaCode::LaunchApp1,
        0x16C => ViziaCode::LaunchMail,
        0x16D => ViziaCode::MediaSelect,
        0x1F1 => ViziaCode::Lang2,
        0x1F2 => ViziaCode::Lang1,
        _ => ViziaCode::Unidentified,
    }
}

pub fn virtual_key_code_to_key(virtual_key_code: WinitVirtualKeyCode) -> Option<ViziaKey> {
    match virtual_key_code {
        WinitVirtualKeyCode::Back => Some(ViziaKey::Backspace),
        WinitVirtualKeyCode::Tab => Some(ViziaKey::Tab),
        WinitVirtualKeyCode::Return => Some(ViziaKey::Enter),
        WinitVirtualKeyCode::LShift | WinitVirtualKeyCode::RShift => Some(ViziaKey::Shift),
        WinitVirtualKeyCode::LControl | WinitVirtualKeyCode::RControl => Some(ViziaKey::Control),
        WinitVirtualKeyCode::LAlt | WinitVirtualKeyCode::RAlt => Some(ViziaKey::Alt),
        WinitVirtualKeyCode::Pause => Some(ViziaKey::Pause),
        WinitVirtualKeyCode::Capital => Some(ViziaKey::CapsLock),
        // TODO: disambiguate kana and hangul? same vk
        WinitVirtualKeyCode::Kana => Some(ViziaKey::KanaMode),
        WinitVirtualKeyCode::Kanji => Some(ViziaKey::KanjiMode),
        WinitVirtualKeyCode::Escape => Some(ViziaKey::Escape),
        WinitVirtualKeyCode::PageUp => Some(ViziaKey::PageUp),
        WinitVirtualKeyCode::PageDown => Some(ViziaKey::PageDown),
        WinitVirtualKeyCode::End => Some(ViziaKey::End),
        WinitVirtualKeyCode::Home => Some(ViziaKey::Home),
        WinitVirtualKeyCode::Left => Some(ViziaKey::ArrowLeft),
        WinitVirtualKeyCode::Up => Some(ViziaKey::ArrowUp),
        WinitVirtualKeyCode::Right => Some(ViziaKey::ArrowRight),
        WinitVirtualKeyCode::Down => Some(ViziaKey::ArrowDown),
        WinitVirtualKeyCode::MediaSelect => Some(ViziaKey::Select),
        WinitVirtualKeyCode::Snapshot => Some(ViziaKey::Print),
        WinitVirtualKeyCode::Insert => Some(ViziaKey::Insert),
        WinitVirtualKeyCode::Delete => Some(ViziaKey::Delete),
        WinitVirtualKeyCode::LWin | WinitVirtualKeyCode::RWin => Some(ViziaKey::Meta),
        WinitVirtualKeyCode::Apps => Some(ViziaKey::ContextMenu),
        WinitVirtualKeyCode::Sleep => Some(ViziaKey::Standby),
        WinitVirtualKeyCode::F1 => Some(ViziaKey::F1),
        WinitVirtualKeyCode::F2 => Some(ViziaKey::F2),
        WinitVirtualKeyCode::F3 => Some(ViziaKey::F3),
        WinitVirtualKeyCode::F4 => Some(ViziaKey::F4),
        WinitVirtualKeyCode::F5 => Some(ViziaKey::F5),
        WinitVirtualKeyCode::F6 => Some(ViziaKey::F6),
        WinitVirtualKeyCode::F7 => Some(ViziaKey::F7),
        WinitVirtualKeyCode::F8 => Some(ViziaKey::F8),
        WinitVirtualKeyCode::F9 => Some(ViziaKey::F9),
        WinitVirtualKeyCode::F10 => Some(ViziaKey::F10),
        WinitVirtualKeyCode::F11 => Some(ViziaKey::F11),
        WinitVirtualKeyCode::F12 => Some(ViziaKey::F12),
        WinitVirtualKeyCode::Numlock => Some(ViziaKey::NumLock),
        WinitVirtualKeyCode::Scroll => Some(ViziaKey::ScrollLock),
        // WinitVirtualKeyCode::BROWSER_BACK => Some(ViziaKey::BrowserBack),
        // WinitVirtualKeyCode::BROWSER_FORWARD => Some(ViziaKey::BrowserForward),
        // WinitVirtualKeyCode::BROWSER_REFRESH => Some(ViziaKey::BrowserRefresh),
        // WinitVirtualKeyCode::BROWSER_STOP => Some(ViziaKey::BrowserStop),
        // WinitVirtualKeyCode::BROWSER_SEARCH => Some(ViziaKey::BrowserSearch),
        // WinitVirtualKeyCode::BROWSER_FAVORITES => Some(ViziaKey::BrowserFavorites),
        // WinitVirtualKeyCode::BROWSER_HOME => Some(ViziaKey::BrowserHome),
        // WinitVirtualKeyCode::VOLUME_MUTE => Some(ViziaKey::AudioVolumeMute),
        // WinitVirtualKeyCode::VOLUME_DOWN => Some(ViziaKey::AudioVolumeDown),
        // WinitVirtualKeyCode::VOLUME_UP => Some(ViziaKey::AudioVolumeUp),
        // WinitVirtualKeyCode::MEDIA_NEXT_TRACK => Some(ViziaKey::MediaTrackNext),
        // WinitVirtualKeyCode::MEDIA_PREV_TRACK => Some(ViziaKey::MediaTrackPrevious),
        // WinitVirtualKeyCode::MEDIA_STOP => Some(ViziaKey::MediaStop),
        // WinitVirtualKeyCode::MEDIA_PLAY_PAUSE => Some(ViziaKey::MediaPlayPause),
        // WinitVirtualKeyCode::LAUNCH_MAIL => Some(ViziaKey::LaunchMail),
        // WinitVirtualKeyCode::LAUNCH_MEDIA_SELECT => Some(ViziaKey::LaunchMediaPlayer),
        // WinitVirtualKeyCode::LAUNCH_APP1 => Some(ViziaKey::LaunchApplication1),
        // WinitVirtualKeyCode::LAUNCH_APP2 => Some(ViziaKey::LaunchApplication2),
        // WinitVirtualKeyCode::OEM_ATTN => Some(ViziaKey::Alphanumeric),
        // WinitVirtualKeyCode::CONVERT => Some(ViziaKey::Convert),
        // WinitVirtualKeyCode::MODECHANGE => Some(ViziaKey::ModeChange),
        // WinitVirtualKeyCode::PROCESSKEY => Some(ViziaKey::Process),
        // WinitVirtualKeyCode::ATTN => Some(ViziaKey::Attn),
        // WinitVirtualKeyCode::CRSEL => Some(ViziaKey::CrSel),
        // WinitVirtualKeyCode::EXSEL => Some(ViziaKey::ExSel),
        // WinitVirtualKeyCode::EREOF => Some(ViziaKey::EraseEof),
        // WinitVirtualKeyCode::PLAY => Some(ViziaKey::Play),
        // WinitVirtualKeyCode::ZOOM => Some(ViziaKey::ZoomToggle),
        // WinitVirtualKeyCode::OEM_CLEAR => Some(ViziaKey::Clear),
        _ => None,
    }
}

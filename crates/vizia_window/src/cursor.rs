use std::fmt::Formatter;

/// Describes the icon the mouse cursor should use.
///
/// See the cursor_icon example for a gallery of icons and sample usage.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CursorIcon {
    Default,
    Crosshair,
    Hand,
    Arrow,
    Move,
    Text,
    Wait,
    Help,
    Progress,
    NotAllowed,
    ContextMenu,
    Cell,
    VerticalText,
    Alias,
    Copy,
    NoDrop,
    Grab,
    Grabbing,
    AllScroll,
    ZoomIn,
    ZoomOut,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
    None,
}

impl Default for CursorIcon {
    fn default() -> Self {
        CursorIcon::Default
    }
}

impl std::fmt::Display for CursorIcon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CursorIcon::Default => "default",
                CursorIcon::Crosshair => "crosshair",
                CursorIcon::Hand => "hand",
                CursorIcon::Arrow => "arrow",
                CursorIcon::Move => "move",
                CursorIcon::Text => "text",
                CursorIcon::Wait => "wait",
                CursorIcon::Help => "help",
                CursorIcon::Progress => "progress",
                CursorIcon::NotAllowed => "not-allowed",
                CursorIcon::ContextMenu => "context-menu",
                CursorIcon::Cell => "cell",
                CursorIcon::VerticalText => "vertical-text",
                CursorIcon::Alias => "alias",
                CursorIcon::Copy => "copy",
                CursorIcon::NoDrop => "no-drop",
                CursorIcon::Grab => "grab",
                CursorIcon::Grabbing => "grabbing",
                CursorIcon::AllScroll => "all-scroll",
                CursorIcon::ZoomIn => "zoom-in",
                CursorIcon::ZoomOut => "zoom-out",
                CursorIcon::EResize => "e-resize",
                CursorIcon::NResize => "n-resize",
                CursorIcon::NeResize => "ne-resize",
                CursorIcon::NwResize => "nw-resize",
                CursorIcon::SResize => "s-resize",
                CursorIcon::SeResize => "se-resize",
                CursorIcon::SwResize => "sw-resize",
                CursorIcon::WResize => "w-resize",
                CursorIcon::EwResize => "ew-resize",
                CursorIcon::NsResize => "ns-resize",
                CursorIcon::NeswResize => "nesw-resize",
                CursorIcon::NwseResize => "nwse-resize",
                CursorIcon::ColResize => "col-resize",
                CursorIcon::RowResize => "row-resize",
                CursorIcon::None => "none",
            }
        )
    }
}

/// Describes the icon the mouse cursor should use.
///
/// See the cursor_icon example for a gallery of icons and sample usage.
///
/// This type is part of the prelude.
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

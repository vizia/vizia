use crate::{macros::define_enum, Parse};

define_enum! {
    /// An icon of a cursor.
    pub enum CursorIcon {
        /// The platform-dependent default cursor. Typically an arrow.
        "default": Default,
        /// An arrow which is usually also the default cursor icon.
        "arrow": Arrow,
        /// No cursor is rendered.
        "none": None,

        /// Indicates the table cell or set of cells can be selected.
        "cell": Cell,
        /// Indicates selection.
        "crosshair": Crosshair,
        /// Indicates the text can be selected.
        "text": Text,
        /// Indicates the vertical text can be selected.
        "vertical-text": VerticalText,

        /// Indicates a context menu is available.
        "context-menu": ContextMenu,
        /// Indicates help information is available.
        "help": Help,
        /// Indicates something clickable like a link.
        "hand": Hand,
        /// Indicates the program is busy in the background, but the user can still interact with the interface.
        "progress": Progress,
        /// Indicates the program is busy in the background and the user can't interact with the interface.
        "wait": Wait,

        /// Indicates an alias or shortcut is to be created.
        "alias": Alias,
        /// Indicates something is to be copied.
        "copy": Copy,
        /// Indicates something is to be moved.
        "move": Move,
        /// Indicates an item may not be dropped at the current location.
        "no-drop": NoDrop,
        /// Indicates the requested action will not be carried out.
        "not-allowed": NotAllowed,
        /// Indicates something can be grabbed and moved around.
        "grab": Grab,
        /// Indicates something is being grabbed and moved around.
        "grabbing": Grabbing,

        /// Indicates something can be zoomed in.
        "zoom-in": ZoomIn,
        /// Indicates something can be zoomed out.
        "zoom-out": ZoomOut,

        /// Indicates something can be resized or moved north.
        "n-resize": NResize,
        /// Indicates something can be resized or moved east.
        "e-resize": EResize,
        /// Indicates something can be resized or moved south.
        "s-resize": SResize,
        /// Indicates something can be resized or moved west.
        "w-resize": WResize,

        /// Indicates something can be resized or moved north-east.
        "ne-resize": NeResize,
        /// Indicates something can be resized or moved north-west.
        "nw-resize": NwResize,
        /// Indicates something can be resized or moved south-east.
        "se-resize": SeResize,
        /// Indicates something can be resized or moved south-west.
        "sw-resize": SwResize,

        /// Indicates something can be resized or moved horizontally.
        "ew-resize": EwResize,
        /// Indicates something can be resized or moved vertically.
        "ns-resize": NsResize,

        /// Indicates something can be resized or moved diagonally from north-east to south-west.
        "nesw-resize": NeswResize,
        /// Indicates something can be resized or moved diagonally from north-west to south-east.
        "nwse-resize": NwseResize,

        /// Indicates something can be scrolled/panned in any direction.
        "all-scroll": AllScroll,
        /// Indicates a column can be resized horizontally.
        "col-resize": ColResize,
        /// Indicates a row can be resized vertically.
        "row-resize": RowResize,
    }
}

impl Default for CursorIcon {
    fn default() -> Self {
        CursorIcon::Default
    }
}

/// The logical size of an application window.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowSize {
    /// The width of the window.
    pub width: u32,
    /// The height of the window.
    pub height: u32,
}

impl WindowSize {
    /// Creates a new window size.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl From<(u32, u32)> for WindowSize {
    fn from(s: (u32, u32)) -> Self {
        WindowSize::new(s.0, s.1)
    }
}

impl From<WindowSize> for (u32, u32) {
    fn from(s: WindowSize) -> Self {
        (s.width, s.height)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    #[default]
    TopLeft,
    TopCenter,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Anchor {
    pub fn opposite(&self) -> Self {
        match self {
            Anchor::TopLeft => Anchor::BottomLeft,
            Anchor::TopCenter => Anchor::BottomCenter,
            Anchor::TopRight => Anchor::BottomRight,
            Anchor::Left => Anchor::Right,
            Anchor::Center => Anchor::Center,
            Anchor::Right => Anchor::Left,
            Anchor::BottomLeft => Anchor::TopLeft,
            Anchor::BottomCenter => Anchor::TopCenter,
            Anchor::BottomRight => Anchor::TopRight,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnchorTarget {
    #[default]
    Monitor,
    Window,
    Mouse,
}

/// The logical position of a window in screen coordinates.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowPosition {
    /// The x coordinate of the position.
    pub x: i32,
    /// The y coordinate of the position.
    pub y: i32,
}

impl WindowPosition {
    /// Creates a new window position.
    pub fn new(x: i32, y: i32) -> Self {
        WindowPosition { x, y }
    }
}

impl From<(i32, i32)> for WindowPosition {
    fn from(s: (i32, i32)) -> Self {
        WindowPosition::new(s.0, s.1)
    }
}

impl From<WindowPosition> for (i32, i32) {
    fn from(s: WindowPosition) -> Self {
        (s.x, s.y)
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct WindowButtons: u32 {
        const CLOSE  = 1 << 0;
        const MINIMIZE  = 1 << 1;
        const MAXIMIZE  = 1 << 2;
    }
}

/// Passed to the window to set initial window properties.
#[derive(Clone, Debug)]
pub struct WindowDescription {
    pub title: String,
    pub inner_size: WindowSize,
    pub min_inner_size: Option<WindowSize>,
    pub max_inner_size: Option<WindowSize>,
    /// A scale factor applied on top of any DPI scaling, defaults to 1.0.
    pub user_scale_factor: f64,
    pub position: Option<WindowPosition>,
    pub offset: Option<WindowPosition>,
    pub anchor: Option<Anchor>,
    pub anchor_target: Option<AnchorTarget>,
    pub parent_anchor: Option<Anchor>,
    pub resizable: bool,
    pub minimized: bool,
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
    pub decorations: bool,
    pub undecorated_shadow: bool,
    pub always_on_top: bool,
    pub vsync: bool,
    pub enabled_window_buttons: WindowButtons,

    // Change this to resource id when the resource manager is working
    pub icon: Option<Vec<u8>>,
    pub icon_width: u32,
    pub icon_height: u32,
}

impl Default for WindowDescription {
    fn default() -> Self {
        Self {
            title: "Vizia Application".to_string(),
            inner_size: WindowSize::new(800, 600),
            min_inner_size: Some(WindowSize::new(100, 100)),
            max_inner_size: None,
            user_scale_factor: 1.0,
            position: None,
            offset: None,
            anchor: None,
            anchor_target: None,
            parent_anchor: None,
            resizable: true,
            minimized: true,
            maximized: false,
            visible: true,
            transparent: false,
            decorations: true,
            undecorated_shadow: false,
            always_on_top: false,
            vsync: true,
            enabled_window_buttons: WindowButtons::all(),

            icon: None,
            icon_width: 0,
            icon_height: 0,
        }
    }
}

impl WindowDescription {
    pub fn new() -> Self {
        WindowDescription::default()
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();

        self
    }

    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;

        self
    }

    pub fn with_inner_size(mut self, width: u32, height: u32) -> Self {
        self.inner_size = WindowSize::new(width, height);

        self
    }

    pub fn with_min_inner_size(mut self, width: u32, height: u32) -> Self {
        self.min_inner_size = Some(WindowSize::new(width, height));

        self
    }

    pub fn with_max_inner_size(mut self, width: u32, height: u32) -> Self {
        self.max_inner_size = Some(WindowSize::new(width, height));

        self
    }

    /// Apply a user scale factor to the window. This is separate from any DPI scaling that already
    /// gets applied to the window.
    pub fn with_scale_factor(mut self, factor: f64) -> Self {
        self.user_scale_factor = factor;

        self
    }

    pub fn with_always_on_top(mut self, flag: bool) -> Self {
        self.always_on_top = flag;

        self
    }

    pub fn with_resizable(mut self, flag: bool) -> Self {
        self.resizable = flag;

        self
    }

    pub fn with_icon(mut self, icon: Vec<u8>, width: u32, height: u32) -> Self {
        self.icon = Some(icon);
        self.icon_width = width;
        self.icon_height = height;
        self
    }
}



/// Represents the size of the application window.
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

impl WindowSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Position { x, y }
    }
}

/// Passed to the window to set various window properties
pub struct WindowDescription {
    pub title: String,
    pub inner_size: WindowSize,
    pub min_inner_size: WindowSize,
    pub max_inner_size: Option<WindowSize>,
    pub position: Option<Position>,
    pub resizable: bool,
    //pub fullscreen: 
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
    pub decorations: bool,
    pub always_on_top: bool,
    
    // Change this to resource id when the resource manager is working
    pub icon: Option<Vec<u8>>,
    pub icon_width: u32,
    pub icon_height: u32,
}

impl Default for WindowDescription {
    fn default() -> Self {
        Self {
            title: "Tuix Application".to_string(),
            inner_size: WindowSize::new(800, 600),
            min_inner_size: WindowSize::new(100, 100),
            max_inner_size: None,
            position: None,
            resizable: true,
            maximized: false,
            visible: true,
            transparent: false,
            decorations: true,
            always_on_top: false,

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

    pub fn with_inner_size(mut self, width: u32, height: u32) -> Self {
        self.inner_size = WindowSize::new(width, height);

        self
    }

    pub fn with_min_inner_size(mut self, width: u32, height: u32) -> Self {
        self.min_inner_size = WindowSize::new(width, height);

        self
    }

    pub fn with_max_inner_size(mut self, width: u32, height: u32) -> Self {
        self.max_inner_size = Some(WindowSize::new(width, height));

        self
    }

    pub fn with_icon(mut self, icon: Vec<u8>, width: u32, height: u32) -> Self {
        self.icon = Some(icon);
        self.icon_width = width;
        self.icon_height = height;
        self
    }
}

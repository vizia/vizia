pub struct Enviroment {
    // Signifies whether the app should be rebuilt.
    pub needs_rebuild: bool,
    pub include_default_theme: bool,
}

impl Default for Enviroment {
    fn default() -> Self {
        Enviroment::new()
    }
}

impl Enviroment {
    pub fn new() -> Self {
        Self { needs_rebuild: true, include_default_theme: true }
    }
}

pub trait Env {
    fn ignore_default_styles(self) -> Self;
}

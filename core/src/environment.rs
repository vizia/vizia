pub struct Environment {
    // Signifies whether the app should be rebuilt.
    pub include_default_theme: bool,
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self { include_default_theme: true }
    }
}

/// Methods which control the environment the application will run in. This trait is implemented for
/// Application.
///
/// This trait is part of the prelude.
pub trait Env {
    fn ignore_default_styles(self) -> Self;
}

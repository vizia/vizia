use crate::{context::Context, events::Event, state::Model};

pub struct Environment {
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

pub enum EnvironmentEvent {
    IncludeDefaultTheme(bool),
}

impl Model for Environment {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        event.map(|event, _| match event {
            EnvironmentEvent::IncludeDefaultTheme(flag) => {
                self.include_default_theme = *flag;
            }
        });
    }
}

/// Methods which control the environment the application will run in. This trait is implemented for
/// Application.
pub trait Env {
    fn ignore_default_styles(self) -> Self;
}

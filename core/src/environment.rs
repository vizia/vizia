use unic_langid::LanguageIdentifier;
use vizia_derive::Lens;

use crate::{context::Context, events::Event, state::Lens, state::Model};

#[derive(Lens)]
pub struct Environment {
    pub locale: LanguageIdentifier,
    pub include_default_theme: bool,
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self { locale: LanguageIdentifier::default(), include_default_theme: true }
    }
}

pub enum EnvironmentEvent {
    IncludeDefaultTheme(bool),
    SetLocale(LanguageIdentifier),
    //UseSystemLocale,
}

impl Model for Environment {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        event.map(|event, _| match event {
            EnvironmentEvent::IncludeDefaultTheme(flag) => {
                self.include_default_theme = *flag;
            }

            EnvironmentEvent::SetLocale(locale) => {
                println!("Set the locale: {}", locale);
                self.locale = locale.clone();
            }
        });
    }
}

/// Methods which control the environment the application will run in. This trait is implemented for
/// Application.
pub trait Env {
    fn ignore_default_styles(self) -> Self;
}

use unic_langid::LanguageIdentifier;
use vizia_derive::Lens;

use crate::{context::Context, events::Event, state::Lens, state::Model};

#[derive(Lens)]
pub struct Environment {
    pub locale: LanguageIdentifier,
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
        let locale = sys_locale::get_locale().map(|l| l.parse().ok()).flatten().unwrap_or_default();

        Self { locale, include_default_theme: true }
    }
}

pub enum EnvironmentEvent {
    IncludeDefaultTheme(bool),
    SetLocale(LanguageIdentifier),
    UseSystemLocale,
}

impl Model for Environment {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        event.map(|event, _| match event {
            EnvironmentEvent::IncludeDefaultTheme(flag) => {
                self.include_default_theme = *flag;
            }

            EnvironmentEvent::SetLocale(locale) => {
                self.locale = locale.clone();
            }

            EnvironmentEvent::UseSystemLocale => {
                self.locale =
                    sys_locale::get_locale().map(|l| l.parse().unwrap()).unwrap_or_default();
            }
        });
    }
}

/// Methods which control the environment the application will run in. This trait is implemented for
/// Application.
pub trait Env {
    fn ignore_default_styles(self) -> Self;
}

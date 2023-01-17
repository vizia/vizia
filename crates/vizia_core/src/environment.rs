use crate::{context::EventContext, events::Event, state::Lens, state::Model};
use unic_langid::LanguageIdentifier;
use vizia_derive::Lens;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    DarkMode,
    LightMode,
}

#[derive(Lens)]
pub struct Environment {
    pub locale: LanguageIdentifier,
    pub theme_mode: ThemeMode,
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        let locale = sys_locale::get_locale().map(|l| l.parse().ok()).flatten().unwrap_or_default();

        Self { locale, theme_mode: ThemeMode::DarkMode }
    }
}

pub enum EnvironmentEvent {
    SetLocale(LanguageIdentifier),
    SetThemeMode(ThemeMode),
    UseSystemLocale,
}

impl Model for Environment {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            EnvironmentEvent::SetLocale(locale) => {
                self.locale = locale.clone();
            }

            EnvironmentEvent::SetThemeMode(theme_mode) => {
                self.theme_mode = *theme_mode;
                cx.set_theme_mode(self.theme_mode);
                cx.reload_styles().unwrap();
            }

            EnvironmentEvent::UseSystemLocale => {
                self.locale =
                    sys_locale::get_locale().map(|l| l.parse().unwrap()).unwrap_or_default();
            }
        });
    }
}

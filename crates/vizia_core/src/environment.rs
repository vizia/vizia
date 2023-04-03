use crate::{modifiers::TooltipEvent, prelude::Wrapper};
use unic_langid::LanguageIdentifier;
use vizia_derive::Lens;

use crate::{context::EventContext, events::Event, state::Lens, state::Model};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    DarkMode,
    LightMode,
}

#[derive(Lens)]
pub struct Environment {
    pub locale: LanguageIdentifier,
    pub theme_mode: ThemeMode,

    pub tooltips_visible: bool,
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        let locale = sys_locale::get_locale().and_then(|l| l.parse().ok()).unwrap_or_default();

        Self { locale, theme_mode: ThemeMode::DarkMode, tooltips_visible: false }
    }
}

pub enum EnvironmentEvent {
    SetLocale(LanguageIdentifier),
    UseSystemLocale,
    SetThemeMode(ThemeMode),
    ToggleThemeMode,
}

impl Model for Environment {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            EnvironmentEvent::SetLocale(locale) => {
                self.locale = locale.clone();
            }

            EnvironmentEvent::UseSystemLocale => {
                self.locale =
                    sys_locale::get_locale().map(|l| l.parse().unwrap()).unwrap_or_default();
            }

            EnvironmentEvent::SetThemeMode(theme_mode) => {
                self.theme_mode = *theme_mode;
                cx.set_theme_mode(self.theme_mode);
                cx.reload_styles().unwrap();
            }

            EnvironmentEvent::ToggleThemeMode => {
                let theme_mode = match self.theme_mode {
                    ThemeMode::DarkMode => ThemeMode::LightMode,
                    ThemeMode::LightMode => ThemeMode::DarkMode,
                };

                self.theme_mode = theme_mode;
                cx.set_theme_mode(self.theme_mode);
                cx.reload_styles().unwrap();
            }
        });

        event.map(|tooltip_event, _| match tooltip_event {
            TooltipEvent::ShowTooltip => {
                self.tooltips_visible = true;
                println!("show tooltip");
            }
            TooltipEvent::HideTooltip => self.tooltips_visible = false,
        });
    }
}

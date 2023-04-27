//! A model for system specific state which can be accessed by any model or view.
use crate::{model::Model, modifiers::TooltipEvent, prelude::Wrapper};
use unic_langid::LanguageIdentifier;
use vizia_derive::Lens;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    DarkMode,
    LightMode,
}

use crate::{binding::Lens, context::EventContext, events::Event};

/// A model for system specific state which can be accessed by any model or view.
#[derive(Lens)]
pub struct Environment {
    // The locale used for localization.
    pub locale: LanguageIdentifier,
    // The theme mode used when using the built-in theming.
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

/// Events for setting the state in the [Environment].  
pub enum EnvironmentEvent {
    /// Set the locale used for the whole application.
    SetLocale(LanguageIdentifier),
    /// Set the default theme mode.
    SetThemeMode(ThemeMode),
    /// Reset the locale to use the system provided locale.
    UseSystemLocale,
    /// Alternate between dark and light theme modes.
    ToggleThemeMode,
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
            TooltipEvent::ShowTooltip => self.tooltips_visible = true,
            TooltipEvent::HideTooltip => self.tooltips_visible = false,
        });
    }
}

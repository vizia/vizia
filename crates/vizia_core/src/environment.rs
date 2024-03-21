//! A model for system specific state which can be accessed by any model or view.
use crate::prelude::LensValue;
use crate::{
    context::{Context, EmitContext},
    events::{Timer, TimerAction},
    model::Model,
    prelude::Wrapper,
    views::TextEvent,
    window::WindowEvent,
};

use unic_langid::LanguageIdentifier;
use vizia_derive::Lens;
use web_time::Duration;

use crate::{binding::Lens, context::EventContext, events::Event};

/// Different internal theme modes.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    DarkMode,
    #[default]
    LightMode,
}

/// The AppTheme enum defines different types of themes that can be used in the application.
///
/// This includes:
///
/// - `System`: Use the system theme set on the user's OS.
/// - `BuiltIn`: Use one of the built-in Vizia theme modes.
/// - `Custom`: Use a custom theme defined by the user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppTheme {
    /// System theme, if we choose this as our theme vizia
    /// will follow system theme in supported platforms.
    System,
    /// builtin vizia themes
    BuiltIn(ThemeMode),
    /// A custom theme defined by the user to use as internal theme.
    Custom(String),
}

#[derive(Lens)]
pub struct Theme {
    /// The current application theme
    pub app_theme: AppTheme,
    /// The current system theme
    pub sys_theme: Option<ThemeMode>,
}

impl Default for Theme {
    fn default() -> Self {
        Self { app_theme: AppTheme::BuiltIn(ThemeMode::LightMode), sys_theme: None }
    }
}

impl Theme {
    /// Returns the current theme mode based on the configured app theme and system theme.
    ///
    /// If the app theme is set to [`AppTheme::System`], it will return the current system theme if available else `None`.\
    /// If the app theme is [`AppTheme::BuiltIn`], it will return the configured built-in theme mode.\
    /// If the app theme is [`AppTheme::Custom`], it will return `None`.
    /// Get the current internal theme
    pub fn get_current_theme(&self) -> Option<ThemeMode> {
        match self.app_theme {
            AppTheme::System => Some(self.sys_theme?),
            AppTheme::BuiltIn(theme) => Some(theme),
            AppTheme::Custom(_) => None,
        }
    }
}

/// A model for system specific state which can be accessed by any model or view.
#[derive(Lens)]
pub struct Environment {
    /// The locale used for localization.
    pub locale: LanguageIdentifier,
    /// Current application and system theme.
    pub theme: Theme,

    pub(crate) caret_timer: Timer,
}

impl Environment {
    pub fn new(cx: &mut Context) -> Self {
        let locale = sys_locale::get_locale().and_then(|l| l.parse().ok()).unwrap_or_default();
        let caret_timer = cx.add_timer(Duration::from_millis(530), None, |cx, action| {
            if let TimerAction::Tick(_) = action {
                cx.emit(TextEvent::ToggleCaret);
            }
        });
        Self { locale, theme: Theme::default(), caret_timer }
    }
}

/// Events for setting the state in the [Environment].
pub enum EnvironmentEvent {
    /// Set the locale used for the whole application.
    SetLocale(LanguageIdentifier),
    /// Set the default theme mode.
    // TODO: add SetSysTheme event when the winit `set_theme` fixed.
    SetThemeMode(AppTheme),
    /// Reset the locale to use the system provided locale.
    UseSystemLocale,
    /// Alternate between dark and light theme modes.
    ToggleThemeMode,
}

impl Model for Environment {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|event, _| match event {
            EnvironmentEvent::SetLocale(locale) => {
                self.locale = locale;
            }

            EnvironmentEvent::SetThemeMode(theme) => {
                theme.clone_into(&mut self.theme.app_theme);

                match self.theme.app_theme {
                    AppTheme::System => cx.set_theme_mode(self.theme.sys_theme.unwrap_or_default()),
                    AppTheme::BuiltIn(theme) => cx.set_theme_mode(theme),
                    AppTheme::Custom(ref theme) => cx.set_custom_theme(theme.clone()),
                }
                cx.reload_styles().unwrap();
            }

            EnvironmentEvent::UseSystemLocale => {
                self.locale =
                    sys_locale::get_locale().map(|l| l.parse().unwrap()).unwrap_or_default();
            }

            EnvironmentEvent::ToggleThemeMode => {
                let theme_mode = match self.theme.get_current_theme().unwrap_or_default() {
                    ThemeMode::DarkMode => ThemeMode::LightMode,
                    ThemeMode::LightMode => ThemeMode::DarkMode,
                };

                self.theme.app_theme = AppTheme::BuiltIn(theme_mode);

                cx.set_theme_mode(theme_mode);
                cx.reload_styles().unwrap();
            }
        });

        event.map(|event, _| match event {
            WindowEvent::ThemeChanged(theme) => {
                self.theme.sys_theme = Some(*theme);
                if self.theme.app_theme == AppTheme::System {
                    cx.set_theme_mode(*theme);
                    cx.reload_styles().unwrap();
                }
            }
            _ => (),
        })
    }
}

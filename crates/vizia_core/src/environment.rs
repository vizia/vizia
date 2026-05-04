//! A model for system specific state which can be accessed by any model or view.
use crate::prelude::*;

use mundy::Interest;
use mundy::Preferences;
use unic_langid::CharacterDirection;
use unic_langid::LanguageIdentifier;

/// And enum which represents the current built-in theme mode.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    /// Follow the system theme.
    #[default]
    System,
    /// The built-in vizia dark theme.
    DarkMode,
    /// The built-in vizia light theme.
    LightMode,
}

use crate::{context::EventContext, events::Event};

/// A model for system specific state which can be accessed by any model or view.
pub struct Environment {
    /// The locale used for localization.
    pub locale: Signal<LanguageIdentifier>,
    /// The text and layout direction used by the application.
    pub direction: Signal<Direction>,
    /// The maximum interval between two clicks to be recognised as a double-click.
    pub double_click_interval: Duration,
    /// The delay before a tooltip fades in.
    pub tooltip_delay: Duration,
    /// The user's theme preference (may be `System` to follow the OS).
    pub theme_mode: ThemeMode,
    /// The OS-reported system theme (always `DarkMode` or `LightMode`, never `System`).
    pub system_theme_mode: ThemeMode,
    /// The timer used to blink the caret of a textbox.
    pub(crate) caret_timer: Timer,
}

fn direction_from_locale(locale: &LanguageIdentifier) -> Direction {
    match locale.character_direction() {
        CharacterDirection::RTL => Direction::RightToLeft,
        _ => Direction::LeftToRight,
    }
}

pub(crate) fn apply_direction_class(cx: &mut EventContext, direction: Direction) {
    let rtl = direction == Direction::RightToLeft;
    let window_entities = cx
        .windows
        .keys()
        .copied()
        .filter(|window_entity| cx.tree.get_parent(*window_entity) == Some(Entity::root()))
        .collect::<Vec<_>>();

    cx.with_current(Entity::root(), |cx| {
        cx.toggle_class("rtl", rtl);
    });

    for window_entity in window_entities {
        cx.with_current(window_entity, |cx| {
            cx.toggle_class("rtl", rtl);
        });
    }
}

pub(crate) fn apply_theme_class(cx: &mut EventContext, is_dark: bool) {
    let window_entities = cx
        .windows
        .keys()
        .copied()
        .filter(|window_entity| cx.tree.get_parent(*window_entity) == Some(Entity::root()))
        .collect::<Vec<_>>();

    cx.with_current(Entity::root(), |cx| {
        cx.toggle_class("dark", is_dark);
    });

    for window_entity in window_entities {
        cx.with_current(window_entity, |cx| {
            cx.toggle_class("dark", is_dark);
        });
    }
}

fn detect_theme() -> ThemeMode {
    let mundy_prefs = Preferences::once_blocking(Interest::ColorScheme, Duration::from_millis(500));

    if let Some(preferences) = mundy_prefs
        && preferences.color_scheme == mundy::ColorScheme::Dark
    {
        ThemeMode::DarkMode
    } else {
        ThemeMode::LightMode
    }
}

impl Environment {
    pub(crate) fn new(cx: &mut Context) -> Self {
        let locale: LanguageIdentifier =
            sys_locale::get_locale().and_then(|l| l.parse().ok()).unwrap_or_default();
        let caret_timer = cx.add_timer(Duration::from_millis(530), None, |cx, action| {
            if matches!(action, TimerAction::Tick(_)) {
                cx.emit(TextEvent::ToggleCaret);
            }
        });
        let direction = direction_from_locale(&locale);
        Self {
            locale: Signal::new(locale.clone()),
            direction: Signal::new(direction),
            double_click_interval: Duration::from_millis(500),
            tooltip_delay: Duration::from_millis(1500),
            theme_mode: ThemeMode::default(),
            system_theme_mode: detect_theme(),
            caret_timer,
        }
    }

    /// Returns the effective (resolved) theme, substituting the OS theme when the
    /// user preference is [`ThemeMode::System`].
    pub fn effective_theme(&self) -> ThemeMode {
        match self.theme_mode {
            ThemeMode::System => self.system_theme_mode,
            other => other,
        }
    }
}

/// Events for setting the state in the [Environment].
pub enum EnvironmentEvent {
    /// Set the locale used for the whole application.
    SetLocale(LanguageIdentifier),
    /// Set the text and layout direction used by the whole application.
    SetDirection(Direction),
    /// Set the default theme mode.
    // TODO: add SetSysTheme event when the winit `set_theme` fixed.
    SetThemeMode(ThemeMode),
    /// Reset the locale to use the system provided locale.
    UseSystemLocale,
    /// Alternate between dark and light theme modes.
    ToggleThemeMode,
    /// Set the maximum interval between two clicks to be recognised as a double-click.
    SetDoubleClickInterval(Duration),
    /// Set the delay before a tooltip fades in.
    SetTooltipDelay(Duration),
}

impl Model for Environment {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|event, _| match event {
            EnvironmentEvent::SetLocale(locale) => {
                self.locale.set(locale.clone());
                let direction = direction_from_locale(&locale);
                self.direction.set(direction);
                apply_direction_class(cx, direction);
                cx.reload_styles().unwrap();
            }

            EnvironmentEvent::SetDirection(direction) => {
                self.direction.set_if_changed(direction);
                apply_direction_class(cx, direction);
                cx.reload_styles().unwrap();
            }

            EnvironmentEvent::SetThemeMode(theme) => {
                self.theme_mode = theme;
                let is_dark = self.effective_theme() == ThemeMode::DarkMode;
                apply_theme_class(cx, is_dark);
                cx.reload_styles().unwrap();
            }

            EnvironmentEvent::UseSystemLocale => {
                let locale: LanguageIdentifier =
                    sys_locale::get_locale().map(|l| l.parse().unwrap()).unwrap_or_default();
                let direction = direction_from_locale(&locale);
                self.locale.set(locale);
                self.direction.set(direction);
                apply_direction_class(cx, direction);
                cx.reload_styles().unwrap();
            }

            EnvironmentEvent::ToggleThemeMode => {
                let theme_mode = match self.theme_mode {
                    ThemeMode::System => ThemeMode::System,
                    ThemeMode::DarkMode => ThemeMode::LightMode,
                    ThemeMode::LightMode => ThemeMode::DarkMode,
                };

                self.theme_mode = theme_mode;

                let is_dark = self.effective_theme() == ThemeMode::DarkMode;
                apply_theme_class(cx, is_dark);

                cx.reload_styles().unwrap();
            }

            EnvironmentEvent::SetDoubleClickInterval(interval) => {
                self.double_click_interval = interval;
            }

            EnvironmentEvent::SetTooltipDelay(delay) => {
                self.tooltip_delay = delay;
            }
        });

        event.map(|event, _| match event {
            WindowEvent::ThemeChanged(theme) => {
                self.system_theme_mode = *theme;
                if self.theme_mode == ThemeMode::System {
                    let is_dark = self.system_theme_mode == ThemeMode::DarkMode;
                    apply_theme_class(cx, is_dark);
                    cx.reload_styles().unwrap();
                }
            }
            _ => (),
        })
    }
}

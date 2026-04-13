#![allow(dead_code)]
use vizia::prelude::*;

use log::LevelFilter;
use std::error::Error;

pub const CENTER_LAYOUT: &str = r#"
    .container {
        width: 1s;
        height: 1s;
        padding: 20px;
        gap: 20px;
        alignment: top-center;
    }

    :root.primary-blue {
        --primary: #1f6feb;
        --primary-foreground: #ffffff;
    }

    :root.primary-emerald {
        --primary: #0f9d58;
        --primary-foreground: #ffffff;
    }

    :root.primary-crimson {
        --primary: #c62828;
        --primary-foreground: #ffffff;
    }

    :root.primary-amber {
        --primary: #f59e0b;
        --primary-foreground: #1f2937;
    }

    :root.primary-violet {
        --primary: #7c3aed;
        --primary-foreground: #ffffff;
    }
"#;

const PRIMARY_COLOR_CLASSES: [&str; 6] = [
    "default",
    "primary-blue",
    "primary-emerald",
    "primary-crimson",
    "primary-amber",
    "primary-violet",
];

pub struct ControlsData {
    pub disabled: Signal<bool>,
    pub theme_options: Signal<Vec<Localized>>,
    pub selected_theme: Signal<Option<usize>>,
    pub language_options: Signal<Vec<Localized>>,
    pub selected_language: Signal<Option<usize>>,
    pub direction_options: Signal<Vec<&'static str>>,
    pub primary_color_options: Signal<Vec<Localized>>,
    pub selected_primary_color: Signal<Option<usize>>,
}

impl Default for ControlsData {
    fn default() -> Self {
        Self {
            disabled: Signal::new(false),
            theme_options: Signal::new(
                [
                    Localized::new("system-theme"),
                    Localized::new("dark-theme"),
                    Localized::new("light-theme"),
                ]
                .to_vec(),
            ),
            selected_theme: Signal::new(Some(0)),
            language_options: Signal::new(
                [Localized::new("en"), Localized::new("fr"), Localized::new("ar")].to_vec(),
            ),
            selected_language: Signal::new(Some(0)),
            direction_options: Signal::new(["LTR", "RTL"].to_vec()),
            primary_color_options: Signal::new(
                [
                    Localized::new("default"),
                    Localized::new("blue"),
                    Localized::new("emerald"),
                    Localized::new("crimson"),
                    Localized::new("amber"),
                    Localized::new("violet"),
                ]
                .to_vec(),
            ),
            selected_primary_color: Signal::new(Some(0)),
        }
    }
}

pub enum ControlsEvent {
    ToggleDisabled,
    SetThemeMode(usize),
    SetLanguage(usize),
    SetDirection(usize),
    SetPrimaryThemeColor(usize),
}

impl Model for ControlsData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            ControlsEvent::ToggleDisabled => {
                self.disabled.update(|disabled| *disabled ^= true);
            }
            ControlsEvent::SetThemeMode(theme_mode) => {
                self.selected_theme.set(Some(*theme_mode));
                cx.emit(EnvironmentEvent::SetThemeMode(match theme_mode {
                    0 /* system */ => ThemeMode::System,
                    1 /* Dark */ => ThemeMode::DarkMode,
                    2 /* Light */ => ThemeMode::LightMode,
                    _ => unreachable!(),
                }));
            }
            ControlsEvent::SetLanguage(language) => {
                self.selected_language.set(Some(*language));
                cx.emit(EnvironmentEvent::SetLocale(match language {
                    0 /* English */ => langid!("en-US"),
                    1 /* French */ => langid!("fr"),
                    2 /* Arabic */ => langid!("ar"),
                    _ => unreachable!(),
                }));
            }
            ControlsEvent::SetDirection(direction) => {
                cx.emit(EnvironmentEvent::SetDirection(match direction {
                    0 /* LTR */ => Direction::LeftToRight,
                    1 /* RTL */ => Direction::RightToLeft,
                    _ => unreachable!(),
                }));
            }
            ControlsEvent::SetPrimaryThemeColor(color) => {
                self.selected_primary_color.set(Some(*color));

                cx.with_current(Entity::root(), |cx| {
                    for (index, class) in PRIMARY_COLOR_CLASSES.iter().enumerate() {
                        cx.toggle_class(class, index == *color);
                    }
                });
            }
        });
    }
}

pub struct ExamplePage;

impl ExamplePage {
    pub fn vertical(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<'_, Self> {
        //setup_logging().expect("Failed to init logging");

        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to add stylesheet");

        cx.add_translation(
            langid!("en-US"),
            include_str!("../../resources/translations/en-US/helper.ftl"),
        )
        .expect("Failed to add en-US helper translation");

        cx.add_translation(
            langid!("fr"),
            include_str!("../../resources/translations/fr/helper.ftl"),
        )
        .expect("Failed to add fr helper translation");

        cx.add_translation(
            langid!("ar"),
            include_str!("../../resources/translations/ar/helper.ftl"),
        )
        .expect("Failed to add ar helper translation");

        Self.build(cx, |cx| {
            let controls = ControlsData::default();
            let disabled = controls.disabled;
            let theme_options = controls.theme_options;
            let selected_theme = controls.selected_theme;
            let language_options = controls.language_options;
            let selected_language = controls.selected_language;
            let direction_options = controls.direction_options;
            let primary_color_options = controls.primary_color_options;
            let selected_primary_color = controls.selected_primary_color;
            controls.build(cx);
            cx.emit(EnvironmentEvent::SetThemeMode(ThemeMode::System)); // set system theme
            cx.emit(EnvironmentEvent::SetDirection(Direction::LeftToRight));
            cx.emit(ControlsEvent::SetPrimaryThemeColor(0));

            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Switch::new(cx, disabled)
                        .on_toggle(|cx| cx.emit(ControlsEvent::ToggleDisabled))
                        .id("disabled_toggle")
                        .tooltip(|cx| {
                            Tooltip::new(cx, |cx| {
                                Label::new(cx, Localized::new("toggle-disabled"));
                            })
                        });
                    Label::new(cx, Localized::new("toggle-disabled")).describing("disabled_toggle");
                })
                .alignment(Alignment::Center)
                .gap(Pixels(4.0))
                .size(Auto);

                theme_selection_dropdown(cx, theme_options, selected_theme);
                language_selection_dropdown(cx, language_options, selected_language);
                direction_selection_dropdown(cx, direction_options);
                primary_color_selection_dropdown(cx, primary_color_options, selected_primary_color);
            })
            .height(Auto)
            .width(Stretch(1.0))
            .padding(Pixels(10.0))
            .alignment(Alignment::Right)
            .wrap(LayoutWrap::Wrap)
            .gap(Pixels(12.0));

            VStack::new(cx, |cx| {
                (content)(cx);
            })
            .disabled(disabled)
            .class("container");
        })
    }

    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<'_, Self> {
        //setup_logging().expect("Failed to init logging");

        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to add stylesheet");

        cx.add_translation(
            langid!("en-US"),
            include_str!("../../resources/translations/en-US/helper.ftl"),
        )
        .expect("Failed to add en-US helper translation");

        cx.add_translation(
            langid!("fr"),
            include_str!("../../resources/translations/fr/helper.ftl"),
        )
        .expect("Failed to add fr helper translation");

        cx.add_translation(
            langid!("ar"),
            include_str!("../../resources/translations/ar/helper.ftl"),
        )
        .expect("Failed to add ar helper translation");

        Self.build(cx, |cx| {
            let controls = ControlsData::default();
            let disabled = controls.disabled;
            let theme_options = controls.theme_options;
            let selected_theme = controls.selected_theme;
            let language_options = controls.language_options;
            let selected_language = controls.selected_language;
            let direction_options = controls.direction_options;
            let primary_color_options = controls.primary_color_options;
            let selected_primary_color = controls.selected_primary_color;
            controls.build(cx);
            cx.emit(EnvironmentEvent::SetThemeMode(ThemeMode::System)); // set system theme
            cx.emit(ControlsEvent::SetPrimaryThemeColor(0));

            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Switch::new(cx, disabled)
                        .on_toggle(|cx| cx.emit(ControlsEvent::ToggleDisabled));
                    // .tooltip(|cx| {
                    //     Tooltip::new(cx, |cx| {
                    //         Label::new(cx, "Toggle disabled");
                    //     })
                    // });
                    Label::new(cx, Localized::new("toggle-disabled"));
                })
                .alignment(Alignment::Center)
                .gap(Pixels(4.0))
                .size(Auto);

                theme_selection_dropdown(cx, theme_options, selected_theme);
                language_selection_dropdown(cx, language_options, selected_language);
                direction_selection_dropdown(cx, direction_options);
                primary_color_selection_dropdown(cx, primary_color_options, selected_primary_color);
            })
            .height(Auto)
            .width(Stretch(1.0))
            .padding(Pixels(10.0))
            .alignment(Alignment::Right)
            .wrap(LayoutWrap::Wrap)
            .gap(Pixels(12.0));

            HStack::new(cx, |cx| {
                let _e = HStack::new(cx, |cx| {
                    (content)(cx);
                })
                .disabled(disabled)
                .class("container")
                .entity();
            });
        })
    }
}

impl View for ExamplePage {}

fn theme_selection_dropdown(
    cx: &mut Context,
    theme_options: Signal<Vec<Localized>>,
    selected_theme: Signal<Option<usize>>,
) {
    Select::new(cx, theme_options, selected_theme, true)
        .min_selected(1)
        .on_select(|cx, index| cx.emit(ControlsEvent::SetThemeMode(index)))
        .width(Pixels(100.0))
        .tooltip(|cx| {
            Tooltip::new(cx, |cx| {
                Label::new(cx, "Select Theme Mode");
            })
        });
}

fn primary_color_selection_dropdown(
    cx: &mut Context,
    color_options: Signal<Vec<Localized>>,
    selected_color: Signal<Option<usize>>,
) {
    Select::new(cx, color_options, selected_color, true)
        .min_selected(1)
        .on_select(|cx, index| cx.emit(ControlsEvent::SetPrimaryThemeColor(index)))
        .width(Pixels(120.0))
        .tooltip(|cx| {
            Tooltip::new(cx, |cx| {
                Label::new(cx, "Select Primary Color");
            })
        });
}

fn direction_selection_dropdown(cx: &mut Context, direction_options: Signal<Vec<&'static str>>) {
    let selected_direction = cx.environment().direction.map(|direction| match direction {
        Direction::LeftToRight => Some(0),
        Direction::RightToLeft => Some(1),
    });
    Select::new(cx, direction_options, selected_direction, true)
        .min_selected(1)
        .on_select(|cx, index| cx.emit(ControlsEvent::SetDirection(index)))
        .width(Pixels(100.0))
        .tooltip(|cx| {
            Tooltip::new(cx, |cx| {
                Label::new(cx, "Select Direction");
            })
        });
}

fn language_selection_dropdown(
    cx: &mut Context,
    language_options: Signal<Vec<Localized>>,
    selected_language: Signal<Option<usize>>,
) {
    Select::new(cx, language_options, selected_language, true)
        .min_selected(1)
        .on_select(|cx, index| cx.emit(ControlsEvent::SetLanguage(index)))
        .width(Pixels(110.0))
        .tooltip(|cx| {
            Tooltip::new(cx, |cx| {
                Label::new(cx, "Select Language");
            })
        });
}

pub fn setup_logging() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    const MAIN_LOG_LEVEL: LevelFilter = LevelFilter::Debug;
    #[cfg(not(debug_assertions))]
    const MAIN_LOG_LEVEL: LevelFilter = LevelFilter::Info;

    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(move |out, message, record| {
            out.finish(format_args!("[{}][{}] {}", record.target(), record.level(), message))
        })
        // Add blanket level filter
        .level(MAIN_LOG_LEVEL)
        .level_for("cosmic_text::buffer", LevelFilter::Warn)
        .level_for("selectors::matching", LevelFilter::Warn)
        .level_for("cosmic_text::font::system::std", LevelFilter::Warn)
        // Output to stdout
        .chain(std::io::stdout())
        // Apply globally
        .apply()?;

    Ok(())
}

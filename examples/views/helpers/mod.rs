#![allow(dead_code)]
use vizia::{
    icons::{ICON_MOON, ICON_SUN},
    prelude::*,
};

use log::LevelFilter;
use std::error::Error;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref THEME_MODES: Vec<&'static str> = vec!["Light", "Dark"];
}

pub const CENTER_LAYOUT: &str = r#"
    .container {
        width: 1s;
        height: 1s;
        child-space: 1s;
        col-between: 20px;
        row-between: 20px;
    }
"#;

#[derive(Lens)]
pub struct ControlsData {
    pub disabled: bool,
}

pub enum ControlsEvent {
    ToggleDisabled,
}

impl Model for ControlsData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            ControlsEvent::ToggleDisabled => {
                self.disabled ^= true;
            }
        });
    }
}

pub struct ExamplePage {}

impl ExamplePage {
    pub fn vertical(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        setup_logging().expect("Failed to init logging");

        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to add stylesheet");

        Self {}.build(cx, |cx| {
            ControlsData { disabled: false }.build(cx);

            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Switch::new(cx, ControlsData::disabled)
                        .on_toggle(|cx| cx.emit(ControlsEvent::ToggleDisabled));
                    Label::new(cx, "Toggle Disabled");
                })
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(5.0))
                .top(Stretch(1.0))
                .bottom(Stretch(1.0))
                .size(Auto);

                Button::new(
                    cx,
                    |ex| ex.emit(EnvironmentEvent::ToggleThemeMode),
                    |cx| {
                        Label::new(
                            cx,
                            Environment::theme_mode.map(|mode| {
                                if *mode == ThemeMode::DarkMode {
                                    ICON_SUN
                                } else {
                                    ICON_MOON
                                }
                            }),
                        )
                        .class("icon")
                    },
                )
                .class("icon")
                .tooltip(|cx| {
                    Label::new(cx, "Toggle Dark/Light Mode");
                });
            })
            .height(Auto)
            .width(Stretch(1.0))
            .child_top(Pixels(10.0))
            .child_bottom(Pixels(10.0))
            .child_left(Stretch(1.0))
            .child_right(Pixels(20.0))
            .top(Pixels(0.0))
            .left(Pixels(0.0))
            .right(Pixels(0.0))
            .col_between(Pixels(20.0));

            VStack::new(cx, |cx| {
                (content)(cx);
            })
            .disabled(ControlsData::disabled)
            .class("container");
        })
    }
    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        setup_logging().expect("Failed to init logging");

        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to add stylesheet");

        Self {}.build(cx, |cx| {
            ControlsData { disabled: false }.build(cx);

            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Switch::new(cx, ControlsData::disabled)
                        .on_toggle(|cx| cx.emit(ControlsEvent::ToggleDisabled));
                    Label::new(cx, "Toggle Disabled");
                })
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(5.0))
                .top(Stretch(1.0))
                .bottom(Stretch(1.0))
                .size(Auto);

                Button::new(
                    cx,
                    |ex| ex.emit(EnvironmentEvent::ToggleThemeMode),
                    |cx| {
                        Label::new(
                            cx,
                            Environment::theme_mode.map(|mode| {
                                if *mode == ThemeMode::DarkMode {
                                    ICON_SUN
                                } else {
                                    ICON_MOON
                                }
                            }),
                        )
                        .class("icon")
                    },
                )
                .class("icon")
                .tooltip(|cx| {
                    Label::new(cx, "Toggle dark/light mode");
                });
            })
            .height(Auto)
            .width(Stretch(1.0))
            .child_top(Pixels(10.0))
            .child_bottom(Pixels(10.0))
            .child_left(Stretch(1.0))
            .child_right(Pixels(20.0))
            .top(Pixels(0.0))
            .left(Pixels(0.0))
            .right(Pixels(0.0))
            .col_between(Pixels(20.0));

            HStack::new(cx, |cx| {
                (content)(cx);
            })
            .disabled(ControlsData::disabled)
            .class("container");
        })
    }
}

impl View for ExamplePage {}

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

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
"#;

pub struct ExamplePage {
    theme_options: Signal<Vec<&'static str>>,
    selected_theme: Signal<usize>,
}

impl ExamplePage {
    pub fn vertical(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<'_, Self> {
        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to add stylesheet");

        let disabled = cx.state(false);
        let theme_options = cx.state(vec!["System", "Dark", "Light"]);
        let selected_theme = cx.state(0usize);
        let align_left = cx.state(Alignment::Left);
        let align_right = cx.state(Alignment::Right);
        let stretch_one = cx.state(Stretch(1.0));
        let auto = cx.state(Auto);
        let gap_5 = cx.state(Pixels(5.0));
        let gap_20 = cx.state(Pixels(20.0));
        let padding_10 = cx.state(Pixels(10.0));
        let picklist_width = cx.state(Pixels(100.0));

        cx.emit(EnvironmentEvent::SetThemeMode(AppTheme::System));

        Self { theme_options, selected_theme }.build(cx, move |cx| {
            HStack::new(cx, move |cx| {
                HStack::new(cx, |cx| {
                    Switch::new(cx, disabled).two_way();
                    Label::new(cx, "Toggle Disabled");
                })
                .alignment(align_left)
                .horizontal_gap(gap_5)
                .top(stretch_one)
                .bottom(stretch_one)
                .size(auto);

                PickList::new(cx, theme_options, selected_theme, true)
                    .on_select(move |cx, index| {
                        selected_theme.set(cx, index);
                        cx.emit(EnvironmentEvent::SetThemeMode(match index {
                            0 => AppTheme::System,
                            1 => AppTheme::BuiltIn(ThemeMode::DarkMode),
                            2 => AppTheme::BuiltIn(ThemeMode::LightMode),
                            _ => AppTheme::System,
                        }));
                    })
                    .width(picklist_width);
            })
            .height(auto)
            .width(stretch_one)
            .padding(padding_10)
            .alignment(align_right)
            .horizontal_gap(gap_20);

            VStack::new(cx, |cx| {
                (content)(cx);
            })
            .disabled(disabled)
            .class("container");
        })
    }

    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<'_, Self> {
        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to add stylesheet");

        let disabled = cx.state(false);
        let theme_options = cx.state(vec!["System", "Dark", "Light"]);
        let selected_theme = cx.state(0usize);
        let align_center = cx.state(Alignment::Center);
        let align_right = cx.state(Alignment::Right);
        let stretch_one = cx.state(Stretch(1.0));
        let auto = cx.state(Auto);
        let gap_5 = cx.state(Pixels(5.0));
        let gap_20 = cx.state(Pixels(20.0));
        let padding_10 = cx.state(Pixels(10.0));
        let picklist_width = cx.state(Pixels(100.0));

        cx.emit(EnvironmentEvent::SetThemeMode(AppTheme::System));

        Self { theme_options, selected_theme }.build(cx, move |cx| {
            HStack::new(cx, move |cx| {
                HStack::new(cx, |cx| {
                    Switch::new(cx, disabled).two_way();
                    Label::new(cx, "Toggle Disabled");
                })
                .alignment(align_center)
                .horizontal_gap(gap_5)
                .top(stretch_one)
                .bottom(stretch_one)
                .size(auto);

                PickList::new(cx, theme_options, selected_theme, true)
                    .on_select(move |cx, index| {
                        selected_theme.set(cx, index);
                        cx.emit(EnvironmentEvent::SetThemeMode(match index {
                            0 => AppTheme::System,
                            1 => AppTheme::BuiltIn(ThemeMode::DarkMode),
                            2 => AppTheme::BuiltIn(ThemeMode::LightMode),
                            _ => AppTheme::System,
                        }));
                    })
                    .width(picklist_width);
            })
            .height(auto)
            .width(stretch_one)
            .padding(padding_10)
            .alignment(align_right)
            .horizontal_gap(gap_20);

            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    (content)(cx);
                })
                .disabled(disabled)
                .class("container");
            });
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
        .format(move |out, message, record| {
            out.finish(format_args!("[{}][{}] {}", record.target(), record.level(), message))
        })
        .level(MAIN_LOG_LEVEL)
        .level_for("cosmic_text::buffer", LevelFilter::Warn)
        .level_for("selectors::matching", LevelFilter::Warn)
        .level_for("cosmic_text::font::system::std", LevelFilter::Warn)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}

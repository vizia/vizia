#![allow(dead_code)]
use vizia::prelude::*;

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
    pub theme_options: Vec<&'static str>,
    pub selected_theme: usize,
}

impl Default for ControlsData {
    fn default() -> Self {
        Self { disabled: false, theme_options: vec!["System", "Dark", "Light"], selected_theme: 0 }
    }
}

pub enum ControlsEvent {
    ToggleDisabled,
    SetThemeMode(usize),
}

impl Model for ControlsData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            ControlsEvent::ToggleDisabled => {
                self.disabled ^= true;
            }
            ControlsEvent::SetThemeMode(theme_mode) => {
                self.selected_theme = *theme_mode;
                cx.emit(EnvironmentEvent::SetThemeMode(match theme_mode {
                    0 /* system */ => AppTheme::System,
                    1 /* Dark */ => AppTheme::BuiltIn(ThemeMode::DarkMode),
                    2 /* Light */ => AppTheme::BuiltIn(ThemeMode::LightMode),
                    _ => unreachable!(),
                }));
            }
        });
    }
}

pub struct ExamplePage;

impl ExamplePage {
    pub fn vertical(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to add stylesheet");

        Self.build(cx, |cx| {
            ControlsData::default().build(cx);

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

                theme_selection_dropdown(cx);
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
        // setup_logging().expect("Failed to setup logging");

        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to add stylesheet");

        Self.build(cx, |cx| {
            ControlsData::default().build(cx);

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

                theme_selection_dropdown(cx);
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

fn theme_selection_dropdown(cx: &mut Context) {
    PickList::new(cx, ControlsData::theme_options, ControlsData::selected_theme, true)
        .on_select(|cx, index| cx.emit(ControlsEvent::SetThemeMode(index)))
        .width(Pixels(85.0))
        .tooltip(|cx| {
            Label::new(cx, "Select Theme Mode");
        });
}

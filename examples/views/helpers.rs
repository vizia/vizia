#![allow(dead_code)]
use vizia::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref THEME_MODES: Vec<&'static str> = vec!["Light", "Dark"];
}

pub const CENTER_LAYOUT: &str = "crates/vizia_core/resources/themes/center_layout.css";

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

pub fn view_controls(cx: &mut Context) {
    cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to find stylesheet");

    ControlsData { disabled: false }.build(cx);

    HStack::new(cx, |cx| {
        PickList::new(
            cx,
            StaticLens::new(THEME_MODES.as_ref()),
            Environment::theme_mode.map(|mode| if *mode == ThemeMode::LightMode { 0 } else { 1 }),
            true,
        )
        .on_select(|cx, idx| {
            if idx == 0 {
                cx.emit(EnvironmentEvent::SetThemeMode(ThemeMode::LightMode));
            } else {
                cx.emit(EnvironmentEvent::SetThemeMode(ThemeMode::DarkMode));
            }
        })
        .width(Pixels(100.0));

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
    })
    .height(Auto)
    .child_top(Pixels(10.0))
    .child_bottom(Pixels(10.0))
    .child_left(Stretch(1.0))
    .child_right(Pixels(20.0))
    .top(Pixels(0.0))
    .left(Pixels(0.0))
    .right(Pixels(0.0))
    .col_between(Pixels(20.0));
}

pub fn theme_selector(cx: &mut Context) {
    cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to find stylesheet");

    ControlsData { disabled: false }.build(cx);

    HStack::new(cx, |cx| {
        PickList::new(
            cx,
            StaticLens::new(THEME_MODES.as_ref()),
            Environment::theme_mode.map(|mode| if *mode == ThemeMode::LightMode { 0 } else { 1 }),
            true,
        )
        .on_select(|cx, idx| {
            if idx == 0 {
                cx.emit(EnvironmentEvent::SetThemeMode(ThemeMode::LightMode));
            } else {
                cx.emit(EnvironmentEvent::SetThemeMode(ThemeMode::DarkMode));
            }
        })
        .width(Pixels(100.0));
    })
    .height(Auto)
    .child_top(Pixels(10.0))
    .child_bottom(Pixels(10.0))
    .child_left(Stretch(1.0))
    .child_right(Pixels(20.0))
    .top(Pixels(0.0))
    .left(Pixels(0.0))
    .right(Pixels(0.0))
    .col_between(Pixels(20.0));
}

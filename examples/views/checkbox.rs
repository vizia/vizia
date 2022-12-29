use vizia::fonts::unicode_names::CANCEL;
use vizia::prelude::*;

#[derive(Debug, Lens)]
pub struct AppData {
    pub option1: bool,
    pub option2: bool,
}

#[derive(Debug)]
pub enum AppEvent {
    ToggleOption1,
    ToggleOption2,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleOption1 => {
                self.option1 ^= true;
            }

            AppEvent::ToggleOption2 => {
                self.option2 ^= true;
            }
        });
    }
}

const CENTER_LAYOUT: &str = "crates/vizia_core/resources/themes/center_layout.css";
#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        AppData { option1: true, option2: false }.build(cx);

        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to find stylesheet");
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Basic checkboxes");

                HStack::new(cx, |cx| {
                    // CBasic Checkboxes
                    Checkbox::new(cx, AppData::option1)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleOption1));
                    Checkbox::new(cx, AppData::option2)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleOption2));
                    Checkbox::new(cx, AppData::option2)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleOption2))
                        .disabled(true);
                    Checkbox::new(cx, AppData::option1)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleOption1))
                        .disabled(true);
                })
                .col_between(Pixels(4.0));

                Label::new(cx, "Checkbox with label").top(Pixels(20.0));

                // Checkboxes with label
                HStack::new(cx, |cx| {
                    Checkbox::new(cx, AppData::option1)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleOption1))
                        .id("checkbox_1");
                    Label::new(cx, "Checkbox").describing("checkbox_1");
                })
                .size(Auto)
                .col_between(Pixels(5.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0));

                HStack::new(cx, |cx| {
                    Checkbox::new(cx, AppData::option2)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleOption2))
                        .id("checkbox_2");
                    Label::new(cx, "Disabled").describing("checkbox_2");
                })
                .disabled(true)
                .size(Auto)
                .col_between(Pixels(5.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0));

                Label::new(cx, "Checkbox with custom icon and label")
                    .top(Pixels(20.0))
                    .top(Pixels(20.0));

                HStack::new(cx, |cx| {
                    Checkbox::new(cx, AppData::option1)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleOption1))
                        .text(AppData::option1.map(|flag| if *flag { CANCEL } else { "" }))
                        .id("checkbox_3");
                    Label::new(cx, "Custom").describing("checkbox_3");
                })
                .size(Auto)
                .col_between(Pixels(5.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0));
            })
            .class("container");
        })
        .class("main");
    })
    .ignore_default_theme()
    .title("Checkbox")
    .inner_size((300, 250))
    .run();
}

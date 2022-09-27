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

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        AppData { option1: true, option2: false }.build(cx);

        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Basic Switches");

                HStack::new(cx, |cx| {
                    Switch::new(cx, AppData::option1)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleOption1))
                        .id("Switch_2");
                    Label::new(cx, "Switch 1").describing("Switch_1");
                })
                .size(Auto)
                .col_between(Pixels(5.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0));

                HStack::new(cx, |cx| {
                    Switch::new(cx, AppData::option2)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleOption2))
                        .id("Switch_2");
                    Label::new(cx, "Switch 2").describing("Switch_2");
                })
                .size(Auto)
                .col_between(Pixels(5.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0));

                HStack::new(cx, |cx| {
                    Switch::new(cx, AppData::option2)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleOption2))
                        .id("Switch_2");
                    Label::new(cx, "Switch Disabled").describing("Switch_D");
                })
                .disabled(true)
                .size(Auto)
                .col_between(Pixels(5.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0));
            })
            .size(Auto)
            .row_between(Pixels(10.0))
            .space(Stretch(1.0));
        })
        .class("main")
        .width(Units::Stretch(1.0))
        .height(Units::Stretch(1.0));
    })
    //.ignore_default_theme()
    .ignore_default_theme()
    .title("Switch")
    .run();
}

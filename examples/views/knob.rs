use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    value: f32,
}

#[derive(Debug)]
pub enum AppEvent {
    SetValue(f32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetValue(value) => {
                self.value = *value;
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
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        AppData { value: 0.2 }.build(cx);

        HStack::new(cx, |cx| {
            Knob::new(cx, 0.5, AppData::value, false).on_changing(|cx, val| {
                cx.emit(AppEvent::SetValue(val));
            });
            Knob::new(cx, 0.5, AppData::value, true).on_changing(|cx, val| {
                cx.emit(AppEvent::SetValue(val));
            });
        })
        .class("container");
    })
    .ignore_default_theme()
    .title("Knob")
    .run();
}

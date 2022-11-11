use chrono::{NaiveDate, Utc};
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppState {
    color: Color,
}

pub enum AppEvent {
    SetColor(Color),
}

impl Model for AppState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetColor(color) => {
                println!("Color changed to: {:?}", color);
                self.color = *color;
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
        AppState { color: Color::rgb(200, 100, 50) }.build(cx);

        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        ColorPicker::new(cx, AppState::color)
            .on_change(|cx, color| cx.emit(AppEvent::SetColor(color)));
        //.on_select(|cx, date| cx.emit(AppEvent::SetDate(date)));
    })
    .ignore_default_theme()
    .title("Datepicker")
    .run();
}

use chrono::{NaiveDate, Utc};
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppState {
    date: NaiveDate,
}

pub enum AppEvent {
    SetDate(NaiveDate),
}

impl Model for AppState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetDate(date) => {
                println!("Date changed to: {}", date);
                self.date = *date;
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
        AppState { date: Utc::now().date_naive() }.build(cx);

        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to find stylesheet");
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        Datepicker::new(cx, AppState::date).on_select(|cx, date| cx.emit(AppEvent::SetDate(date)));
    })
    .ignore_default_theme()
    .title("Datepicker")
    .run();
}

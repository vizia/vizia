use chrono::{NaiveTime, Utc};
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppState {
    time: NaiveTime,
}

pub enum AppEvent {
    SetTime(NaiveTime),
}

impl Model for AppState {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetTime(time) => {
                println!("Set time to: {}", time);
                self.time = *time;
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
        AppState { time: Utc::now().naive_utc().time() }.build(cx);

        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        HStack::new(cx, |cx| {
            // Timepicker::new(cx, AppData::timepicker_value)
            //     .on_changing(|cx, day_time| {
            //         cx.emit(AppDataSetter::TimepickerValue(day_time.clone()));
            //     })
            //     .on_ok(|_| println!("Ok!"));
            RadialTimepicker::new(cx, AppState::time)
                .on_changing(|cx, time| cx.emit(AppEvent::SetTime(time)));
        })
        .class("container");
    })
    .ignore_default_theme()
    .title("Spinbox")
    .run();
}

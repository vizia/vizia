mod helpers;
use chrono::{NaiveTime, Utc};
use helpers::*;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppState {
    time: NaiveTime,
}

pub enum AppEvent {
    SetTime(NaiveTime),
}

impl Model for AppState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetTime(time) => {
                self.time = *time;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppState { time: Utc::now().naive_utc().time() }.build(cx);

        ExamplePage::new(cx, |cx| {
            // Timepicker::new(cx, AppState::time).on_change(|cx, time| {
            //     cx.emit(AppEvent::SetTime(time));
            // });
            // DigitalTimepicker::new(cx, AppState::time).on_change(|cx, time| {
            //     cx.emit(AppEvent::SetTime(time));
            // });
            AnalogTimepicker::new(cx, AppState::time)
                .on_change(|cx, time| cx.emit(AppEvent::SetTime(time)));
        });
    })
    .title("Timepicker")
    .run();
}

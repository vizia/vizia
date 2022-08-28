use chrono::{Date, Utc};
use vizia::prelude::*;

#[derive(Clone, Data, Lens)]
struct AppState {
    date: Date<Utc>,
}

fn main() {
    Application::new(|cx| {
        AppState { date: Utc::today() }.build(cx);

        HStack::new(cx, |cx| {
            // Basic Datepicker
            Datepicker::new(cx, AppState::date);
        })
        .size(Auto)
        .space(Stretch(1.0))
        .col_between(Pixels(10.0));
    })
    .title("Datepicker")
    .run();
}

impl Model for AppState {}

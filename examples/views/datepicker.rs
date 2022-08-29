use chrono::{Date, Utc};
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppState {
    date: Date<Utc>,
}

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        AppState { date: Utc::today() }.build(cx);

        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Datepicker::new(cx);
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
    .title("Datepicker")
    .run();
}

impl Model for AppState {}

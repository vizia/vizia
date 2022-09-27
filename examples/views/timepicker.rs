use vizia::prelude::*;

#[derive(Lens, Setter, Model)]
pub struct AppData {
    timepicker_value: DayTime,
}

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        AppData { timepicker_value: DayTime { hour: 9, minutes: 30, zone: AMOrPM::AM } }.build(cx);

        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Timepicker::new(cx, AppData::timepicker_value)
                    .on_changing(|cx, day_time| {
                        cx.emit(AppDataSetter::TimepickerValue(day_time.clone()));
                    })
                    .on_ok(|_| println!("Ok!"));
            })
            .class("container");
        })
        .class("main");
    })
    .ignore_default_theme()
    .title("Spinbox")
    .run();
}

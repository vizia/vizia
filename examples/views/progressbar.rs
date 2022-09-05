use vizia::fonts::unicode_names::CHECK;
use vizia::prelude::*;

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

enum AppEvent {
    SetValue(f32),
}

#[derive(Lens)]
struct AppState {
    progress_bar: f32,
}

fn main() {
    Application::new(|cx| {
        AppState { progress_bar: 0.1 }.build(cx);

        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Slider::new(cx, AppState::progress_bar)
                    .on_changing(move |cx, val| cx.emit(AppEvent::SetValue(val)));

                ProgressBar::new(cx, AppState::progress_bar).width(Units::Stretch(1.0));
            })
            .class("container");
        })
        .class("main");
    })
    .ignore_default_theme()
    .title("Progress Bar")
    .run();
}

impl Model for AppState {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            AppEvent::SetValue(val) => {
                self.progress_bar = *val;
            }
        })
    }
}

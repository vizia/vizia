use vizia::fonts::unicode_names::CHECK;
use vizia::prelude::*;

#[derive(Clone, Data, Lens)]
struct AppState {
    spinbox_value: i64,
}

#[derive(Clone)]
enum AppEvent {
    Increment,
    Decrement,
}

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        AppState { spinbox_value: 99 }.build(cx);

        cx.add_stylesheet(LIGHT_THEME).expect("Failed to find stylesheet");

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Spinbox::new(cx, AppState::spinbox_value, SpinboxKind::Horizontal)
                    .on_increment(|ex| ex.emit(AppEvent::Increment))
                    .on_decrement(|ex| ex.emit(AppEvent::Decrement));
                Spinbox::new(cx, AppState::spinbox_value, SpinboxKind::Vertical)
                    .on_increment(|ex| ex.emit(AppEvent::Increment))
                    .on_decrement(|ex| ex.emit(AppEvent::Decrement));
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
    .title("Spinbox")
    .run();
}

impl Model for AppState {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            AppEvent::Decrement => {
                self.spinbox_value -= 1;
            }

            AppEvent::Increment => {
                self.spinbox_value += 1;
            }
        })
    }
}

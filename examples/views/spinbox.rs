use vizia::prelude::*;

#[derive(Clone, Data, Lens)]
struct AppState {
    spinbox_value_1: i64,
    spinbox_value_2_index: usize,
    spinbox_value_2: String,
}

const spinbox_values_2: [&str; 3] = ["One", "Two", "Three"];

#[derive(Clone)]
enum AppEvent {
    Increment_1,
    Decrement_1,

    Increment_2,
    Decrement_2,
}

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        AppState {
            spinbox_value_1: 99,
            spinbox_value_2: spinbox_values_2[0].to_string(),
            spinbox_value_2_index: 0,
        }
        .build(cx);

        cx.add_stylesheet(LIGHT_THEME).expect("Failed to find stylesheet");

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Spinbox::new(cx, AppState::spinbox_value_1, SpinboxKind::Horizontal)
                    .on_increment(|ex| ex.emit(AppEvent::Increment_1))
                    .on_decrement(|ex| ex.emit(AppEvent::Decrement_1));
                // Spinbox::new(cx, AppState::spinbox_value_1, SpinboxKind::Vertical)
                //     .on_increment(|ex| ex.emit(AppEvent::Increment_1))
                //     .on_decrement(|ex| ex.emit(AppEvent::Decrement_1));
                Spinbox::new(cx, AppState::spinbox_value_2, SpinboxKind::Horizontal)
                    .on_increment(|ex| ex.emit(AppEvent::Increment_2))
                    .on_decrement(|ex| ex.emit(AppEvent::Decrement_2));
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
            AppEvent::Decrement_1 => {
                println!("d1");
                self.spinbox_value_1 -= 1;
            }

            AppEvent::Increment_1 => {
                println!("i1");
                self.spinbox_value_1 += 1;
            }

            AppEvent::Decrement_2 => {
                println!("d2");
                if self.spinbox_value_2_index == 0 {
                    self.spinbox_value_2_index = spinbox_values_2.len();
                }
                self.spinbox_value_2_index -= 1;
                self.spinbox_value_2_index %= spinbox_values_2.len();

                self.spinbox_value_2 = spinbox_values_2[self.spinbox_value_2_index].to_string();
            }

            AppEvent::Increment_2 => {
                println!("i2");
                self.spinbox_value_2_index += 1;
                self.spinbox_value_2_index %= spinbox_values_2.len();

                self.spinbox_value_2 = spinbox_values_2[self.spinbox_value_2_index].to_string();
            }
        })
    }
}

mod helpers;
use helpers::*;
use std::fmt::Display;

use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppState {
    spinbox_value_1: i64,
    spinbox_value_2: usize,
    spinbox_value_3_choices: Vec<SpinboxValues>,
    spinbox_value_3: usize,
}

#[derive(Clone, PartialEq, Copy, Eq, Data)]
enum SpinboxValues {
    One,
    Two,
    Three,
}

impl SpinboxValues {
    pub fn values() -> Vec<Self> {
        vec![SpinboxValues::One, SpinboxValues::Two, SpinboxValues::Three]
    }
}

impl Display for SpinboxValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SpinboxValues::One => "one",
            SpinboxValues::Two => "two",
            SpinboxValues::Three => "three",
        })
    }
}

#[derive(Clone)]
enum AppEvent {
    Increment1,
    Decrement1,

    Increment2,
    Decrement2,
    Set2(String),

    Increment3,
    Decrement3,
    Set3(usize),
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppState {
            spinbox_value_1: 99,
            spinbox_value_2: 0,
            spinbox_value_3: 0,
            spinbox_value_3_choices: SpinboxValues::values(),
        }
        .build(cx);

        ExamplePage::new(cx, |cx| {
            Spinbox::new(cx, AppState::spinbox_value_1)
                .width(Pixels(100.0))
                .on_increment(|ex| ex.emit(AppEvent::Increment1))
                .on_decrement(|ex| ex.emit(AppEvent::Decrement1));

            Spinbox::custom(cx, |cx| {
                Textbox::new(cx, AppState::spinbox_value_2)
                    .on_edit(|ex, v| ex.emit(AppEvent::Set2(v)))
            })
            .icons(SpinboxIcons::PlusMinus)
            .width(Pixels(100.0))
            .on_increment(|ex| ex.emit(AppEvent::Increment2))
            .on_decrement(|ex| ex.emit(AppEvent::Decrement2));

            Spinbox::custom(cx, |cx| {
                PickList::new(
                    cx,
                    AppState::spinbox_value_3_choices,
                    AppState::spinbox_value_3,
                    false,
                )
                .on_select(|cx, val| cx.emit(AppEvent::Set3(val)))
            })
            .on_increment(|ex| ex.emit(AppEvent::Increment3))
            .on_decrement(|ex| ex.emit(AppEvent::Decrement3));
        });
    })
    .title("Spinbox")
    .run()
}

impl Model for AppState {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            AppEvent::Decrement1 => {
                self.spinbox_value_1 -= 1;
            }

            AppEvent::Increment1 => {
                self.spinbox_value_1 += 1;
            }

            AppEvent::Decrement2 => {
                if self.spinbox_value_2 != 0 {
                    self.spinbox_value_2 -= 1;
                }
            }

            AppEvent::Increment2 => {
                self.spinbox_value_2 += 1;
            }

            AppEvent::Set2(v) => {
                self.spinbox_value_2 = match v.parse::<usize>() {
                    Ok(number) => number,
                    Err(_) => self.spinbox_value_2,
                }
            }

            AppEvent::Increment3 => {
                self.spinbox_value_3 = (self.spinbox_value_3 + 1) % 3;
            }

            AppEvent::Decrement3 => {
                let mut index = self.spinbox_value_3;
                if index == 0 {
                    index = 3
                }
                self.spinbox_value_3 = index - 1;
            }

            AppEvent::Set3(v) => self.spinbox_value_3 = *v,
        })
    }
}

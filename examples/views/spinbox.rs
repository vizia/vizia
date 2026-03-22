mod helpers;
use helpers::*;
use std::fmt::Display;

use vizia::prelude::*;

struct AppState {
    spinbox_value_1: Signal<i64>,
    spinbox_value_2: Signal<usize>,
    spinbox_value_3_choices: Signal<Vec<Signal<SpinboxValues>>>,
    spinbox_value_3: Signal<usize>,
}

#[derive(Clone, PartialEq, Copy, Eq)]
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
        let spinbox_value_1 = Signal::new(99);
        let spinbox_value_2 = Signal::new(0usize);
        let spinbox_value_3 = Signal::new(0usize);
        let spinbox_value_3_choices =
            Signal::new(SpinboxValues::values().into_iter().map(Signal::new).collect::<Vec<_>>());

        AppState { spinbox_value_1, spinbox_value_2, spinbox_value_3, spinbox_value_3_choices }
            .build(cx);

        ExamplePage::new(cx, |cx| {
            Spinbox::new(cx, spinbox_value_1)
                .icons(SpinboxIcons::PlusMinus)
                .width(Pixels(100.0))
                .on_increment(|ex| ex.emit(AppEvent::Increment1))
                .on_decrement(|ex| ex.emit(AppEvent::Decrement1));

            Spinbox::custom(cx, |cx| {
                Textbox::new(cx, spinbox_value_2).on_edit(|ex, v| ex.emit(AppEvent::Set2(v)))
            })
            .icons(SpinboxIcons::PlusMinus)
            .width(Pixels(100.0))
            .on_increment(|ex| ex.emit(AppEvent::Increment2))
            .on_decrement(|ex| ex.emit(AppEvent::Decrement2));

            Spinbox::custom(cx, |cx| {
                PickList::new(cx, spinbox_value_3_choices, spinbox_value_3, false)
                    .on_select(|cx, val| cx.emit(AppEvent::Set3(val)))
            })
            .width(Pixels(100.0))
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
                self.spinbox_value_1.update(|value| *value -= 1);
            }

            AppEvent::Increment1 => {
                self.spinbox_value_1.update(|value| *value += 1);
            }

            AppEvent::Decrement2 => {
                self.spinbox_value_2.update(|value| {
                    if *value != 0 {
                        *value -= 1;
                    }
                });
            }

            AppEvent::Increment2 => {
                self.spinbox_value_2.update(|value| *value += 1);
            }

            AppEvent::Set2(v) => {
                let current = self.spinbox_value_2.get();
                let parsed = match v.parse::<usize>() {
                    Ok(number) => number,
                    Err(_) => current,
                };
                self.spinbox_value_2.set(parsed);
            }

            AppEvent::Increment3 => {
                let len = self.spinbox_value_3_choices.get().len();
                if len > 0 {
                    self.spinbox_value_3.update(|index| *index = (*index + 1) % len);
                }
            }

            AppEvent::Decrement3 => {
                let len = self.spinbox_value_3_choices.get().len();
                if len > 0 {
                    self.spinbox_value_3.update(|index| {
                        if *index == 0 {
                            *index = len;
                        }
                        *index -= 1;
                    });
                }
            }

            AppEvent::Set3(v) => self.spinbox_value_3.set(*v),
        })
    }
}

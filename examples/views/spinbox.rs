mod helpers;
use helpers::*;
use std::fmt::Display;

use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppState {
    spinbox_value_1: i64,
    spinbox_value_2: usize,
    spinbox_value_3_choices: Vec<Spinbox3Values>,
    spinbox_value_3: Spinbox3Values,
}

#[derive(Clone, PartialEq, Copy, Eq, Data)]
enum Spinbox3Values {
    One,
    Two,
    Three,
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
    Set3(Spinbox3Values),
}

fn main() {
    Application::new(|cx| {
        AppState {
            spinbox_value_1: 99,
            spinbox_value_2: 0,
            spinbox_value_3: Spinbox3Values::One,
            spinbox_value_3_choices: Spinbox3Values::values(),
        }
        .build(cx);

        ExamplePage::new(cx, |cx| {
            Spinbox::new(
                cx,
                AppState::spinbox_value_1,
                SpinboxKind::Horizontal,
                SpinboxIcons::PlusMinus,
            )
            .on_increment(|ex| ex.emit(AppEvent::Increment1))
            .on_decrement(|ex| ex.emit(AppEvent::Decrement1));

            Spinbox::custom(
                cx,
                |cx| {
                    Textbox::new(cx, AppState::spinbox_value_2)
                        .on_edit(|ex, v| ex.emit(AppEvent::Set2(v)))
                },
                SpinboxKind::Vertical,
                SpinboxIcons::PlusMinus,
            )
            .on_increment(|ex| ex.emit(AppEvent::Increment2))
            .on_decrement(|ex| ex.emit(AppEvent::Decrement2));

            Spinbox::custom(
                cx,
                |cx| {
                    Dropdown::new(
                        cx,
                        |cx| {
                            HStack::new(cx, move |cx| {
                                Label::new(cx, AppState::spinbox_value_3);
                            })
                            .child_left(Pixels(5.0))
                            .child_right(Pixels(5.0))
                            .col_between(Stretch(1.0))
                        },
                        |cx| {
                            List::new(cx, AppState::spinbox_value_3_choices, |cx, _, item| {
                                Label::new(cx, &format!("{}", item.get(cx))).on_press(
                                    move |cx| {
                                        cx.emit(AppEvent::Set3(item.get(cx)));
                                        cx.emit(PopupEvent::Close);
                                    },
                                );
                            })
                            .child_right(Pixels(4.0));
                        },
                    )
                    .width(Pixels(50.0))
                },
                SpinboxKind::Horizontal,
                SpinboxIcons::Chevrons,
            )
            .on_increment(|ex| ex.emit(AppEvent::Increment3))
            .on_decrement(|ex| ex.emit(AppEvent::Decrement3));
        });
    })
    .title("Spinbox")
    .run();
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
                let index = self.spinbox_value_3 as usize;
                self.spinbox_value_3 = Spinbox3Values::from_number((index + 1) % 3).unwrap();
            }

            AppEvent::Decrement3 => {
                let mut index = self.spinbox_value_3 as usize;
                if index == 0 {
                    index = 3
                }
                self.spinbox_value_3 = Spinbox3Values::from_number(index - 1).unwrap();
            }

            AppEvent::Set3(v) => self.spinbox_value_3 = *v,
        })
    }
}

impl Spinbox3Values {
    pub fn from_number(num: usize) -> Result<Self, ()> {
        match num {
            0 => Ok(Spinbox3Values::One),
            1 => Ok(Spinbox3Values::Two),
            2 => Ok(Spinbox3Values::Three),
            _ => Err(()),
        }
    }

    pub fn values() -> Vec<Self> {
        vec![Spinbox3Values::One, Spinbox3Values::Two, Spinbox3Values::Three]
    }
}

impl Display for Spinbox3Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Spinbox3Values::One => "one",
            Spinbox3Values::Two => "two",
            Spinbox3Values::Three => "three",
        })
    }
}

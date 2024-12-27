use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Clone, Lens)]
struct SpinboxData {
    spinbox_value_1: i64,
    // spinbox_value_2: usize,
    // spinbox_value_3_choices: Vec<SpinboxValues>,
    // spinbox_value_3: usize,
}

// #[derive(Clone, PartialEq, Copy, Eq, Data)]
// enum SpinboxValues {
//     One,
//     Two,
//     Three,
// }

// impl SpinboxValues {
//     pub fn values() -> Vec<Self> {
//         vec![SpinboxValues::One, SpinboxValues::Two, SpinboxValues::Three]
//     }
// }

// impl std::fmt::Display for SpinboxValues {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(match self {
//             SpinboxValues::One => "one",
//             SpinboxValues::Two => "two",
//             SpinboxValues::Three => "three",
//         })
//     }
// }

#[derive(Clone)]
enum SpinboxEvent {
    Increment1,
    Decrement1,
    // Increment2,
    // Decrement2,
    // Set2(String),

    // Increment3,
    // Decrement3,
    // Set3(usize),
}

impl Model for SpinboxData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            SpinboxEvent::Decrement1 => {
                self.spinbox_value_1 -= 1;
            }

            SpinboxEvent::Increment1 => {
                self.spinbox_value_1 += 1;
            } // SpinboxEvent::Decrement2 => {
              //     if self.spinbox_value_2 != 0 {
              //         self.spinbox_value_2 -= 1;
              //     }
              // }

              // SpinboxEvent::Increment2 => {
              //     self.spinbox_value_2 += 1;
              // }

              // SpinboxEvent::Set2(v) => {
              //     self.spinbox_value_2 = match v.parse::<usize>() {
              //         Ok(number) => number,
              //         Err(_) => self.spinbox_value_2,
              //     }
              // }

              // SpinboxEvent::Increment3 => {
              //     self.spinbox_value_3 = (self.spinbox_value_3 + 1) % 3;
              // }

              // SpinboxEvent::Decrement3 => {
              //     let mut index = self.spinbox_value_3 as usize;
              //     if index == 0 {
              //         index = 3
              //     }
              //     self.spinbox_value_3 = index - 1;
              // }

              // SpinboxEvent::Set3(v) => self.spinbox_value_3 = *v,
        })
    }
}

pub fn spinbox(cx: &mut Context) {
    SpinboxData {
        spinbox_value_1: 99,
        // spinbox_value_2: 0,
        // spinbox_value_3: 0,
        // spinbox_value_3_choices: SpinboxValues::values(),
    }
    .build(cx);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Spinbox");

        Divider::new(cx);

        Label::new(cx, "### Basic spinbox");

        DemoRegion::new(
            cx,
            |cx| {
                Spinbox::new(cx, SpinboxData::spinbox_value_1)
                    .width(Pixels(100.0))
                    .on_increment(|ex| ex.emit(SpinboxEvent::Increment1))
                    .on_decrement(|ex| ex.emit(SpinboxEvent::Decrement1));
            },
            r#"Spinbox::new(cx, SpinboxData::spinbox_value_1)
    .width(Pixels(100.0))
    .on_increment(|ex| ex.emit(SpinboxEvent::Increment1))
    .on_decrement(|ex| ex.emit(SpinboxEvent::Decrement1));"#,
        );
    })
    .class("panel");
}

mod helpers;
use helpers::*;
use std::fmt::Display;

use vizia::prelude::*;

#[derive(Clone, PartialEq, Copy, Eq)]
enum SpinboxValues {
    One,
    Two,
    Three,
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

struct SpinboxApp {
    value1: Signal<i64>,
    value2: Signal<usize>,
    choices: Signal<Vec<SpinboxValues>>,
    value3: Signal<usize>,
}

impl App for SpinboxApp {
    fn app_name() -> &'static str {
        "Spinbox"
    }

    fn new(cx: &mut Context) -> Self {
        Self {
            value1: cx.state(99i64),
            value2: cx.state(0usize),
            choices: cx.state(vec![SpinboxValues::One, SpinboxValues::Two, SpinboxValues::Three]),
            value3: cx.state(0usize),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let value1 = self.value1;
        let value2 = self.value2;
        let choices = self.choices;
        let value3 = self.value3;

        let icons = cx.state(SpinboxIcons::PlusMinus);

        ExamplePage::new(cx, move |cx| {
            Spinbox::new(cx, value1)
                .icons(icons)
                .width(Pixels(100.0))
                .on_increment(move |cx| value1.upd(cx, |v| *v += 1))
                .on_decrement(move |cx| value1.upd(cx, |v| *v -= 1));

            Spinbox::custom(cx, move |cx| {
                Textbox::new(cx, value2).on_edit(move |cx, v| {
                    if let Ok(n) = v.parse::<usize>() {
                        value2.set(cx, n);
                    }
                })
            })
            .icons(icons)
            .width(Pixels(100.0))
            .on_increment(move |cx| value2.upd(cx, |v| *v += 1))
            .on_decrement(move |cx| value2.upd(cx, |v| *v = v.saturating_sub(1)));

            Spinbox::custom(cx, move |cx| {
                PickList::new(cx, choices, value3, false)
                    .on_select(move |cx, val| value3.set(cx, val))
            })
            .width(Pixels(100.0))
            .on_increment(move |cx| value3.upd(cx, |v| *v = (*v + 1) % 3))
            .on_decrement(move |cx| value3.upd(cx, |v| *v = if *v == 0 { 2 } else { *v - 1 }));
        });
        self
    }
}

fn main() -> Result<(), ApplicationError> {
    SpinboxApp::run()
}

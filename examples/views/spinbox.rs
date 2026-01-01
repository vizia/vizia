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

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        let value1 = cx.state(99i64);
        let value2 = cx.state(0usize);
        let choices = cx.state(vec![SpinboxValues::One, SpinboxValues::Two, SpinboxValues::Three]);
        let value3 = cx.state(0usize);
        let spinbox_width = cx.state(Pixels(100.0));
        let plus_minus = cx.state(SpinboxIcons::PlusMinus);

        ExamplePage::new(cx, move |cx| {
            Spinbox::new(cx, value1)
                .icons(plus_minus)
                .width(spinbox_width)
                .on_increment(move |cx| value1.update(cx, |v| *v += 1))
                .on_decrement(move |cx| value1.update(cx, |v| *v -= 1));

            Spinbox::custom(cx, move |cx| {
                Textbox::new(cx, value2).on_edit(move |cx, v| {
                    if let Ok(n) = v.parse::<usize>() {
                        value2.set(cx, n);
                    }
                })
            })
            .icons(plus_minus)
            .width(spinbox_width)
            .on_increment(move |cx| value2.update(cx, |v| *v += 1))
            .on_decrement(move |cx| value2.update(cx, |v| *v = v.saturating_sub(1)));

            Spinbox::custom(cx, move |cx| {
                PickList::new(cx, choices, value3, false)
                    .on_select(move |cx, val| value3.set(cx, val))
            })
            .width(spinbox_width)
            .on_increment(move |cx| value3.update(cx, |v| *v = (*v + 1) % 3))
            .on_decrement(move |cx| value3.update(cx, |v| *v = if *v == 0 { 2 } else { *v - 1 }));
        });
        cx.state("Spinbox")
    });

    app.title(title).run()
}

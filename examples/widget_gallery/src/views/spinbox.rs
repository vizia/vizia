use vizia::prelude::*;

use crate::DemoRegion;

pub fn spinbox(cx: &mut Context) {
    let spinbox_value_1 = cx.state(99i64);
    let width_100 = cx.state(Pixels(100.0));

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Spinbox");

        Divider::new(cx);

        Label::new(cx, "### Basic spinbox");

        DemoRegion::new(
            cx,
            |cx| {
                Spinbox::new(cx, spinbox_value_1)
                    .width(width_100)
                    .on_increment(move |ex| spinbox_value_1.update(ex, |value| *value += 1))
                    .on_decrement(move |ex| spinbox_value_1.update(ex, |value| *value -= 1));
            },
            r#"let spinbox_value_1 = cx.state(99i64);
let width_100 = cx.state(Pixels(100.0));
Spinbox::new(cx, spinbox_value_1)
    .width(width_100)
    .on_increment(|ex| spinbox_value_1.update(ex, |value| *value += 1))
    .on_decrement(|ex| spinbox_value_1.update(ex, |value| *value -= 1));"#,
        );
    })
    .class("panel");
}

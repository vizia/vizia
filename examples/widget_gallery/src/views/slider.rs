use vizia::prelude::*;

use crate::DemoRegion;

pub fn slider(cx: &mut Context) {
    let value = cx.state(0.5f32);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Slider");

        Divider::new(cx);

        Markdown::new(cx, "### Basic slider");

        DemoRegion::new(
            cx,
            move |cx| {
                Slider::new(cx, value).on_change(move |cx, val| value.set(cx, val));
            },
            r#"let value = cx.state(0.5f32);
Slider::new(cx, value)
    .on_change(move |cx, val| value.set(cx, val));"#,
        );
    })
    .class("panel");
}

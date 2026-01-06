use crate::components::DemoRegion;
use vizia::prelude::*;

pub fn knob(cx: &mut Context) {
    let value = cx.state(0.2f32);

    VStack::new(cx, move |cx| {
        Markdown::new(cx, "# Knob");

        Divider::new(cx);

        DemoRegion::new(
            cx,
            move |cx| {
                Knob::new(cx, 0.5, value, false).two_way();
            },
            r#"let value = cx.state(0.2f32);
Knob::new(cx, 0.5, value, false).two_way();"#,
        );
    })
    .class("panel");
}

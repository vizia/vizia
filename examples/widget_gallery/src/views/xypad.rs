use vizia::prelude::*;

use crate::DemoRegion;

pub fn xypad(cx: &mut Context) {
    let xy = Signal::new((0.5f32, 0.5f32));

    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# XYPad
An XYPad provides a 2D input surface for controlling two values simultaneously by dragging.",
        );

        Divider::new(cx);

        DemoRegion::new(cx, "Basic XYPad", move |cx| {
            VStack::new(cx, |cx| {
                XYPad::new(cx, xy).on_change(move |_cx, x, y| xy.set((x, y))).size(Pixels(160.0));

                Label::new(cx, xy.map(|val| format!("X: {:.2}   Y: {:.2}", val.0, val.1)));
            })
            .height(Auto)
            .gap(Pixels(8.0))
            .alignment(Alignment::Center);
        });
    })
    .class("panel");
}

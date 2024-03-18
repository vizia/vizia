use vizia::prelude::*;

use crate::DemoRegion;

pub fn dialog(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Dialog").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Basic dialog").class("header");

        DemoRegion::new(
            cx,
            |_cx| {
                // Dialog::new(cx, |cx| {
                //     Label::new(cx, "todo...");
                // });
            },
            r#""#,
        );
    })
    .class("panel");
}

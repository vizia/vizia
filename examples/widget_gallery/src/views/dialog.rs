use vizia::prelude::*;

use crate::DemoRegion;

pub fn dialog(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Divider").class("title");
        Label::new(cx, "...").class("paragraph");

        // Divider here
        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        DemoRegion::new(
            cx,
            |cx| {
                Divider::new(cx);
            },
            |cx| {
                Label::new(cx, r#"TODO"#);
            },
        );
    })
    .class("panel");
}

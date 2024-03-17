use vizia::prelude::*;

use crate::DemoRegion;

pub fn divider(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Divider").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Basic divider").class("header");

        DemoRegion::new(
            cx,
            |cx| {
                Divider::new(cx);
            },
            r#"Divider::new(cx);"#,
        );
    })
    .class("panel");
}

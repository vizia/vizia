use vizia::prelude::*;

use crate::DemoRegion;

pub fn image(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Image").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx);

        DemoRegion::new(
            cx,
            |cx| {
                Label::new(cx, "Coming soon...");
            },
            r#""#,
        );
    })
    .class("panel");
}

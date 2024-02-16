use vizia::prelude::*;

use crate::DemoRegion;

pub fn image(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Image").class("title");
        Label::new(cx, "").class("paragraph");

        // Divider here
        Divider::new(cx);

        Label::new(cx, "Label").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Label::new(cx, "Hello Vizia");
            },
            |cx| {
                Label::new(cx, r#"Label::new(cx, "Hello Vizia");"#);
            },
        );
    })
    .class("panel");
}

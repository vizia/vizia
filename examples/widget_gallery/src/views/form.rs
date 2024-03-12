use vizia::prelude::*;

use crate::DemoRegion;

pub fn form(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Image").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx);

        DemoRegion::new(
            cx,
            |cx| {
                Label::new(cx, "Hello Vizia");
            },
            r#"Label::new(cx, "Hello Vizia");"#,
        );
    })
    .class("panel");
}

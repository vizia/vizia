use vizia::prelude::*;

use crate::DemoRegion;

pub fn image(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Image");

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

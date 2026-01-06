use vizia::prelude::*;

use crate::DemoRegion;

pub fn image(cx: &mut Context) {
    VStack::new(cx, move |cx| {
        Markdown::new(cx, "# Image");

        Divider::new(cx);

        DemoRegion::new(
            cx,
            move |cx| {
                Label::new(cx, "Coming soon...");
            },
            r#""#,
        );
    })
    .class("panel");
}

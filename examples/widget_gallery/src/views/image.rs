use vizia::prelude::*;

use crate::DemoRegion;

pub fn image(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Image");

        Divider::new(cx);

        DemoRegion::new(cx, "Image", |cx| {
            Label::new(cx, "Coming soon...");
        });
    })
    .class("panel");
}

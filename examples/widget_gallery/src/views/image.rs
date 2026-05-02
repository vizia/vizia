use vizia::prelude::*;

use crate::DemoRegion;

pub fn image(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("image")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Image", |cx| {
            Image::new(cx, "vizia.png")
                .width(Pixels(128.0))
                .height(Pixels(128.0))
                .background_size(vec![BackgroundSize::Cover]);
        });
    })
    .class("panel");
}

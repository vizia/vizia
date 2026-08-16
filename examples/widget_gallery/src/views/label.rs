use vizia::prelude::*;

use crate::DemoRegion;

pub fn label(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("label")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Basic Label", |cx| {
            Label::new(cx, "Hello Vizia");
        });
    })
    .class("panel");
}

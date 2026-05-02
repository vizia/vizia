use vizia::prelude::*;

use crate::DemoRegion;

pub fn divider(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("divider")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Basic Divider", |cx| {
            Divider::new(cx);
        });
    })
    .class("panel");
}

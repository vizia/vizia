use vizia::prelude::*;

use crate::DemoRegion;

pub fn divider(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Divider");

        Divider::new(cx);

        Markdown::new(cx, "### Basic divider");

        DemoRegion::new(cx, "Basic Divider", |cx| {
            Divider::new(cx);
        });
    })
    .class("panel");
}

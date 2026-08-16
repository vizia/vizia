use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn badge(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("badge")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Basic Badge", |cx| {
            Badge::new(cx, |cx| Label::new(cx, "New"));
            Badge::new(cx, |cx| Label::new(cx, "42"));
        });

        DemoRegion::new(cx, "Status Variants", |cx| {
            Badge::new(cx, |cx| Label::new(cx, "Success")).class("success");
            Badge::new(cx, |cx| Label::new(cx, "Warning")).class("warning");
            Badge::new(cx, |cx| Label::new(cx, "Error")).class("error");
        });

        DemoRegion::new(cx, "Dot Badge", |cx| {
            Badge::empty(cx);
            Badge::empty(cx).class("success");
            Badge::empty(cx).class("warning");
            Badge::empty(cx).class("error");
        });
    })
    .class("panel");
}

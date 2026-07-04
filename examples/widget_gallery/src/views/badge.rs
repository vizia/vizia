use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn badge(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("badge")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Basic Badge", |cx| {
            Button::new(cx, |cx| Label::new(cx, "Inbox"))
                .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "New")));

            Button::new(cx, |cx| Label::new(cx, "Messages"))
                .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "42")));
        });

        DemoRegion::new(cx, "Status Variants", |cx| {
            Button::new(cx, |cx| Label::new(cx, "Deploy"))
                .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "Success")).class("success"));

            Button::new(cx, |cx| Label::new(cx, "Checks"))
                .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "Warning")).class("warning"));

            Button::new(cx, |cx| Label::new(cx, "Alerts"))
                .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "Error")).class("error"));
        });

        DemoRegion::new(cx, "Dot Badge", |cx| {
            Button::new(cx, |cx| Label::new(cx, "Default")).badge(Badge::empty);

            Button::new(cx, |cx| Label::new(cx, "Online"))
                .badge(|cx| Badge::empty(cx).class("success"));

            Button::new(cx, |cx| Label::new(cx, "Pending"))
                .badge(|cx| Badge::empty(cx).class("warning"));

            Button::new(cx, |cx| Label::new(cx, "Offline"))
                .badge(|cx| Badge::empty(cx).class("error"));
        });
    })
    .class("panel");
}

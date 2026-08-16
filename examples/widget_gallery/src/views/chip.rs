use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn chip(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("chip")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Basic Chip", |cx| {
            Chip::new(cx, "Chip");
        });

        DemoRegion::new(cx, "Chip Variants", |cx| {
            Chip::new(cx, "Filled (Default)").variant(ChipVariant::Filled);
            Chip::new(cx, "Outline").variant(ChipVariant::Outline);
        });

        DemoRegion::new(cx, "Chip Actions", |cx| {
            Chip::new(cx, "Clickable").on_press(|_| {});
            Chip::new(cx, "Closable").on_close(|_| {});
            Chip::new(cx, "Clickable & Closable")
                .variant(ChipVariant::Outline)
                .on_press(|_| {})
                .on_close(|_| {});
        });
    })
    .class("panel");
}

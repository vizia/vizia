use vizia::{
    icons::{ICON_HEART, ICON_HEART_FILLED, ICON_USER},
    prelude::*,
};

use crate::components::DemoRegion;

pub fn svg(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("svg")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Svg", |cx| {
            HStack::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
                Svg::new(cx, ICON_HEART_FILLED);
            })
            .height(Auto)
            .alignment(Alignment::Center);
        });
    })
    .class("panel");
}

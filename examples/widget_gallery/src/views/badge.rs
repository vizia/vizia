use vizia::{
    icons::{ICON_CLOCK, ICON_COLUMN_INSERT_LEFT, ICON_USER},
    image,
    prelude::*,
};

use crate::components::DemoRegion;

pub fn badge(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Badge").class("title");
        Label::new(cx, "").class("paragraph");

        // Divider here
        Element::new(cx)
            .height(Pixels(1.0))
            .background_color(Color::rgb(210, 210, 210))
            .top(Pixels(12.0))
            .bottom(Pixels(12.0));

        Label::new(cx, "Badge").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Badge::new(cx, |cx| Label::new(cx, "5"));
            },
            |cx| {
                Label::new(cx, r#"Badge::new(cx, |cx| Label::new(cx, "5"));"#).class("code");
            },
        );
    })
    .class("panel");
}

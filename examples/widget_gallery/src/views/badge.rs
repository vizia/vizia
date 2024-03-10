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

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Badge").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Badge::new(cx, |cx| Label::new(cx, "5"));
            },
            r#"Badge::new(cx, |cx| Label::new(cx, "5"));"#,
        );
    })
    .class("panel");
}

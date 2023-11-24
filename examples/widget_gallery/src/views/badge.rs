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
                Avatar::new(cx, |cx| {
                    Icon::new(cx, ICON_USER);
                })
                .badge(|cx| {
                    Badge::new(cx, |cx| {
                        Icon::new(cx, ICON_CLOCK);
                    })
                    .class("warning")
                });
            },
            |cx| {
                Label::new(
                    cx,
                    r#"Avatar::new(cx, |cx|{
    Icon::new(cx, ICON_USER);
}).badge(|cx| Badge::new(cx, |cx| {Icon::new(cx, ICON_CLOCK);}).class("warning"));

Avatar::new(cx, |cx|{
    Icon::new(cx, ICON_USER);
}).badge(|cx| Badge::empty(cx).class("error"));

Avatar::new(cx, |cx|{
    Icon::new(cx, ICON_USER);
}).badge(|cx| Badge::empty(cx).class("success"));

Avatar::new(cx, |cx|{
    Icon::new(cx, ICON_USER);
}).badge(|cx| Badge::new(cx, |cx| {Label::new(cx, "2");}));"#,
                )
                .class("code");
            },
        );
    })
    .class("panel");
}

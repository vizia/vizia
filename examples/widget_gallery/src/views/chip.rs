use vizia::icons::ICON_CODE;
use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn chip(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Chip").class("title");
        Label::new(cx, "A chip can be used to inform the user of the status of specific data.")
            .class("paragraph");

        Label::new(cx, "Chip").class("header");

        DemoRegion::new(
            cx,
            |cx| {
                Chip::new(cx, "Chip").background_color(Color::from("#ff004444"));
            },
            |cx| {
                Label::new(cx, r#"Chip::new(cx, "Chip");"#).class("code");
            },
        );
    })
    .class("panel");
}

use vizia::{
    icons::{ICON_CLOCK, ICON_COLUMN_INSERT_LEFT, ICON_USER},
    image,
    prelude::*,
};

use crate::components::DemoRegion;

pub fn icon(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Icon").class("title");
        Label::new(cx, "").class("paragraph");

        // Divider here
        Element::new(cx)
            .height(Pixels(1.0))
            .background_color(Color::rgb(210, 210, 210))
            .top(Pixels(12.0))
            .bottom(Pixels(12.0));

        DemoRegion::new(
            cx,
            |cx| {
                Icon::new(cx, ICON_USER);
            },
            r#"Icon::new(cx, ICON_USER);"#,
        );
    })
    .class("panel");
}

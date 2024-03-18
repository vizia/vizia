use vizia::{icons::ICON_USER, prelude::*};

use crate::components::DemoRegion;

pub fn icon(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Icon").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

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

use vizia::{icons::ICON_USER, prelude::*};

use crate::components::DemoRegion;

pub fn svg(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Svg");

        Divider::new(cx);

        DemoRegion::new(
            cx,
            |cx| {
                Svg::new(cx, ICON_USER);
            },
            r#"Svg::new(cx, ICON_USER);"#,
        );
    })
    .class("panel");
}

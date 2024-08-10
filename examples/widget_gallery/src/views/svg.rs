use vizia::{icons::ICON_USER, prelude::*};

use crate::components::DemoRegion;

pub fn svg(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Svg").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

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

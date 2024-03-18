use vizia::prelude::*;

use crate::DemoRegion;

pub fn element(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Element").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Element").class("header");

        DemoRegion::new(
            cx,
            |cx| {
                Element::new(cx).size(Pixels(100.0)).background_color(Color::red());
            },
            r#"Element::new(cx)
    .size(Pixels(100.0))
    .background_color(Color::red());"#,
        );
    })
    .class("panel");
}

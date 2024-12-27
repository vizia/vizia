use vizia::prelude::*;

use crate::DemoRegion;

pub fn dialog(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Dialog");

        Divider::new(cx);

        Markdown::new(cx, "### Basic dialog");

        DemoRegion::new(
            cx,
            |_cx| {
                // Dialog::new(cx, |cx| {
                //     Label::new(cx, "todo...");
                // });
            },
            r#""#,
        );
    })
    .class("panel");
}

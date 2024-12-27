use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn badge(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Badge");

        Divider::new(cx);

        Markdown::new(cx, "### Basic badge");
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

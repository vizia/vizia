use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn badge(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Badge");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

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

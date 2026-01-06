use vizia::prelude::*;

use crate::DemoRegion;

pub fn divider(cx: &mut Context) {
    VStack::new(cx, move |cx| {
        Markdown::new(cx, "# Divider");

        Divider::new(cx);

        Markdown::new(cx, "### Basic divider");

        DemoRegion::new(
            cx,
            move |cx| {
                Divider::new(cx);
            },
            r#"Divider::new(cx);"#,
        );
    })
    .class("panel");
}

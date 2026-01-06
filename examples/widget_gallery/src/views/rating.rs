use vizia::prelude::*;

use crate::DemoRegion;

pub fn rating(cx: &mut Context) {
    let rating_signal = cx.state(3u32);

    VStack::new(cx, move |cx| {
        Markdown::new(cx, "# Rating");

        Divider::new(cx);

        Markdown::new(cx, "### Basic rating");

        DemoRegion::new(
            cx,
            move |cx| {
                Rating::new(cx, 5, rating_signal).two_way();
            },
            r#"let rating_signal = cx.state(3u32);
Rating::new(cx, 5, rating_signal).two_way();"#,
        );
    })
    .class("panel");
}

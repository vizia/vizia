use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn switch(cx: &mut Context) {
    let flag = cx.state(true);

    VStack::new(cx, move |cx| {
        Markdown::new(cx, "# Switch");

        Divider::new(cx);

        Markdown::new(cx, "### Basic switch");

        DemoRegion::new(
            cx,
            move |cx| {
                Switch::new(cx, flag).two_way();
            },
            r#"let flag = cx.state(true);
Switch::new(cx, flag).two_way();"#,
        );
    })
    .class("panel");
}

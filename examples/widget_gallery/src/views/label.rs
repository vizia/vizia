use vizia::prelude::*;

use crate::DemoRegion;

pub fn label(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Label").class("title");
        Label::new(cx, "A label can be used to display a string of text.").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Basic label").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Label::new(cx, "Hello Vizia");
            },
            r#"Label::new(cx, "Hello Vizia");"#,
        );
    })
    .class("panel");
}

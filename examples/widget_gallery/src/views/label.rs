use vizia::prelude::*;

use crate::DemoRegion;

pub fn label(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# Label
A label can be used to display a string of text.        
        ",
        );

        Divider::new(cx);

        Markdown::new(cx, "### Basic label");

        DemoRegion::new(
            cx,
            |cx| {
                Label::static_text(cx, "Hello Vizia");
            },
            r#"Label::static_text(cx, "Hello Vizia");"#,
        );
    })
    .class("panel");
}

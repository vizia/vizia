use vizia::prelude::*;

use crate::DemoRegion;

pub fn textbox(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Textbox").class("title");
        Label::new(cx, "A textbox can be used to display a string of text which can be edited.")
            .class("paragraph");

        // Divider here
        Element::new(cx)
            .height(Pixels(1.0))
            .background_color(Color::rgb(210, 210, 210))
            .top(Pixels(12.0))
            .bottom(Pixels(12.0));

        Label::new(cx, "Label").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Label::new(cx, "Hello Vizia");
            },
            |cx| {
                Label::new(cx, r#"Label::new(cx, "Hello Vizia");"#).class("code");
            },
        );
    })
    .class("panel");
}

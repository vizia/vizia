use vizia::prelude::*;

use crate::DemoRegion;

pub fn hstack(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "HStack").class("title");
        Label::new(cx, "The hstack container can be used to layout views in a row.")
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
                HStack::new(cx, |cx| {
                    Element::new(cx).size(Pixels(100.0)).background_color(Color::red());
                    Element::new(cx).size(Pixels(100.0)).background_color(Color::green());
                    Element::new(cx).size(Pixels(100.0)).background_color(Color::blue());
                })
                .height(Auto)
                .child_space(Stretch(1.0));
            },
            |cx| {
                Label::new(cx, r#"Label::new(cx, "Hello Vizia");"#);
            },
        );
    })
    .class("panel");
}

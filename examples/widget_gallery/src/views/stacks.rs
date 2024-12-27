use vizia::prelude::*;

use crate::DemoRegion;

pub fn hstack(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "HStack").class("title");
        Label::new(cx, "The hstack container can be used to layout views in a row.")
            .class("paragraph");

        Divider::new(cx);

        Label::new(cx, "HStack").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                HStack::new(cx, |cx| {
                    Element::new(cx).size(Pixels(100.0)).background_color(Color::red());
                    Element::new(cx).size(Pixels(100.0)).background_color(Color::green());
                    Element::new(cx).size(Pixels(100.0)).background_color(Color::blue());
                })
                .height(Auto)
                .alignment(Alignment::Center);
            },
            r#"HStack::new(cx, |cx| {
        Element::new(cx).size(Pixels(100.0)).background_color(Color::red());
        Element::new(cx).size(Pixels(100.0)).background_color(Color::green());
        Element::new(cx).size(Pixels(100.0)).background_color(Color::blue());
    })
    .height(Auto)
    .alignment(Alignment::Center);"#,
        );
    })
    .class("panel");
}

pub fn vstack(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "VStack").class("title");
        Label::new(cx, "The vstack container can be used to layout views in a column.")
            .class("paragraph");

        Divider::new(cx);

        Label::new(cx, "Label").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                VStack::new(cx, |cx| {
                    Element::new(cx).size(Pixels(100.0)).background_color(Color::red());
                    Element::new(cx).size(Pixels(100.0)).background_color(Color::green());
                    Element::new(cx).size(Pixels(100.0)).background_color(Color::blue());
                })
                .height(Auto)
                .alignment(Alignment::Center);
            },
            r#"VStack::new(cx, |cx| {
        Element::new(cx).size(Pixels(100.0)).background_color(Color::red());
        Element::new(cx).size(Pixels(100.0)).background_color(Color::green());
        Element::new(cx).size(Pixels(100.0)).background_color(Color::blue());
    })
    .height(Auto)
    .alignment(Alignment::Center);"#,
        );
    })
    .class("panel");
}

pub fn zstack(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# ZStack
The zstack container can be used to layout views in a vertical stack.        
        ",
        );

        Divider::new(cx);

        Label::new(cx, "### ZStack");

        DemoRegion::new(
            cx,
            |cx| {
                ZStack::new(cx, |cx| {
                    Element::new(cx).size(Pixels(100.0)).background_color(Color::red());
                    Element::new(cx)
                        .size(Pixels(100.0))
                        .space(Pixels(20.0))
                        .background_color(Color::green());
                    Element::new(cx)
                        .size(Pixels(100.0))
                        .space(Pixels(40.0))
                        .background_color(Color::blue());
                })
                .size(Pixels(140.0))
                .alignment(Alignment::Center);
            },
            r#"ZStack::new(cx, |cx| {
        Element::new(cx).size(Pixels(100.0)).background_color(Color::red());
        Element::new(cx)
            .size(Pixels(100.0))
            .space(Pixels(20.0))
            .background_color(Color::green());
        Element::new(cx)
            .size(Pixels(100.0))
            .space(Pixels(40.0))
            .background_color(Color::blue());
    })
    .height(Auto)
    .alignment(Alignment::Center);"#,
        );
    })
    .class("panel");
}

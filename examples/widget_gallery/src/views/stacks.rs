use vizia::prelude::*;

use crate::DemoRegion;

pub fn hstack(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# HStack
The hstack container can be used to layout views in a row.        
        ",
        );

        Divider::new(cx);

        Markdown::new(cx, "### HStack");

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
        Markdown::new(
            cx,
            "# VStack
The vstack container can be used to layout views in a column.        
        ",
        );

        Divider::new(cx);

        Markdown::new(cx, "## VStack");

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

        Markdown::new(cx, "### ZStack");

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

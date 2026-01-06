use vizia::prelude::*;

use crate::DemoRegion;

pub fn hstack(cx: &mut Context) {
    let size_100 = cx.state(Pixels(100.0));
    let auto = cx.state(Auto);
    let align_center = cx.state(Alignment::Center);

    VStack::new(cx, move |cx| {
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
            move |cx| {
                HStack::new(cx, move |cx| {
                    Element::new(cx).size(size_100).background_color(Color::red());
                    Element::new(cx).size(size_100).background_color(Color::green());
                    Element::new(cx).size(size_100).background_color(Color::blue());
                })
                .height(auto)
                .alignment(align_center);
            },
            r#"let size_100 = cx.state(Pixels(100.0));
let auto = cx.state(Auto);
let align_center = cx.state(Alignment::Center);
HStack::new(cx, move |cx| {
        Element::new(cx).size(size_100).background_color(Color::red());
        Element::new(cx).size(size_100).background_color(Color::green());
        Element::new(cx).size(size_100).background_color(Color::blue());
    })
    .height(auto)
    .alignment(align_center);"#,
        );
    })
    .class("panel");
}

pub fn vstack(cx: &mut Context) {
    let size_100 = cx.state(Pixels(100.0));
    let auto = cx.state(Auto);
    let align_center = cx.state(Alignment::Center);

    VStack::new(cx, move |cx| {
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
            move |cx| {
                VStack::new(cx, move |cx| {
                    Element::new(cx).size(size_100).background_color(Color::red());
                    Element::new(cx).size(size_100).background_color(Color::green());
                    Element::new(cx).size(size_100).background_color(Color::blue());
                })
                .height(auto)
                .alignment(align_center);
            },
            r#"let size_100 = cx.state(Pixels(100.0));
let auto = cx.state(Auto);
let align_center = cx.state(Alignment::Center);
VStack::new(cx, move |cx| {
        Element::new(cx).size(size_100).background_color(Color::red());
        Element::new(cx).size(size_100).background_color(Color::green());
        Element::new(cx).size(size_100).background_color(Color::blue());
    })
    .height(auto)
    .alignment(align_center);"#,
        );
    })
    .class("panel");
}

pub fn zstack(cx: &mut Context) {
    let size_100 = cx.state(Pixels(100.0));
    let size_140 = cx.state(Pixels(140.0));
    let space_20 = cx.state(Pixels(20.0));
    let space_40 = cx.state(Pixels(40.0));
    let align_center = cx.state(Alignment::Center);

    VStack::new(cx, move |cx| {
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
            move |cx| {
                ZStack::new(cx, |cx| {
                    Element::new(cx).size(size_100).background_color(Color::red());
                    Element::new(cx)
                        .size(size_100)
                        .space(space_20)
                        .background_color(Color::green());
                    Element::new(cx).size(size_100).space(space_40).background_color(Color::blue());
                })
                .size(size_140)
                .alignment(align_center);
            },
            r#"let size_100 = cx.state(Pixels(100.0));
let size_140 = cx.state(Pixels(140.0));
let space_20 = cx.state(Pixels(20.0));
let space_40 = cx.state(Pixels(40.0));
let align_center = cx.state(Alignment::Center);
ZStack::new(cx, |cx| {
        Element::new(cx).size(size_100).background_color(Color::red());
        Element::new(cx)
            .size(size_100)
            .space(space_20)
            .background_color(Color::green());
        Element::new(cx)
            .size(size_100)
            .space(space_40)
            .background_color(Color::blue());
    })
    .size(size_140)
    .alignment(align_center);"#,
        );
    })
    .class("panel");
}

use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Lens)]
pub struct TabData {
    tabs: Vec<&'static str>,
}

impl Model for TabData {}

pub fn tabview(cx: &mut Context) {
    TabData { tabs: vec!["Tab1", "Tab2"] }.build(cx);

    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# Label
A label can be used to display a string of text.        
        ",
        );

        Divider::new(cx);

        Label::new(cx, "### Basic tab view");

        DemoRegion::new(
            cx,
            |cx| {
                TabView::new(cx, TabData::tabs, |cx, item| match item.get(cx) {
                    "Tab1" => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx| {
                            Element::new(cx).size(Pixels(200.0)).background_color(Color::red());
                        },
                    ),

                    "Tab2" => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx| {
                            Element::new(cx).size(Pixels(200.0)).background_color(Color::blue());
                        },
                    ),

                    _ => unreachable!(),
                })
                .width(Pixels(300.0))
                .height(Pixels(300.0));
            },
            r#"TabView::new(cx, AppData::tabs, |cx, item| match item.get(cx) {
    "Tab1" => TabPair::new(
        move |cx| {
            Label::new(cx, item).hoverable(false);
            Element::new(cx).class("indicator");
        },
        |cx| {
            Element::new(cx).size(Pixels(200.0)).background_color(Color::red());
        },
    ),

    "Tab2" => TabPair::new(
        move |cx| {
            Label::new(cx, item).hoverable(false);
            Element::new(cx).class("indicator");
        },
        |cx| {
            Element::new(cx).size(Pixels(200.0)).background_color(Color::blue());
        },
    ),

    _ => unreachable!(),
})
.width(Pixels(300.0))
.height(Pixels(300.0));"#,
        );
    })
    .class("panel");
}

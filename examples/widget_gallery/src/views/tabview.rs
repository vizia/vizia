use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Lens)]
pub struct TabData {
    tabs: Vec<&'static str>,
}

impl Model for TabData {}

pub fn tabview(cx: &mut Context) {
    TabData { tabs: vec!["Tab1", "Tab2", "Tab3"] }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Label").class("title");
        Label::new(cx, "A label can be used to display a string of text.").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Basic tab view").class("header");
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
                            Element::new(cx).size(Pixels(200.0)).background_color(Color::green());
                        },
                    ),

                    "Tab3" => TabPair::new(
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

        Label::new(cx, "Vertical tab view").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                TabView::new(cx, TabData::tabs, |cx, item| match item.get(cx) {
                    "Tab1" => TabPair::new(
                        move |cx| {
                            Element::new(cx).class("indicator");
                            Label::new(cx, item).hoverable(false);
                        },
                        |cx| {
                            Element::new(cx).size(Pixels(200.0)).background_color(Color::red());
                        },
                    ),

                    "Tab2" => TabPair::new(
                        move |cx| {
                            Element::new(cx).class("indicator");
                            Label::new(cx, item).hoverable(false);
                        },
                        |cx| {
                            Element::new(cx).size(Pixels(200.0)).background_color(Color::green());
                        },
                    ),

                    "Tab3" => TabPair::new(
                        move |cx| {
                            Element::new(cx).class("indicator");
                            Label::new(cx, item).hoverable(false);
                        },
                        |cx| {
                            Element::new(cx).size(Pixels(200.0)).background_color(Color::blue());
                        },
                    ),

                    _ => unreachable!(),
                })
                .vertical()
                .width(Pixels(300.0))
                .height(Pixels(300.0));
            },
            r#"TabView::new(cx, TabData::tabs, |cx, item| match item.get(cx) {
"Tab1" => TabPair::new(
    move |cx| {
        Element::new(cx).class("indicator");
        Label::new(cx, item).hoverable(false);
    },
    |cx| {
        Element::new(cx).size(Pixels(200.0)).background_color(Color::red());
    },
),

"Tab2" => TabPair::new(
    move |cx| {
        Element::new(cx).class("indicator");
        Label::new(cx, item).hoverable(false);
    },
    |cx| {
        Element::new(cx).size(Pixels(200.0)).background_color(Color::green());
    },
),

"Tab3" => TabPair::new(
    move |cx| {
        Element::new(cx).class("indicator");
        Label::new(cx, item).hoverable(false);
    },
    |cx| {
        Element::new(cx).size(Pixels(200.0)).background_color(Color::blue());
    },
),

_ => unreachable!(),
})
.vertical()
.width(Pixels(300.0))
.height(Pixels(300.0));"#,
        );
    })
    .class("panel");
}

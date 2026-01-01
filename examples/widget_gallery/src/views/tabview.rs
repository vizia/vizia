use vizia::prelude::*;

use crate::DemoRegion;

pub fn tabview(cx: &mut Context) {
    let tabs = cx.state(vec!["Tab1", "Tab2"]);
    let size_200 = cx.state(Pixels(200.0));
    let width_300 = cx.state(Pixels(300.0));
    let height_300 = cx.state(Pixels(300.0));

    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# Label
A label can be used to display a string of text.        
        ",
        );

        Divider::new(cx);

        Label::static_text(cx, "### Basic tab view");

        DemoRegion::new(
            cx,
            |cx| {
                TabView::new(cx, tabs, |cx, item| match item.get(cx) {
                    "Tab1" => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx| {
                            Element::new(cx).size(size_200).background_color(Color::red());
                        },
                    ),

                    "Tab2" => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx| {
                            Element::new(cx).size(size_200).background_color(Color::blue());
                        },
                    ),

                    _ => unreachable!(),
                })
                .width(width_300)
                .height(height_300);
            },
            r#"let tabs = cx.state(vec!["Tab1", "Tab2"]);
let size_200 = cx.state(Pixels(200.0));
let width_300 = cx.state(Pixels(300.0));
let height_300 = cx.state(Pixels(300.0));
TabView::new(cx, tabs, |cx, item| match item.get(cx) {
    "Tab1" => TabPair::new(
        move |cx| {
            Label::new(cx, item).hoverable(false);
            Element::new(cx).class("indicator");
        },
        |cx| {
            Element::new(cx).size(size_200).background_color(Color::red());
        },
    ),

    "Tab2" => TabPair::new(
        move |cx| {
            Label::new(cx, item).hoverable(false);
            Element::new(cx).class("indicator");
        },
        |cx| {
            Element::new(cx).size(size_200).background_color(Color::blue());
        },
    ),

    _ => unreachable!(),
})
.width(width_300)
.height(height_300);"#,
        );
    })
    .class("panel");
}

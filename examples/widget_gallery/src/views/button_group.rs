use vizia::icons::{ICON_CHECK, ICON_PENCIL, ICON_TRASH};
use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn button_group(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Button Group").class("title");
        Label::new(cx, "Buttons can be grouped by wrapping them in a ButtonGroup view.")
            .class("paragraph");

        Label::new(cx, "Basic button group").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                ButtonGroup::new(cx, |cx| {
                    Button::new(cx, |cx| Label::new(cx, "One"));
                    Button::new(cx, |cx| Label::new(cx, "Two"));
                    Button::new(cx, |cx| Label::new(cx, "Three"));
                });
            },
            |cx| {
                Label::new(
                    cx,
                    r#"ButtonGroup::new(cx, |cx|{
    Button::new(cx, |cx| Label::new(cx, "One"));
    Button::new(cx, |cx| Label::new(cx, "Two"));
    Button::new(cx, |cx| Label::new(cx, "Three"));
});"#,
                )
                .class("code");
            },
        );

        Label::new(cx, "Vertical button group").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                ButtonGroup::new(cx, |cx| {
                    Button::new(cx, |cx| Label::new(cx, "One"));
                    Button::new(cx, |cx| Label::new(cx, "Two"));
                    Button::new(cx, |cx| Label::new(cx, "Three"));
                })
                .width(Pixels(100.0))
                .vertical(true);
            },
            |cx| {
                Label::new(
                    cx,
                    r#"ButtonGroup::new(cx, |cx|{
    Button::new(cx, |cx| Label::new(cx, "One"));
    Button::new(cx, |cx| Label::new(cx, "Two"));
    Button::new(cx, |cx| Label::new(cx, "Three"));
})
.width(Pixels(100.0))
.vertical(true);"#,
                )
                .class("code");
            },
        );
    })
    .class("panel");
}

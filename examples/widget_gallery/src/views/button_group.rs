use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn button_group(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# Button Group
Buttons can be grouped by wrapping them in a ButtonGroup view.        
        ",
        );

        Divider::new(cx);

        Markdown::new(cx, "### Basic button group");

        DemoRegion::new(
            cx,
            |cx| {
                ButtonGroup::new(cx, |cx| {
                    Button::new(cx, |cx| Label::new(cx, "One"));
                    Button::new(cx, |cx| Label::new(cx, "Two"));
                    Button::new(cx, |cx| Label::new(cx, "Three"));
                });
            },
            r#"ButtonGroup::new(cx, |cx|{
    Button::new(cx, |cx| Label::new(cx, "One"));
    Button::new(cx, |cx| Label::new(cx, "Two"));
    Button::new(cx, |cx| Label::new(cx, "Three"));
});"#,
        );

        Markdown::new(cx, "### Vertical button group").class("header");

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
            r#"ButtonGroup::new(cx, |cx|{
    Button::new(cx, |cx| Label::new(cx, "One"));
    Button::new(cx, |cx| Label::new(cx, "Two"));
    Button::new(cx, |cx| Label::new(cx, "Three"));
})
.width(Pixels(100.0))
.vertical(true);"#,
        );
    })
    .class("panel");
}

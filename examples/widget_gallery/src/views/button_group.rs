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
                    Button::new(cx, |cx| Label::static_text(cx, "One"));
                    Button::new(cx, |cx| Label::static_text(cx, "Two"));
                    Button::new(cx, |cx| Label::static_text(cx, "Three"));
                });
            },
            r#"ButtonGroup::new(cx, |cx|{
    Button::new(cx, |cx| Label::static_text(cx, "One"));
    Button::new(cx, |cx| Label::static_text(cx, "Two"));
    Button::new(cx, |cx| Label::static_text(cx, "Three"));
});"#,
        );

        Markdown::new(cx, "### Vertical button group").class("header");

        DemoRegion::new(
            cx,
            |cx| {
                let vertical = cx.state(true);
                ButtonGroup::new(cx, |cx| {
                    Button::new(cx, |cx| Label::static_text(cx, "One"));
                    Button::new(cx, |cx| Label::static_text(cx, "Two"));
                    Button::new(cx, |cx| Label::static_text(cx, "Three"));
                })
                .vertical(vertical);
            },
            r#"let vertical = cx.state(true);

ButtonGroup::new(cx, |cx|{
    Button::new(cx, |cx| Label::static_text(cx, "One"));
    Button::new(cx, |cx| Label::static_text(cx, "Two"));
    Button::new(cx, |cx| Label::static_text(cx, "Three"));
})
.vertical(vertical);"#,
        );
    })
    .class("panel");
}

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

        DemoRegion::new(cx, "Horizontal Button Group", |cx| {
            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "One"));
                Button::new(cx, |cx| Label::new(cx, "Two"));
                Button::new(cx, |cx| Label::new(cx, "Three"));
            });
        });

        DemoRegion::new(cx, "Vertical Button Group", |cx| {
            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "One"));
                Button::new(cx, |cx| Label::new(cx, "Two"));
                Button::new(cx, |cx| Label::new(cx, "Three"));
            })
            .vertical(true);
        });
    })
    .class("panel");
}

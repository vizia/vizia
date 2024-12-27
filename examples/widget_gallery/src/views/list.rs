use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Lens)]
pub struct ListData {
    list: Vec<u32>,
}

impl Model for ListData {}

pub fn list(cx: &mut Context) {
    let list: Vec<u32> = (1..14u32).collect();
    ListData { list }.build(cx);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# List");

        Divider::new(cx);

        Markdown::new(cx, "### Basic list");

        DemoRegion::new(
            cx,
            |cx| {
                List::new(cx, ListData::list, |cx, index, item| {
                    Label::new(cx, item)
                        .toggle_class("dark", index % 2 == 0)
                        .width(Stretch(1.0))
                        .height(Pixels(30.0))
                        .hoverable(false);
                })
                .size(Pixels(300.0));
            },
            r#"List::new(cx, ListData::list, |cx, index, item| {
    Label::new(cx, item)
        .toggle_class("dark", index % 2 == 0)
        .width(Stretch(1.0))
        .height(Pixels(30.0));
})
.width(Pixels(300.0));"#,
        );
    })
    .class("panel");
}

use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Lens)]
pub struct ListData {
    list: Vec<u32>,
}

impl Model for ListData {}

pub fn list(cx: &mut Context) {
    let list: Vec<u32> = (10..14u32).collect();
    ListData { list }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "List").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Basic list").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                List::new(cx, ListData::list, |cx, index, item| {
                    Label::new(cx, item)
                        .toggle_class("dark", index % 2 == 0)
                        .width(Stretch(1.0))
                        .height(Pixels(30.0));
                })
                .width(Pixels(300.0));
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

use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Lens)]
pub struct VirtualListData {
    list: Vec<u32>,
}

impl Model for VirtualListData {}

pub fn virtual_list(cx: &mut Context) {
    let list: Vec<u32> = (1..100u32).collect();
    VirtualListData { list }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Virtual List").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Basic virtual list").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                VirtualList::new(cx, VirtualListData::list, 40.0, |cx, index, item| {
                    Label::new(cx, item).toggle_class("dark", index % 2 == 0)
                })
                .size(Pixels(300.0));
            },
            r#"VirtualList::new(cx, VirtualListData::list, 40.0, |cx, index, item| {
        Label::new(cx, item).toggle_class("dark", index % 2 == 0)
    })
    .size(Pixels(300.0));"#,
        );
    })
    .class("panel");
}

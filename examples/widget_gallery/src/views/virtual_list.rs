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
        Markdown::new(cx, "# Virtual List");

        Divider::new(cx);

        Markdown::new(cx, "### Basic virtual list");

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

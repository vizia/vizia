use vizia::prelude::*;

use crate::DemoRegion;

pub fn virtual_list(cx: &mut Context) {
    let list = cx.state((1..100u32).collect::<Vec<_>>());
    let size_300 = cx.state(Pixels(300.0));

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Virtual List");

        Divider::new(cx);

        Markdown::new(cx, "### Basic virtual list");

        DemoRegion::new(
            cx,
            |cx| {
                VirtualList::new(cx, list, 40.0, |cx, index, item| {
                    Label::new(cx, item).toggle_class("dark", index % 2 == 0)
                })
                .size(size_300);
            },
            r#"let list = cx.state((1..100u32).collect::<Vec<_>>());
let size_300 = cx.state(Pixels(300.0));
VirtualList::new(cx, list, 40.0, |cx, index, item| {
        Label::new(cx, item).toggle_class("dark", index % 2 == 0)
    })
    .size(size_300);"#,
        );
    })
    .class("panel");
}

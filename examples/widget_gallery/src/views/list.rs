use vizia::prelude::*;

use crate::DemoRegion;

pub fn list(cx: &mut Context) {
    let list = cx.state((1..14u32).collect::<Vec<_>>());
    let stretch_one = cx.state(Stretch(1.0));
    let height_30 = cx.state(Pixels(30.0));
    let size_300 = cx.state(Pixels(300.0));

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# List");

        Divider::new(cx);

        Markdown::new(cx, "### Basic list");

        DemoRegion::new(
            cx,
            |cx| {
                List::new(cx, list, |cx, index, item| {
                    Label::new(cx, item)
                        .toggle_class("dark", index % 2 == 0)
                        .width(stretch_one)
                        .height(height_30)
                        .hoverable(false);
                })
                .size(size_300);
            },
            r#"let list = cx.state((1..14u32).collect::<Vec<_>>());
let stretch_one = cx.state(Stretch(1.0));
let height_30 = cx.state(Pixels(30.0));
let size_300 = cx.state(Pixels(300.0));
List::new(cx, list, |cx, index, item| {
    Label::new(cx, item)
        .toggle_class("dark", index % 2 == 0)
        .width(stretch_one)
        .height(height_30);
})
.size(size_300);"#,
        );
    })
    .class("panel");
}

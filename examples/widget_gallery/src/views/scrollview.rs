use vizia::prelude::*;

use crate::DemoRegion;

pub fn scrollview(cx: &mut Context) {
    let height_1000 = cx.state(Pixels(1000.0));
    let stretch_one = cx.state(Stretch(1.0));
    let size_300 = cx.state(Units::Pixels(300.0));

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Scrollview");

        Divider::new(cx);

        Markdown::new(cx, "### Vertical scroll view");

        DemoRegion::new(
            cx,
            |cx| {
                ScrollView::new(cx, |cx| {
                    Label::new(cx, "Vertical Scroll").height(height_1000).width(stretch_one);
                })
                .size(size_300)
                .class("bg-default");
            },
            r#"let height_1000 = cx.state(Pixels(1000.0));
let stretch_one = cx.state(Stretch(1.0));
let size_300 = cx.state(Units::Pixels(300.0));

ScrollView::new(cx, |cx| {
    Label::new(cx, "Vertical Scroll")
        .height(height_1000)
        .width(stretch_one);
})
.size(size_300)
.class("bg-default");"#,
        );
    })
    .class("panel");
}

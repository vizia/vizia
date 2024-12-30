use vizia::prelude::*;

use crate::DemoRegion;

pub fn scrollview(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Scrollview");

        Divider::new(cx);

        Markdown::new(cx, "### Vertical scroll view");

        DemoRegion::new(
            cx,
            |cx| {
                ScrollView::new(cx, |cx| {
                    Label::new(cx, "Vertical Scroll").height(Pixels(1000.0)).width(Stretch(1.0));
                })
                .size(Units::Pixels(300.0))
                .class("bg-default");
            },
            r#"ScrollView::new(cx, false, true, |cx| {
    Label::new(cx, "Vertical Scroll").height(Pixels(1000.0)).width(Stretch(1.0));
})
.size(Units::Pixels(300.0))
.class("bg-default");"#,
        );
    })
    .class("panel");
}

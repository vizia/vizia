use vizia::prelude::*;

use crate::DemoRegion;

pub fn scrollview(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Scrollview").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Vertical scroll view").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                    Label::new(cx, "Vertical Scroll").height(Pixels(1000.0)).width(Stretch(1.0));
                })
                .size(Units::Pixels(300.0))
                .class("bg-default");
            },
            r#"ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
    Label::new(cx, "Vertical Scroll").height(Pixels(1000.0)).width(Stretch(1.0));
})
.size(Units::Pixels(300.0))
.class("bg-default");"#,
        );
    })
    .class("panel");
}

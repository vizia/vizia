use vizia::prelude::*;

use crate::DemoRegion;

pub fn element(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Element");

        Divider::new(cx);

        Markdown::new(cx, "### Element");

        DemoRegion::new(cx, "Element", |cx| {
            Element::new(cx).size(Pixels(100.0)).background_color(Color::red());
        });
    })
    .class("panel");
}

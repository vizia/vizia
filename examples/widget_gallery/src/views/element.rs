use vizia::prelude::*;

use crate::DemoRegion;

pub fn element(cx: &mut Context) {
    let size_100 = cx.state(Pixels(100.0));

    VStack::new(cx, move |cx| {
        Markdown::new(cx, "# Element");

        Divider::new(cx);

        Markdown::new(cx, "### Element");

        DemoRegion::new(
            cx,
            move |cx| {
                Element::new(cx).size(size_100).background_color(Color::red());
            },
            r#"let size_100 = cx.state(Pixels(100.0));
Element::new(cx)
    .size(size_100)
    .background_color(Color::red());"#,
        );
    })
    .class("panel");
}

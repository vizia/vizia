use vizia::prelude::*;

use crate::DemoRegion;

pub fn element(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# Element\n`Element` is a bare-bones view with no built-in content. It is a flexible building block \
for custom layouts, decorative shapes, and styled placeholder regions.",
        );

        Divider::new(cx);

        DemoRegion::new(cx, "Colored Elements", |cx| {
            Element::new(cx).size(Pixels(60.0)).background_color(Color::red());
            Element::new(cx).size(Pixels(80.0)).background_color(Color::green());
            Element::new(cx).size(Pixels(100.0)).background_color(Color::blue());
        });

        DemoRegion::new(cx, "Rounded Element", |cx| {
            Element::new(cx)
                .width(Pixels(200.0))
                .height(Pixels(80.0))
                .background_color(Color::rgb(58, 134, 255))
                .corner_radius(Pixels(16.0));
        });
    })
    .class("panel");
}

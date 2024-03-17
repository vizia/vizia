use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn chip(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Chip").class("title");
        Label::new(cx, "A chip can be used to inform the user of the status of specific data.")
            .class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Basic chip").class("header");

        DemoRegion::new(
            cx,
            |cx| {
                Chip::new(cx, "Chip");
            },
            r#"Chip::new(cx, "Chip");"#,
        );

        Label::new(cx, "Chip variants").class("header");

        DemoRegion::new(
            cx,
            |cx| {
                Chip::new(cx, "Filled (Default)").variant(ChipVariant::Filled);
                Chip::new(cx, "Outline").variant(ChipVariant::Outline);
            },
            r#"Chip::new(cx, "Filled (Default)")
    .variant(ChipVariant::Filled);
Chip::new(cx, "Outline")
    .variant(ChipVariant::Outline);"#,
        );

        Label::new(cx, "Chip actions").class("header");

        DemoRegion::new(
            cx,
            |cx| {
                Chip::new(cx, "Clickable").on_press(|_| {});
                Chip::new(cx, "Closable").on_close(|_| {});
                Chip::new(cx, "Clickable & Closable")
                    .variant(ChipVariant::Outline)
                    .on_press(|_| {})
                    .on_close(|_| {});
            },
            r#"Chip::new(cx, "Clickable")
    .on_press(|cx| {});
Chip::new(cx, "Closable")
    .on_close(|cx| {...});
Chip::new(cx, "Clickable & Closable")
    .variant(ChipVariant::Outline)
    .on_press(|cx| {...})
    .on_close(|cx| {...});"#,
        );
    })
    .class("panel");
}

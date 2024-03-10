use vizia::icons::ICON_CODE;
use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn chip(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Chip").class("title");
        Label::new(cx, "A chip can be used to inform the user of the status of specific data.")
            .class("paragraph");

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
                Chip::new(cx, "Clickable").on_press(|cx| {});
                Chip::new(cx, "Closable").on_close(|cx| {});
                Chip::new(cx, "Clickable & Closable")
                    .variant(ChipVariant::Outline)
                    .on_press(|cx| {})
                    .on_close(|cx| {});
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

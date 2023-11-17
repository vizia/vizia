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
            |cx| {
                Label::new(cx, r#"Chip::new(cx, "Chip");"#).class("code");
            },
        );

        Label::new(cx, "Chip variants").class("header");

        DemoRegion::new(
            cx,
            |cx| {
                Chip::new(cx, "Filled (Default)").variant(ChipVariant::Filled);
                Chip::new(cx, "Outline").variant(ChipVariant::Outline);
            },
            |cx| {
                Label::new(
                    cx,
                    r#"Chip::new(cx, "Filled (Default)")
    .variant(ChipVariant::Filled);
Chip::new(cx, "Outline")
    .variant(ChipVariant::Outline);"#,
                )
                .class("code");
            },
        );

        Label::new(cx, "Chip actions").class("header");

        DemoRegion::new(
            cx,
            |cx| {
                Chip::new(cx, "Clickable").on_press(|cx| {});
                Chip::new(cx, "Closable").on_close(|cx| {});
                Chip::new(cx, "Clickable & Closable").on_press(|cx| {}).on_close(|cx| {});
            },
            |cx| {
                Label::new(
                    cx,
                    r#"Chip::new(cx, "Clickable")
    .on_press(|cx| {});
Chip::new(cx, "Closable")
    .on_close(|cx| {...});
Chip::new(cx, "Clickable & Closable")
    .on_press(|cx| {...})
    .on_close(|cx| {...});"#,
                )
                .class("code");
            },
        );
    })
    .class("panel");
}

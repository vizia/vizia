use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn chip(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# Chip
A chip can be used to inform the user of the status of specific data.        
        ",
        );

        Divider::new(cx);

        Markdown::new(cx, "### Basic chip");

        DemoRegion::new(
            cx,
            |cx| {
                Chip::new(cx, "Chip");
            },
            r#"Chip::new(cx, "Chip");"#,
        );

        Markdown::new(cx, "### Chip variants");

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

        Markdown::new(cx, "### Chip actions");

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

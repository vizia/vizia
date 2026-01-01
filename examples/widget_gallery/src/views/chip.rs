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
                Chip::static_text(cx, "Chip");
            },
            r#"Chip::static_text(cx, "Chip");"#,
        );

        Markdown::new(cx, "### Chip variants");

        DemoRegion::new(
            cx,
            |cx| {
                let filled_variant = cx.state(ChipVariant::Filled);
                let outline_variant = cx.state(ChipVariant::Outline);

                Chip::static_text(cx, "Filled (Default)").variant(filled_variant);
                Chip::static_text(cx, "Outline").variant(outline_variant);
            },
            r#"let filled_variant = cx.state(ChipVariant::Filled);
let outline_variant = cx.state(ChipVariant::Outline);

Chip::static_text(cx, "Filled (Default)").variant(filled_variant);
Chip::static_text(cx, "Outline").variant(outline_variant);"#,
        );

        Markdown::new(cx, "### Chip actions");

        DemoRegion::new(
            cx,
            |cx| {
                let outline_variant = cx.state(ChipVariant::Outline);

                Chip::static_text(cx, "Clickable").on_press(|_| {});
                Chip::static_text(cx, "Closable").on_close(|_| {});
                Chip::static_text(cx, "Clickable & Closable")
                    .variant(outline_variant)
                    .on_press(|_| {})
                    .on_close(|_| {});
            },
            r#"let outline_variant = cx.state(ChipVariant::Outline);

Chip::static_text(cx, "Clickable")
    .on_press(|cx| {});
Chip::static_text(cx, "Closable")
    .on_close(|cx| {});
Chip::static_text(cx, "Clickable & Closable")
    .variant(outline_variant)
    .on_press(|cx| {})
    .on_close(|cx| {});"#,
        );
    })
    .class("panel");
}

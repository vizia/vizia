use vizia::prelude::*;

use crate::DemoRegion;

use vizia::icons::{ICON_BOLD, ICON_ITALIC, ICON_UNDERLINE};

pub fn toggle_button(cx: &mut Context) {
    let bold = cx.state(false);
    let italic = cx.state(false);
    let underline = cx.state(false);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# ToggleButton");

        Divider::new(cx);

        Markdown::new(cx, "### Basic toggle button");

        DemoRegion::new(
            cx,
            |cx| {
                ToggleButton::new(cx, bold, |cx| Label::new(cx, "Bold")).two_way();
            },
            r#"let bold = cx.state(false);
ToggleButton::new(cx, bold, |cx| Label::new(cx, "Bold")).two_way();"#,
        );

        Markdown::new(cx, "### Toggle button group");

        DemoRegion::new(
            cx,
            |cx| {
                ButtonGroup::new(cx, |cx| {
                    ToggleButton::new(cx, bold, |cx| Svg::new(cx, ICON_BOLD)).two_way();

                    ToggleButton::new(cx, italic, |cx| Svg::new(cx, ICON_ITALIC)).two_way();

                    ToggleButton::new(cx, underline, |cx| Svg::new(cx, ICON_UNDERLINE)).two_way();
                });
            },
            r#"let bold = cx.state(false);
let italic = cx.state(false);
let underline = cx.state(false);
ButtonGroup::new(cx, |cx| {
    ToggleButton::new(cx, bold, |cx| Svg::new(cx, ICON_BOLD)).two_way();

    ToggleButton::new(cx, italic, |cx| Svg::new(cx, ICON_ITALIC)).two_way();

    ToggleButton::new(cx, underline, |cx| Svg::new(cx, ICON_UNDERLINE)).two_way();
});"#,
        );
    })
    .class("panel");
}

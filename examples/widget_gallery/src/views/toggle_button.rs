use vizia::prelude::*;

use crate::DemoRegion;

use vizia::icons::{ICON_BOLD, ICON_ITALIC, ICON_UNDERLINE};

#[derive(Lens)]
pub struct ToggleData {
    bold: bool,
    italic: bool,
    underline: bool,
}

pub enum ToggleEvent {
    Bold,
    Italic,
    Underline,
}

impl Model for ToggleData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            ToggleEvent::Bold => {
                self.bold ^= true;
            }

            ToggleEvent::Italic => {
                self.italic ^= true;
            }

            ToggleEvent::Underline => {
                self.underline ^= true;
            }
        })
    }
}

pub fn toggle_button(cx: &mut Context) {
    ToggleData { bold: false, italic: false, underline: false }.build(cx);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# ToggleButton");

        Divider::new(cx);

        Markdown::new(cx, "### Basic toggle button");

        DemoRegion::new(
            cx,
            |cx| {
                ToggleButton::new(cx, ToggleData::bold, |cx| Label::new(cx, "Bold"))
                    .on_toggle(|cx| cx.emit(ToggleEvent::Bold));
            },
            r#"ToggleButton::new(cx, ToggleData::bold, |cx| Label::new(cx, "Bold"))
    .on_toggle(|cx| cx.emit(ToggleEvent::ToggleBold));"#,
        );

        Markdown::new(cx, "### Toggle button group");

        DemoRegion::new(
            cx,
            |cx| {
                ButtonGroup::new(cx, |cx| {
                    ToggleButton::new(cx, ToggleData::bold, |cx| Svg::new(cx, ICON_BOLD))
                        .on_toggle(|cx| cx.emit(ToggleEvent::Bold));

                    ToggleButton::new(cx, ToggleData::italic, |cx| Svg::new(cx, ICON_ITALIC))
                        .on_toggle(|cx| cx.emit(ToggleEvent::Italic));

                    ToggleButton::new(cx, ToggleData::underline, |cx| Svg::new(cx, ICON_UNDERLINE))
                        .on_toggle(|cx| cx.emit(ToggleEvent::Underline));
                });
            },
            r#"ButtonGroup::new(cx, |cx| {
    ToggleButton::new(cx, ToggleData::bold, |cx| Svg::new(cx, ICON_BOLD))
        .on_toggle(|cx| cx.emit(ToggleEvent::ToggleBold));

    ToggleButton::new(cx, ToggleData::italic, |cx| Svg::new(cx, ICON_ITALIC))
        .on_toggle(|cx| cx.emit(ToggleEvent::ToggleItalic));

    ToggleButton::new(cx, ToggleData::underline, |cx| {
        Svg::new(cx, ICON_UNDERLINE)
    })
    .on_toggle(|cx| cx.emit(ToggleEvent::ToggleUnderline));
});"#,
        );
    })
    .class("panel");
}

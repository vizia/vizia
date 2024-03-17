use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Lens)]
pub struct ToggleData {
    bold: bool,
    italic: bool,
    underline: bool,
}

pub enum ToggleEvent {
    ToggleBold,
    ToggleItalic,
    ToggleUnderline,
}

impl Model for ToggleData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            ToggleEvent::ToggleBold => {
                self.bold ^= true;
            }

            ToggleEvent::ToggleItalic => {
                self.italic ^= true;
            }

            ToggleEvent::ToggleUnderline => {
                self.underline ^= true;
            }
        })
    }
}

pub fn toggle_button(cx: &mut Context) {
    ToggleData { bold: false, italic: false, underline: false }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "ToggleButton").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Basic toggle button").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                ToggleButton::new(cx, ToggleData::bold, |cx| Label::new(cx, "Bold"))
                    .on_toggle(|cx| cx.emit(ToggleEvent::ToggleBold));
            },
            r#"ToggleButton::new(cx, ToggleData::bold, |cx| Label::new(cx, "Bold"))
    .on_toggle(|cx| cx.emit(ToggleEvent::ToggleBold));"#,
        );
    })
    .class("panel");
}

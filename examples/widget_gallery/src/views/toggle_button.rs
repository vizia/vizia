use vizia::prelude::*;

use crate::DemoRegion;

use vizia::icons::{ICON_BOLD, ICON_ITALIC, ICON_UNDERLINE};

pub struct ToggleData {
    bold: Signal<bool>,
    italic: Signal<bool>,
    underline: Signal<bool>,
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
                self.bold.update(|bold| *bold ^= true);
            }

            ToggleEvent::Italic => {
                self.italic.update(|italic| *italic ^= true);
            }

            ToggleEvent::Underline => {
                self.underline.update(|underline| *underline ^= true);
            }
        })
    }
}

pub fn toggle_button(cx: &mut Context) {
    let bold = Signal::new(false);
    let italic = Signal::new(false);
    let underline = Signal::new(false);

    ToggleData { bold, italic, underline }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("toggle-button")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Toggle Button", move |cx| {
            ToggleButton::new(cx, bold, |cx| Label::new(cx, "Bold"))
                .on_toggle(|cx| cx.emit(ToggleEvent::Bold));
        });

        DemoRegion::new(cx, "Toggle Button Group", move |cx| {
            ButtonGroup::new(cx, |cx| {
                ToggleButton::new(cx, bold, |cx| Svg::new(cx, ICON_BOLD))
                    .on_toggle(|cx| cx.emit(ToggleEvent::Bold));

                ToggleButton::new(cx, italic, |cx| Svg::new(cx, ICON_ITALIC))
                    .on_toggle(|cx| cx.emit(ToggleEvent::Italic));

                ToggleButton::new(cx, underline, |cx| Svg::new(cx, ICON_UNDERLINE))
                    .on_toggle(|cx| cx.emit(ToggleEvent::Underline));
            });
        });
    })
    .class("panel");
}

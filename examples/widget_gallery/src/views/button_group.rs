use vizia::icons::{ICON_BOLD, ICON_ITALIC, ICON_UNDERLINE};
use vizia::prelude::*;

use crate::components::DemoRegion;

pub struct ButtonGroupData {
    bold: Signal<bool>,
    italic: Signal<bool>,
    underline: Signal<bool>,
}

pub enum ButtonGroupEvent {
    Bold,
    Italic,
    Underline,
}

impl Model for ButtonGroupData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            ButtonGroupEvent::Bold => self.bold.update(|v| *v ^= true),
            ButtonGroupEvent::Italic => self.italic.update(|v| *v ^= true),
            ButtonGroupEvent::Underline => self.underline.update(|v| *v ^= true),
        });
    }
}

pub fn button_group(cx: &mut Context) {
    let bold = Signal::new(false);
    let italic = Signal::new(false);
    let underline = Signal::new(false);

    ButtonGroupData { bold, italic, underline }.build(cx);

    VStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, Localized::new("button-group")).class("panel-title");
            Label::new(cx, Localized::new("button-group").attribute("description"));
        })
        .height(Auto)
        .gap(Pixels(4.0));

        Divider::new(cx);

        DemoRegion::new(cx, Localized::new("demo-region-horizontal-button-group"), |cx| {
            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, Localized::new("one")));
                Button::new(cx, |cx| Label::new(cx, Localized::new("two")));
                Button::new(cx, |cx| Label::new(cx, Localized::new("three")));
            });
        });

        DemoRegion::new(cx, Localized::new("demo-region-vertical-button-group"), |cx| {
            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, Localized::new("one")));
                Button::new(cx, |cx| Label::new(cx, Localized::new("two")));
                Button::new(cx, |cx| Label::new(cx, Localized::new("three")));
            })
            .vertical(true);
        });

        DemoRegion::new(cx, Localized::new("demo-region-toggle-button-group"), move |cx| {
            ButtonGroup::new(cx, |cx| {
                ToggleButton::new(cx, bold, |cx| {
                    Svg::new(cx, ICON_BOLD).direction(Direction::LeftToRight)
                })
                .on_toggle(|cx| cx.emit(ButtonGroupEvent::Bold));
                ToggleButton::new(cx, italic, |cx| {
                    Svg::new(cx, ICON_ITALIC).direction(Direction::LeftToRight)
                })
                .on_toggle(|cx| cx.emit(ButtonGroupEvent::Italic));
                ToggleButton::new(cx, underline, |cx| {
                    Svg::new(cx, ICON_UNDERLINE).direction(Direction::LeftToRight)
                })
                .on_toggle(|cx| cx.emit(ButtonGroupEvent::Underline));
            });
        });
    })
    .class("panel");
}

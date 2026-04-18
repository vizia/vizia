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
        Markdown::new(
            cx,
            "# Button Group
Buttons can be grouped by wrapping them in a ButtonGroup view.
        ",
        );

        Divider::new(cx);

        DemoRegion::new(cx, "Horizontal Button Group", |cx| {
            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "One"));
                Button::new(cx, |cx| Label::new(cx, "Two"));
                Button::new(cx, |cx| Label::new(cx, "Three"));
            });
        });

        DemoRegion::new(cx, "Vertical Button Group", |cx| {
            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "One"));
                Button::new(cx, |cx| Label::new(cx, "Two"));
                Button::new(cx, |cx| Label::new(cx, "Three"));
            })
            .vertical(true);
        });

        Divider::new(cx);

        DemoRegion::new(cx, "Toggle Button Group", move |cx| {
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

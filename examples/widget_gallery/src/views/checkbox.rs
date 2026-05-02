use vizia::prelude::*;

use crate::components::DemoRegion;

pub struct CheckboxData {
    check_a: Signal<bool>,
}

pub enum CheckboxEvent {
    ToggleA,
}

impl Model for CheckboxData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            CheckboxEvent::ToggleA => {
                self.check_a.update(|check_a| *check_a ^= true);
            }
        });
    }
}

pub fn checkbox(cx: &mut Context) {
    let check_a = Signal::new(true);
    CheckboxData { check_a }.build(cx);

    VStack::new(cx, move |cx| {
        Label::new(cx, Localized::new("checkbox")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Basic Checkboxes", move |cx| {
            Checkbox::new(cx, check_a).on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA));
        });

        DemoRegion::new(cx, "Labelled Checkbox", move |cx| {
            HStack::new(cx, |cx| {
                Checkbox::new(cx, check_a)
                    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA))
                    .id("check");
                Label::new(cx, "Label").describing("check");
            })
            .size(Auto)
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(8.0));
        });
    })
    .class("panel");
}

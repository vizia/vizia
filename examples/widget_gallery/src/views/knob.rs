use crate::components::DemoRegion;
use chrono::{NaiveDate, Utc};
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct KnobState {
    value: f32,
}

pub enum KnobEvent {
    SetValue(f32),
}

impl Model for KnobState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            KnobEvent::SetValue(val) => {
                self.value = *val;
            }
        });
    }
}

pub fn knob(cx: &mut Context) {
    VStack::new(cx, |cx| {
        KnobState { value: 0.2 }.build(cx);

        Label::new(cx, "Knob").class("title");
        Label::new(cx, "todo...").class("paragraph");

        Label::new(cx, "Basic knob").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Knob::new(cx, 0.5, KnobState::value, false)
                    .on_changing(|cx, val| cx.emit(KnobEvent::SetValue(val)));
            },
            |cx| {
                Label::new(
                    cx,
                    r#"Knob::new(cx, 0.5, KnobState::value, false)
    .on_changing(|cx, val| cx.emit(KnobEvent::SetValue(val)));"#,
                )
                .class("code");
            },
        );
    })
    .class("panel");
}

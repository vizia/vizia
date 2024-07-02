use crate::components::DemoRegion;
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
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        DemoRegion::new(
            cx,
            |cx| {
                // Knob::new(cx, 0.5, KnobState::value, false)
                //     .on_changing(|cx, val| cx.emit(KnobEvent::SetValue(val)));
            },
            r#"Knob::new(cx, 0.5, KnobState::value, false)
    .on_changing(|cx, val| cx.emit(KnobEvent::SetValue(val)));"#,
        );
    })
    .class("panel");
}

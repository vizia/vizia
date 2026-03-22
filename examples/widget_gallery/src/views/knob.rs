use crate::components::DemoRegion;
use vizia::prelude::*;

struct KnobState {
    value: Signal<f32>,
}

pub enum KnobEvent {
    SetValue(f32),
}

impl Model for KnobState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            KnobEvent::SetValue(val) => {
                self.value.set(*val);
            }
        });
    }
}

pub fn knob(cx: &mut Context) {
    let value = Signal::new(0.2);

    VStack::new(cx, |cx| {
        KnobState { value }.build(cx);

        Markdown::new(cx, "# Knob");

        Divider::new(cx);

        DemoRegion::new(
            cx,
            move |cx| {
                Knob::new(cx, 0.5, value, false)
                    .on_change(|cx, val| cx.emit(KnobEvent::SetValue(val)));
            },
            r#"Knob::new(cx, 0.5, KnobState::value, false)
    .on_changing(|cx, val| cx.emit(KnobEvent::SetValue(val)));"#,
        );
    })
    .class("panel");
}

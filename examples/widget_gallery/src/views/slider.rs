use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Lens)]
pub struct SliderData {
    value: f32,
}

pub enum SliderEvent {
    SetValue(f32),
}

impl Model for SliderData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            SliderEvent::SetValue(value) => {
                self.value = *value;
            }
        });
    }
}

pub fn slider(cx: &mut Context) {
    SliderData { value: 0.5 }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Slider").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Basic slider").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Slider::new(cx, SliderData::value)
                    .on_changing(|cx, value| cx.emit(SliderEvent::SetValue(value)));
            },
            r#"Slider::new(cx, SliderData::value)
    .on_changing(|cx, value| cx.emit(SliderEvent::SetValue(value)));"#,
        );
    })
    .class("panel");
}

use vizia::prelude::*;

use crate::DemoRegion;

pub struct SliderData {
    value: Signal<f32>,
}

pub enum SliderEvent {
    SetValue(f32),
}

impl Model for SliderData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            SliderEvent::SetValue(value) => {
                self.value.set(*value);
            }
        });
    }
}

pub fn slider(cx: &mut Context) {
    let value = Signal::new(0.5);
    SliderData { value }.build(cx);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Slider");

        Divider::new(cx);

        Markdown::new(cx, "### Basic slider");

        DemoRegion::new(cx, "Basic Slider", move |cx| {
            Slider::new(cx, value).on_change(|cx, value| cx.emit(SliderEvent::SetValue(value)));
        });
    })
    .class("panel");
}

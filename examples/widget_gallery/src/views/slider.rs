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
        Label::new(cx, "Badge").class("title");
        Label::new(cx, "").class("paragraph");

        // Divider here
        Element::new(cx)
            .height(Pixels(1.0))
            .background_color(Color::rgb(210, 210, 210))
            .top(Pixels(12.0))
            .bottom(Pixels(12.0));

        Label::new(cx, "Badge").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Slider::new(cx, SliderData::value)
                    .on_changing(|cx, value| cx.emit(SliderEvent::SetValue(value)));
            },
            |cx| {
                Label::new(
                    cx,
                    r#"Slider::new(cx, SliderData::value)
    .on_changing(|cx, value| cx.emit(SliderEvent::SetValue(value)));"#,
                )
                .class("code");
            },
        );
    })
    .class("panel");
}

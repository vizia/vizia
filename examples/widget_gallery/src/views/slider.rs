use vizia::prelude::*;

use crate::DemoRegion;

pub fn slider(cx: &mut Context) {
    let value = Signal::new(0.5f32);
    let ranged = Signal::new(20.0f32);
    let stepped = Signal::new(0.0f32);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("slider")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Slider", move |cx| {
            VStack::new(cx, |cx| {
                Slider::new(cx, value).on_change(move |_cx, v| value.set(v)).width(Pixels(200.0));
                Label::new(cx, value.map(|v| format!("{:.2}", v)));
            })
            .height(Auto)
            .gap(Pixels(8.0))
            .alignment(Alignment::Center);
        });

        DemoRegion::new(cx, "Range Slider", move |cx| {
            VStack::new(cx, |cx| {
                Slider::new(cx, ranged)
                    .range(0.0f32..100.0f32)
                    .on_change(move |_cx, v| ranged.set(v))
                    .width(Pixels(200.0));
                Label::new(cx, ranged.map(|v| format!("{:.0}", v)));
            })
            .height(Auto)
            .gap(Pixels(8.0))
            .alignment(Alignment::Center);
        });

        DemoRegion::new(cx, "Stepped Slider", move |cx| {
            VStack::new(cx, |cx| {
                Slider::new(cx, stepped)
                    .step(0.05f32)
                    .on_change(move |_cx, v| stepped.set(v))
                    .width(Pixels(200.0));
                Label::new(cx, stepped.map(|v| format!("{:.2}", v)));
            })
            .height(Auto)
            .gap(Pixels(8.0))
            .alignment(Alignment::Center);
        });

        DemoRegion::new(cx, "Vertical Slider", move |cx| {
            Slider::new(cx, value)
                .vertical(true)
                .on_change(move |_cx, v| value.set(v))
                .height(Pixels(150.0));
        });
    })
    .class("panel");
}

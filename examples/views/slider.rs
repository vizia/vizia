mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let value = cx.state(0.0f32);

        ExamplePage::new(cx, |cx| {
            // Normalized slider (0..1) displaying -50..50 range
            HStack::new(cx, |cx| {
                let normalized = cx.state(0.5f32);
                Slider::new(cx, normalized)
                    .range(0.0..1.0)
                    .on_change(move |cx, val| {
                        normalized.set(cx, val);
                        value.set(cx, -50.0 + (val * 100.0));
                    });
                Label::new(cx, normalized.map(|val| format!("{:.2}", val)))
                    .width(Pixels(50.0));
            })
            .alignment(Alignment::Center)
            .height(Auto)
            .horizontal_gap(Pixels(8.0));

            // Direct range slider
            HStack::new(cx, |cx| {
                Slider::new(cx, value)
                    .range(-50.0..50.0)
                    .on_change(move |cx, val| value.set(cx, val));
                Label::new(cx, value.map(|val| format!("{:.2}", val))).width(Pixels(50.0));
            })
            .alignment(Alignment::Center)
            .height(Auto)
            .horizontal_gap(Pixels(8.0));

            // Vertical slider
            VStack::new(cx, |cx| {
                Slider::new(cx, value)
                    .range(-50.0..50.0)
                    .on_change(move |cx, val| value.set(cx, val))
                    .orientation(Orientation::Vertical);
                Label::new(cx, value.map(|val| format!("{:.2}", val)))
                    .alignment(Alignment::Center)
                    .width(Pixels(50.0));
            })
            .alignment(Alignment::Center)
            .vertical_gap(Pixels(8.0));
        });
    })
    .title("Slider")
    .run()
}

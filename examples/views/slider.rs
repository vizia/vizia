mod helpers;
use helpers::*;
use vizia::prelude::*;

struct SliderApp {
    value: Signal<f32>,
    vertical: Signal<Orientation>,
}

impl App for SliderApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            value: cx.state(0.0f32),
            vertical: cx.state(Orientation::Vertical),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let value = self.value;
        let vertical = self.vertical;

        ExamplePage::new(cx, |cx| {
            let value_label = cx.derived({
                let value = value;
                move |store| format!("{:.2}", *value.get(store))
            });
            let normalized = cx.derived({
                let value = value;
                move |store| ((*value.get(store) + 50.0) / 100.0).clamp(0.0, 1.0)
            });
            let normalized_label = cx.derived({
                let normalized = normalized;
                move |store| format!("{:.2}", *normalized.get(store))
            });

            // Normalized slider (0..1) displaying -50..50 range
            HStack::new(cx, |cx| {
                Slider::new(cx, normalized).range(0.0..1.0).on_change(move |cx, val| {
                    value.set(cx, -50.0 + (val * 100.0));
                });
                Label::new(cx, normalized_label).width(Pixels(50.0));
            })
            .alignment(Alignment::Center)
            .height(Auto)
            .horizontal_gap(Pixels(8.0));

            // Direct range slider
            HStack::new(cx, |cx| {
                Slider::new(cx, value)
                    .range(-50.0..50.0)
                    .on_change(move |cx, val| value.set(cx, val));
                Label::new(cx, value_label).width(Pixels(50.0));
            })
            .alignment(Alignment::Center)
            .height(Auto)
            .horizontal_gap(Pixels(8.0));

            // Vertical slider
            VStack::new(cx, |cx| {
                Slider::new(cx, value)
                    .range(-50.0..50.0)
                    .on_change(move |cx, val| value.set(cx, val))
                    .orientation(vertical);
                Label::new(cx, value_label).alignment(Alignment::Center).width(Pixels(50.0));
            })
            .alignment(Alignment::Center)
            .vertical_gap(Pixels(8.0));
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Slider"))
    }
}

fn main() -> Result<(), ApplicationError> {
    SliderApp::run()
}

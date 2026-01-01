mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        let value = cx.state(0.0f32);
        let label_width = cx.state(Pixels(50.0));
        let align_center = cx.state(Alignment::Center);
        let auto = cx.state(Auto);
        let gap_8 = cx.state(Pixels(8.0));
        let vertical = cx.state(Orientation::Vertical);

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
                Label::new(cx, normalized_label).width(label_width);
            })
            .alignment(align_center)
            .height(auto)
            .horizontal_gap(gap_8);

            // Direct range slider
            HStack::new(cx, |cx| {
                Slider::new(cx, value)
                    .range(-50.0..50.0)
                    .on_change(move |cx, val| value.set(cx, val));
                Label::new(cx, value_label).width(label_width);
            })
            .alignment(align_center)
            .height(auto)
            .horizontal_gap(gap_8);

            // Vertical slider
            VStack::new(cx, |cx| {
                Slider::new(cx, value)
                    .range(-50.0..50.0)
                    .on_change(move |cx, val| value.set(cx, val))
                    .orientation(vertical);
                Label::new(cx, value_label).alignment(align_center).width(label_width);
            })
            .alignment(align_center)
            .vertical_gap(gap_8);
        });
        cx.state("Slider")
    });

    app.title(title).run()
}

mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        // Single signal for the XY coordinates
        let xy = cx.state((0.25f32, 0.25f32));
        let width_10 = cx.state(Pixels(10.0));
        let height_100 = cx.state(Pixels(100.0));
        let width_100 = cx.state(Pixels(100.0));
        let height_10 = cx.state(Pixels(10.0));
        let auto = cx.state(Auto);
        let gap_5 = cx.state(Pixels(5.0));
        let align_center = cx.state(Alignment::Center);
        let vertical = cx.state(Orientation::Vertical);

        // Derived signals for individual x and y sliders
        let x_value = cx.derived({
            let xy = xy;
            move |s| xy.get(s).0
        });
        let y_value = cx.derived({
            let xy = xy;
            move |s| xy.get(s).1
        });

        ExamplePage::vertical(cx, |cx| {
            Label::static_text(cx, "2-dimensional XY Pad");
            VStack::new(cx, move |cx| {
                HStack::new(cx, move |cx| {
                    Slider::new(cx, y_value)
                        .width(width_10)
                        .height(height_100)
                        .range(0.0..1.0)
                        .orientation(vertical)
                        .on_change(move |cx, val| {
                            let x = xy.get(cx).0;
                            xy.set(cx, (x, val));
                        });
                    XYPad::new(cx, xy).two_way();
                })
                .size(auto)
                .horizontal_gap(gap_5)
                .alignment(align_center);
                Slider::new(cx, x_value)
                    .width(width_100)
                    .height(height_10)
                    .range(0.0..1.0)
                    .on_change(move |cx, val| {
                        let y = xy.get(cx).1;
                        xy.set(cx, (val, y));
                    });
            })
            .alignment(align_center);
        });
        cx.state("XY Pad")
    });

    app.title(title).run()
}

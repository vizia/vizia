mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    XYPadApp::run()
}

struct XYPadApp {
    xy: Signal<(f32, f32)>,
}

impl App for XYPadApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            xy: cx.state((0.25f32, 0.25f32)),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let xy = self.xy;

        // Derived signals for individual x and y sliders
        let x_value = cx.derived(move |s| xy.get(s).0);
        let y_value = cx.derived(move |s| xy.get(s).1);

        ExamplePage::vertical(cx, |cx| {
            Label::new(cx, "2-dimensional XY Pad");
            VStack::new(cx, move |cx| {
                HStack::new(cx, move |cx| {
                    Slider::new(cx, y_value)
                        .width(Pixels(10.0))
                        .height(Pixels(100.0))
                        .range(0.0..1.0)
                        .orientation(Orientation::Vertical)
                        .on_change(move |cx, val| {
                            let x = xy.get(cx).0;
                            xy.set(cx, (x, val));
                        });
                    XYPad::new(cx, xy).two_way();
                })
                .size(Auto)
                .horizontal_gap(Pixels(5.0))
                .alignment(Alignment::Center);
                Slider::new(cx, x_value)
                    .width(Pixels(100.0))
                    .height(Pixels(10.0))
                    .range(0.0..1.0)
                    .on_change(move |cx, val| {
                        let y = xy.get(cx).1;
                        xy.set(cx, (val, y));
                    });
            })
            .alignment(Alignment::Center);
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("XY Pad"))
    }
}

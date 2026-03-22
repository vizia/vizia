mod helpers;
use helpers::*;
use vizia::prelude::*;

pub struct AppData {
    pub xy_data: Signal<(f32, f32)>,
}

#[derive(Debug)]
pub enum AppEvent {
    XYPadChange(f32, f32),
    XSliderChange(f32),
    YSliderChange(f32),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::XYPadChange(value_x, value_y) => {
                self.xy_data.set((*value_x, *value_y));
            }
            AppEvent::XSliderChange(value_x) => {
                self.xy_data.update(|xy| xy.0 = *value_x);
            }
            AppEvent::YSliderChange(value_y) => {
                self.xy_data.update(|xy| xy.1 = *value_y);
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let xy_data = Signal::new((0.25, 0.25));
        let value_x = Memo::new(move |_| xy_data.get().0);
        let value_y = Memo::new(move |_| xy_data.get().1);

        AppData { xy_data }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            Label::new(cx, "2-dimensional XY Pad");
            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Slider::new(cx, value_y)
                        .width(Pixels(10.0))
                        .height(Pixels(100.0))
                        .range(0.0..1.0)
                        .on_change(move |cx, val| cx.emit(AppEvent::YSliderChange(val)));
                    // XY pad
                    XYPad::new(cx, xy_data).on_change(|ex, value_x, value_y| {
                        ex.emit(AppEvent::XYPadChange(value_x, value_y))
                    });
                })
                .size(Auto)
                .horizontal_gap(Pixels(5.0))
                .alignment(Alignment::Center);
                Slider::new(cx, value_x)
                    .width(Pixels(100.0))
                    .height(Pixels(10.0))
                    .range(0.0..1.0)
                    .on_change(move |cx, val| cx.emit(AppEvent::XSliderChange(val)));
            })
            .alignment(Alignment::Center);
        });
    })
    .title("XY Pad")
    .run()
}

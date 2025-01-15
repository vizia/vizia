mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Debug, Lens)]
pub struct AppData {
    pub xy_data: (f32, f32),
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
                self.xy_data = (*value_x, *value_y);
            }
            AppEvent::XSliderChange(value_x) => {
                self.xy_data.0 = *value_x;
            }
            AppEvent::YSliderChange(value_y) => {
                self.xy_data.1 = *value_y;
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { xy_data: (0.25, 0.25) }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            Label::new(cx, "2-dimensional XY Pad");
            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Slider::new(cx, AppData::xy_data.map(|data| data.1))
                        .width(Pixels(10.0))
                        .height(Pixels(100.0))
                        .range(0.0..1.0)
                        .on_change(move |cx, val| cx.emit(AppEvent::YSliderChange(val)));
                    // XY pad
                    XYPad::new(cx, AppData::xy_data.map(|data| (data.0, data.1))).on_change(
                        |ex, value_x, value_y| ex.emit(AppEvent::XYPadChange(value_x, value_y)),
                    );
                })
                .size(Auto)
                .horizontal_gap(Pixels(5.0))
                .alignment(Alignment::Center);
                Slider::new(cx, AppData::xy_data.map(|data| data.0))
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

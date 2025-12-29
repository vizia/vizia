mod helpers;
use helpers::*;
use vizia::prelude::*;

// TODO: XYPad needs to be migrated to Signal architecture
// For now this example uses signals for sliders but XYPad still uses Lens

#[derive(Debug, Lens)]
pub struct AppData {
    pub xy_data: (f32, f32),
}

#[derive(Debug)]
pub enum AppEvent {
    XYPadChange(f32, f32),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::XYPadChange(value_x, value_y) => {
                self.xy_data = (*value_x, *value_y);
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { xy_data: (0.25, 0.25) }.build(cx);

        let x_value = cx.state(0.25f32);
        let y_value = cx.state(0.25f32);

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
                            y_value.set(cx, val);
                            let x = *x_value.get(cx);
                            cx.emit(AppEvent::XYPadChange(x, val));
                        });
                    // XY pad (still uses Lens - TODO: migrate XYPad)
                    XYPad::new(cx, AppData::xy_data.map(|data| (data.0, data.1))).on_change(
                        move |ex, value_x, value_y| {
                            x_value.set(ex, value_x);
                            y_value.set(ex, value_y);
                            ex.emit(AppEvent::XYPadChange(value_x, value_y));
                        },
                    );
                })
                .size(Auto)
                .horizontal_gap(Pixels(5.0))
                .alignment(Alignment::Center);
                Slider::new(cx, x_value)
                    .width(Pixels(100.0))
                    .height(Pixels(10.0))
                    .range(0.0..1.0)
                    .on_change(move |cx, val| {
                        x_value.set(cx, val);
                        let y = *y_value.get(cx);
                        cx.emit(AppEvent::XYPadChange(val, y));
                    });
            })
            .alignment(Alignment::Center);
        });
    })
    .title("XY Pad")
    .run()
}

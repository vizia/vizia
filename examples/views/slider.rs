mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Debug, Lens)]
pub struct AppData {
    value: f32,
}

pub enum AppEvent {
    SetValue(f32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetValue(val) => {
                self.value = *val;
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { value: 0.0 }.build(cx);

        ExamplePage::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Slider::new(cx, AppData::value.map(|val| (val + 50.0) / 100.0))
                    .range(0.0..1.0)
                    .on_change(move |cx, val| cx.emit(AppEvent::SetValue(-50.0 + (val * 100.0))));
                Label::new(cx, AppData::value.map(|val| format!("{:.2}", (val + 50.0) / 100.0)))
                    .width(Pixels(50.0));
            })
            .alignment(Alignment::Center)
            .height(Auto)
            .horizontal_gap(Pixels(8.0));

            HStack::new(cx, |cx| {
                Slider::new(cx, AppData::value)
                    .range(-50.0..50.0)
                    .on_change(move |cx, val| cx.emit(AppEvent::SetValue(val)));
                Label::new(cx, AppData::value.map(|val| format!("{:.2}", val))).width(Pixels(50.0));
            })
            .alignment(Alignment::Center)
            .height(Auto)
            .horizontal_gap(Pixels(8.0));

            VStack::new(cx, |cx| {
                Slider::new(cx, AppData::value)
                    .range(-50.0..50.0)
                    .on_change(move |cx, val| cx.emit(AppEvent::SetValue(val)))
                    .class("vertical");
                Label::new(cx, AppData::value.map(|val| format!("{:.2}", val)))
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

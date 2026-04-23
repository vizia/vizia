mod helpers;
use helpers::*;
use vizia::prelude::*;

pub struct AppData {
    value: Signal<f32>,
}

pub enum AppEvent {
    SetValue(f32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetValue(val) => {
                self.value.set(*val);
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let value = Signal::new(0.0);
        let normalized_value = Memo::new(move |_| (value.get() + 50.0) / 100.0);
        let normalized_text = Memo::new(move |_| format!("{:.2}", normalized_value.get()));
        let value_text = Memo::new(move |_| format!("{:.2}", value.get()));

        AppData { value }.build(cx);

        ExamplePage::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Slider::new(cx, normalized_value).range(0.0..1.0).on_change(
                    move |cx: &mut EventContext, val| {
                        cx.emit(AppEvent::SetValue(-50.0 + (val * 100.0)))
                    },
                );
                Label::new(cx, normalized_text).width(Pixels(50.0));
            })
            .alignment(Alignment::Center)
            .height(Auto)
            .horizontal_gap(Pixels(8.0));

            HStack::new(cx, |cx| {
                Slider::new(cx, value)
                    .range(-50.0..50.0)
                    .on_change(move |cx, val| cx.emit(AppEvent::SetValue(val)));
                Label::new(cx, value_text).width(Pixels(50.0));
            })
            .alignment(Alignment::Center)
            .height(Auto)
            .horizontal_gap(Pixels(8.0));

            VStack::new(cx, |cx| {
                Slider::new(cx, value)
                    .range(-50.0..50.0)
                    .on_change(move |cx, val| cx.emit(AppEvent::SetValue(val)))
                    .vertical(true);
                Label::new(cx, value_text).alignment(Alignment::Center).width(Pixels(50.0));
            })
            .alignment(Alignment::Center)
            .vertical_gap(Pixels(8.0));
        });
    })
    .title(Localized::new("view-title-slider"))
    .run()
}

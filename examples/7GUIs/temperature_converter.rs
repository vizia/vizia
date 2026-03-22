use vizia::prelude::*;

pub struct AppData {
    temperature: Signal<f32>,
}

pub enum AppEvent {
    SetTemperature(f32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetTemperature(temp) => self.temperature.set(*temp),
        });
    }
}

fn input_box<R>(cx: &mut Context, value: R, convert: impl Fn(f32) -> f32 + Send + Sync + 'static)
where
    R: Res<String> + Clone + 'static,
{
    Textbox::new(cx, value)
        .on_edit(move |ex, text| {
            if let Ok(val) = text.parse() {
                ex.emit(AppEvent::SetTemperature(convert(val)));
            }
        })
        .width(Stretch(1.0));
}

fn celcius_to_fahrenheit(temp: &f32) -> f32 {
    *temp * (9. / 5.) + 32.
}

fn fahrenheit_to_celcius(temp: f32) -> f32 {
    (temp - 32.) * (5. / 9.)
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let temperature = Signal::new(5.0);
        let celsius = Memo::new(move |_| format!("{:.0}", temperature.get()));
        let fahrenheit =
            Memo::new(move |_| format!("{:.0}", celcius_to_fahrenheit(&temperature.get())));

        AppData { temperature }.build(cx);

        HStack::new(cx, |cx| {
            input_box(cx, celsius, |val| val);
            Label::new(cx, "Celsius");
            input_box(cx, fahrenheit, fahrenheit_to_celcius);
            Label::new(cx, "Fahrenheit");
        })
        .alignment(Alignment::Center)
        .horizontal_gap(Pixels(10.0));
    })
    .title("Temperature Converter")
    .inner_size((450, 100))
    .run()
}

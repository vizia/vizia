use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    temperature: f32,
}

pub enum AppEvent {
    SetTemperature(f32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetTemperature(temp) => self.temperature = *temp,
        });
    }
}

fn input_box<L: Lens<Target = f32>>(
    cx: &mut Context,
    date_lens: L,
    convert: impl Fn(f32) -> f32 + Send + Sync + 'static,
) {
    Textbox::new(cx, date_lens.map(|num| format!("{:.0}", num)))
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

fn main() {
    Application::new(|cx| {
        AppData { temperature: 5.0 }.build(cx);

        HStack::new(cx, |cx| {
            input_box(cx, AppData::temperature, |val| val);
            Label::new(cx, "Celcius");
            input_box(cx, AppData::temperature.map(celcius_to_fahrenheit), fahrenheit_to_celcius);
            Label::new(cx, "Fahrenheit");
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(10.0));
    })
    .title("Temperature Converter")
    .inner_size((450, 100))
    .run();
}

use vizia::*;

const STYLE: &str = r#"
    textbox {
        width: 100px;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    temperature_celcius: f32,
    temperature_fahrenheit: f32,
}

pub enum AppEvent {
    SetTemperatureCelcius(f32),
    SetTemperatureFahrenheit(f32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetTemperatureCelcius(temp) => {
                self.temperature_celcius = *temp;
                self.temperature_fahrenheit = self.temperature_celcius * (9.0 / 5.0) + 32.0;
            }

            AppEvent::SetTemperatureFahrenheit(temp) => {
                self.temperature_fahrenheit = *temp;
                self.temperature_celcius = (self.temperature_fahrenheit - 32.0) * (5.0 / 9.0);
            }
        });
    }
}

fn main() {
    let window_description =
        WindowDescription::new().with_title("Temperature Converter").with_inner_size(450, 100);
    Application::new(window_description, |cx| {
        cx.add_theme(STYLE);

        AppData { temperature_celcius: 5.0, temperature_fahrenheit: 41.0 }.build(cx);

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::temperature_celcius).on_edit(|cx, text| {
                if let Ok(val) = text.parse::<f32>() {
                    cx.emit(AppEvent::SetTemperatureCelcius(val));
                }
            });
            Label::new(cx, "Celcius");
            Textbox::new(cx, AppData::temperature_fahrenheit).on_edit(|cx, text| {
                if let Ok(val) = text.parse::<f32>() {
                    cx.emit(AppEvent::SetTemperatureFahrenheit(val));
                }
            });
            Label::new(cx, "Fahrenheit");
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(20.0));
    })
    .run();
}

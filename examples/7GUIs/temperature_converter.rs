use vizia::prelude::*;

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    temp * (9. / 5.) + 32.
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    (temp - 32.) * (5. / 9.)
}

struct TemperatureConverterApp {
    celsius: Signal<f32>,
    fahrenheit: Signal<f32>,
}

impl App for TemperatureConverterApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            celsius: cx.state(5.0f32),
            fahrenheit: cx.state(celsius_to_fahrenheit(5.0)),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let celsius = self.celsius;
        let fahrenheit = self.fahrenheit;

        HStack::new(cx, |cx| {
            Textbox::new(cx, celsius)
                .on_submit(move |cx, val, _| {
                    fahrenheit.set(cx, celsius_to_fahrenheit(val));
                })
                .width(Stretch(1.0));
            Label::new(cx, "Celsius");

            Textbox::new(cx, fahrenheit)
                .on_submit(move |cx, val, _| {
                    celsius.set(cx, fahrenheit_to_celsius(val));
                })
                .width(Stretch(1.0));
            Label::new(cx, "Fahrenheit");
        })
        .alignment(Alignment::Center)
        .horizontal_gap(Pixels(10.0));

        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((450, 100)))
    }
}

fn main() -> Result<(), ApplicationError> {
    TemperatureConverterApp::run()
}

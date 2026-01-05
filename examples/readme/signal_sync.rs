use vizia::prelude::*;

fn celsius_to_fahrenheit(c: f32) -> f32 {
    c * 9.0 / 5.0 + 32.0
}
fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

struct SignalSyncApp {
    celsius: Signal<f32>,
    fahrenheit: Signal<f32>,
    title: Signal<&'static str>,
    size: Signal<(u32, u32)>,
}

impl App for SignalSyncApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            celsius: cx.state(20.0f32),
            fahrenheit: cx.state(celsius_to_fahrenheit(20.0)),
            title: cx.state("Temperature Converter"),
            size: cx.state((400, 100)),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let celsius = self.celsius;
        let fahrenheit = self.fahrenheit;

        HStack::new(cx, |cx| {
            Textbox::new(cx, celsius).on_submit(move |cx, val, _| {
                fahrenheit.set(cx, celsius_to_fahrenheit(val));
            });
            Label::new(cx, "C");

            Textbox::new(cx, fahrenheit).on_submit(move |cx, val, _| {
                celsius.set(cx, fahrenheit_to_celsius(val));
            });
            Label::new(cx, "F");
        });

        self
    }

    fn window_config(&self) -> WindowConfig {
        let title = self.title;
        let size = self.size;
        window(move |app| app.title(title).inner_size(size))
    }
}

fn main() -> Result<(), ApplicationError> {
    SignalSyncApp::run()
}

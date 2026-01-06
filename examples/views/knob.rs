mod helpers;
use helpers::*;
use vizia::prelude::*;

struct KnobApp {
    value: Signal<f32>,
}

impl App for KnobApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            value: cx.state(0.2f32),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let value = self.value;
        ExamplePage::new(cx, move |cx| {
            Knob::new(cx, 0.5, value, false).two_way();
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Knob").inner_size((300, 300)))
    }
}

fn main() -> Result<(), ApplicationError> {
    KnobApp::run()
}

mod helpers;
use helpers::*;
use vizia::prelude::*;

const COLORS: [Color; 3] = [Color::red(), Color::green(), Color::blue()];

fn main() -> Result<(), ApplicationError> {
    HStackApp::run()
}

struct HStackApp;

impl App for HStackApp {
    fn new(_cx: &mut Context) -> Self {
        Self
    }

    fn on_build(self, cx: &mut Context) -> Self {
        ExamplePage::new(cx, |cx| {
            HStack::new(cx, |cx| {
                for color in COLORS {
                    Element::new(cx).size(Pixels(100.0)).background_color(color);
                }
            })
            .alignment(Alignment::Center);
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("HStack"))
    }
}

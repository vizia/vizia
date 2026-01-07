use vizia::prelude::*;

const COLORS: [Color; 3] = [Color::red(), Color::green(), Color::blue()];

struct VStackApp;

impl App for VStackApp {
    fn app_name() -> &'static str {
        "VStack"
    }

    fn new(_cx: &mut Context) -> Self {
        Self
    }

    fn on_build(self, cx: &mut Context) -> Self {
        VStack::new(cx, |cx| {
            for color in COLORS {
                Element::new(cx).size(Pixels(100.0)).background_color(color);
            }
        })
        .alignment(Alignment::Center);
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app)
    }
}

fn main() -> Result<(), ApplicationError> {
    VStackApp::run()
}

use vizia::prelude::*;

const COLORS: [Color; 3] = [Color::red(), Color::green(), Color::blue()];

struct ZStackApp;

impl App for ZStackApp {
    fn app_name() -> &'static str {
        "ZStack"
    }

    fn new(_cx: &mut Context) -> Self {
        Self
    }

    fn on_build(self, cx: &mut Context) -> Self {
        ZStack::new(cx, |cx| {
            for (i, color) in COLORS.into_iter().enumerate() {
                Element::new(cx).size(Pixels(100.0)).translate(Pixels(10.0 * i as f32)).background_color(color);
            }
        })
        .alignment(Alignment::Center);
        self
    }
}

fn main() -> Result<(), ApplicationError> {
    ZStackApp::run()
}

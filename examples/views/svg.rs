mod helpers;
use helpers::*;
use vizia::prelude::*;

struct SvgApp;

impl App for SvgApp {
    fn new(_cx: &mut Context) -> Self {
        Self
    }

    fn on_build(self, cx: &mut Context) -> Self {
        ExamplePage::new(cx, |cx| {
            Svg::new(cx, include_bytes!("../resources/images/Ghostscript_Tiger.svg").to_vec())
                .size(Stretch(1.0))
                .border_color(Color::black())
                .border_width(Pixels(1.0));

            Svg::new(cx, r#"<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
                <circle cx="50" cy="50" r="40" stroke="green" stroke-width="4" fill="yellow" />
                </svg>"#)
                .size(Pixels(100.0))
                .border_color(Color::black())
                .border_width(Pixels(1.0));

            Svg::new(cx, r#"<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
                <circle cx="50" cy="50" r="30" stroke="red" stroke-width="2" fill="blue" />
                </svg>"#
                .as_bytes()
                .to_vec())
                .size(Pixels(100.0))
                .border_color(Color::black())
                .border_width(Pixels(1.0));
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((400, 200)))
    }
}

fn main() -> Result<(), ApplicationError> {
    SvgApp::run()
}

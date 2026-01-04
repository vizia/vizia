#[allow(unused)]
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    StylesheetApp::run()
}

struct StylesheetApp {
    size_200: Signal<Units>,
}

impl App for StylesheetApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            size_200: cx.state(Pixels(200.0)),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        cx.add_stylesheet(include_style!("examples/resources/themes/test.css"))
            .expect("Failed to add stylesheet");
        let size_200 = self.size_200;
        HStack::new(cx, |cx| {
            Element::new(cx).class("foo");
        })
        .size(size_200);
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Stylesheet"))
    }
}

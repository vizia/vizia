mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    DividerApp::run()
}

struct DividerApp;

impl App for DividerApp {
    fn new(_cx: &mut Context) -> Self {
        Self
    }

    fn on_build(self, cx: &mut Context) -> Self {
        ExamplePage::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Divider::new(cx);
                Divider::horizontal(cx);
                Divider::vertical(cx);
            });
            VStack::new(cx, |cx| {
                Divider::new(cx);
                Divider::horizontal(cx);
                Divider::vertical(cx);
            });
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Divider").inner_size((350, 300)))
    }
}

use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

#[cfg(not(feature = "baseview"))]
struct WindowModifiersApp {
    title: Signal<String>,
}

#[cfg(not(feature = "baseview"))]
impl App for WindowModifiersApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            title: cx.state("Window Modifiers".to_string()),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let title = self.title;
        VStack::new(cx, |cx| {
            Label::new(cx, "Window title:");
            Textbox::new(cx, title).on_edit(move |cx, txt| title.set(cx, txt)).width(Stretch(1.0));
        })
        .padding(Pixels(8.0))
        .gap(Pixels(8.0));
        self
    }

    fn window_config(&self) -> WindowConfig {
        let title = self.title;
        window(move |app| app.title(title).inner_size((400, 100)))
    }
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    WindowModifiersApp::run()
}

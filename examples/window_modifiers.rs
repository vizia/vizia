use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

#[derive(Lens)]
pub struct AppData {
    title: String,
}

impl Model for AppData {}

#[cfg(all(not(feature = "baseview")))]
fn main() {
    Application::new(|cx| {
        AppData { title: "Window Modifiers".to_string() }.build(cx);

        Label::new(cx, "Hello Vizia");
        Textbox::new(cx, AppData::title)
            .on_edit(|ex, txt| ex.emit(WindowEvent::SetTitle(txt)))
            .width(Stretch(1.0));
    })
    .title("Window Modifiers")
    .inner_size((400, 100))
    .run();
}

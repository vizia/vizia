use vizia::prelude::*;

mod app_data;
pub use app_data::*;

fn main() {
    Application::new(|cx| {
        Label::new(cx, "Hello World");
        Textbox::new(cx, AppData::text);
    })
    .run();
}

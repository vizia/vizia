use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    value: f32,
}

impl Model for AppData {}

fn main() {
    Application::new(|cx| {
        AppData { value: 3.14 }.build(cx);

        Label::new(cx, AppData::value.map(|_val| String::from("Hello World")));
    })
    .title("Lens Map")
    .run();
}

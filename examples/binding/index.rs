use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    pub data: Vec<String>,
}

impl Model for AppData {}

fn main() {
    Application::new(|cx| {
        AppData {
            data: vec![String::from("First"), String::from("Second"), String::from("Third")],
        }
        .build(cx);

        Label::new(cx, AppData::data.index(1));
    })
    .run();
}

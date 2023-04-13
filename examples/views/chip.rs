mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppData {
    chip1: String,
    chip2: String,
}

fn main() {
    Application::new(|cx| {
        AppData { chip1: "Chip".to_string(), chip2: "Another Chip".to_string() }.build(cx);

        ExamplePage::new(cx, |cx| {
            Chip::new(cx, AppData::chip1).background_color(Color::from("#00ffff44"));
            Chip::new(cx, AppData::chip2).background_color(Color::from("#ff004444"));
        });
    })
    .title("Chip")
    .inner_size((400, 200))
    .run();
}

impl Model for AppData {}

mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
}

impl Model for AppData {}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(include_str!("../resources/themes/list_style.css"))
            .expect("Failed to add stylesheet");

        let list: Vec<u32> = (10..14u32).collect();
        AppData { list }.build(cx);

        ExamplePage::new(cx, |cx| {
            List::new(cx, AppData::list, |cx, _, item| {
                Label::new(cx, item);
            });
        });
    })
    .title("List")
    .run();
}

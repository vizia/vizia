mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
}

impl Model for AppData {}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let list: Vec<u32> = (0..14u32).collect();
        AppData { list }.build(cx);

        ExamplePage::new(cx, |cx| {
            List::new(cx, AppData::list, |cx, _, item| {
                Label::new(cx, item).hoverable(false).width(Pixels(100.0)).height(Pixels(30.0));
            });
        });
    })
    .title("List")
    .run()
}

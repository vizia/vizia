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
        AppData { list: (1..1000u32).collect() }.build(cx);

        ExamplePage::new(cx, |cx| {
            VirtualList::new(cx, AppData::list, 40.0, |cx, index, item| {
                Label::new(cx, item).toggle_class("dark", index % 2 == 0)
            })
            .size(Pixels(300.0));
        });
    })
    .title("Virtual List")
    .run()
}

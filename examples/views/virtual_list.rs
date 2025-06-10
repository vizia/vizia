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
        AppData { list: (1..100u32).collect() }.build(cx);

        ExamplePage::new(cx, |cx| {
            VirtualList::new(cx, AppData::list, 40.0, |cx, index, item| {
                Label::new(cx, item).toggle_class("dark", index % 2 == 0).hoverable(false)
            })
            .size(Pixels(300.0))
            .selectable(Selectable::Single)
            .selection_follows_focus(true);
        });
    })
    .title("Virtual List")
    .run()
}

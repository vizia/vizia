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
        let list: Vec<u32> = (10..14u32).collect();
        AppData { list }.build(cx);

        ExamplePage::new(cx, |cx| {
            List::new(cx, AppData::list, |cx, _, item| {
                Label::new(cx, item).hoverable(false).width(Pixels(100.0)).height(Pixels(30.0));
            })
            .selectable(Selectable::Single);
            List::new(cx, AppData::list, |cx, _, item| {
                Label::new(cx, item).hoverable(false).width(Pixels(100.0)).height(Pixels(30.0));
            })
            .selectable(Selectable::Multi);
        });
    })
    .title("List")
    .run();
}

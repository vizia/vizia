use vizia::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
}

impl Model for AppData {}

fn main() {
    Application::new(WindowDescription::new().with_title("List"), |cx| {
        cx.add_stylesheet("examples/lists/list_style.css").unwrap();

        let list: Vec<u32> = (10..14u32).collect();
        AppData { list }.build(cx);

        List::new(cx, AppData::list, |cx, item| {
            let item_text = item.get(cx).to_string();
            Label::new(cx, &item_text);
        }); // Center the list view in the window
    })
    .run();
}

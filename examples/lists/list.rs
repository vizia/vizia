use vizia::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
}

impl Model for AppData {}

fn main() {
    Application::new(WindowDescription::new().with_title("List"), |cx| {
        let list: Vec<u32> = (10..14u32).collect();
        AppData { list }.build(cx);

        List::new(cx, AppData::list, |cx, item| {
            let item_text = item.get(cx).to_string();
            Label::new(cx, &item_text)
                .width(Pixels(100.0))
                .height(Pixels(30.0))
                .border_color(Color::black())
                .border_width(Pixels(1.0));
        })
        .row_between(Pixels(5.0)) // 5px space between list items
        .space(Stretch(1.0)); // Center the list view in the window
    })
    .run();
}

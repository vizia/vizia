use vizia::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
}

impl Model for AppData {}

fn main() {
    Application::new(WindowDescription::new().with_title("List"), |cx| {
        cx.add_theme(include_str!("../lists/list_style.css"));

        let list: Vec<u32> = (10..14u32).collect();
        AppData { list }.build(cx);

        List::new(cx, AppData::list, |cx, _, item| {
            Label::new(cx, item);
        })
        .space(Stretch(1.0));
    })
    .run();
}

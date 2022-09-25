use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<&'static str>,
}

impl Model for AppData {}

fn main() {
    Application::new(|cx| {
        AppData { list: vec!["Tab1", "Tab2"] }.build(cx);

        TabView::new(cx, AppData::list, |cx, item| match item.get(cx) {
            "Tab1" => TabPair::new(
                move |cx| {
                    Label::new(cx, item.clone()).background_color(Color::rgb(200, 200, 200));
                },
                |cx| {
                    Element::new(cx).size(Pixels(100.0)).background_color(Color::red());
                },
            ),

            "Tab2" => TabPair::new(
                move |cx| {
                    Label::new(cx, item.clone()).background_color(Color::rgb(200, 200, 200));
                },
                |cx| {
                    Element::new(cx).size(Pixels(200.0)).background_color(Color::blue());
                },
            ),

            _ => TabPair::new(|_| {}, |_| {}),
        });
    })
    .title("List")
    .run();
}

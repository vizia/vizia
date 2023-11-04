mod helpers;
pub use helpers::*;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<&'static str>,
}

impl Model for AppData {}

fn main() {
    Application::new(|cx| {
        AppData { list: vec!["Tab1", "Tab2"] }.build(cx);

        ExamplePage::new(cx, |cx| {
            TabView::new(cx, AppData::list, |cx, item| match item.get(cx) {
                "Tab1" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).hoverable(false);
                        Element::new(cx).class("indicator");
                    },
                    |cx| {
                        Element::new(cx).size(Pixels(200.0)).background_color(Color::red());
                    },
                ),

                "Tab2" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).hoverable(false);
                        Element::new(cx).class("indicator");
                    },
                    |cx| {
                        Element::new(cx).size(Pixels(200.0)).background_color(Color::blue());
                    },
                ),

                _ => unreachable!(),
            })
            .width(Pixels(500.0))
            .height(Pixels(300.0));
        });
    })
    .title("Tabs")
    .run();
}

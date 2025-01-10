mod helpers;
pub use helpers::*;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    tabs: Vec<&'static str>,
}

impl Model for AppData {}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { tabs: vec!["Tab1", "Tab2", "Tab3", "Tab4", "Tab5", "Tab6"] }.build(cx);

        ExamplePage::new(cx, |cx| {
            TabView::new(cx, AppData::tabs, |cx, item| match item.get(cx) {
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
    .title("Tabview")
    .run()
}

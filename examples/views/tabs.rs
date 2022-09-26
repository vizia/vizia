use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<&'static str>,
}

impl Model for AppData {}

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        AppData { list: vec!["Tab1", "Tab2"] }.build(cx);

        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        VStack::new(cx, |cx| {
            TabView::new(cx, AppData::list, |cx, item| match item.get(cx) {
                "Tab1" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item.clone());
                    },
                    |cx| {
                        Element::new(cx).size(Pixels(100.0)).background_color(Color::red());
                    },
                ),

                "Tab2" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item.clone());
                    },
                    |cx| {
                        Element::new(cx).size(Pixels(200.0)).background_color(Color::blue());
                    },
                ),

                _ => TabPair::new(|_| {}, |_| {}),
            });
        })
        .class("main")
        .width(Units::Stretch(1.0))
        .height(Units::Stretch(1.0));
    })
    .ignore_default_theme()
    .title("Tabs")
    .run();
}

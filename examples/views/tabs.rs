use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<&'static str>,
}

impl Model for AppData {}

const CENTER_LAYOUT: &str = "crates/vizia_core/resources/themes/center_layout.css";
#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        AppData { list: vec!["Tab1", "Tab2"] }.build(cx);

        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to find stylesheet");
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        TabView::new(cx, AppData::list, |cx, item| match item.get(cx) {
            "Tab1" => TabPair::new(
                move |cx| {
                    Label::new(cx, item);
                },
                |cx| {
                    Element::new(cx).size(Pixels(200.0)).background_color(Color::red());
                },
            ),

            "Tab2" => TabPair::new(
                move |cx| {
                    Label::new(cx, item);
                },
                |cx| {
                    Element::new(cx).size(Pixels(200.0)).background_color(Color::blue());
                },
            ),

            _ => TabPair::new(|_| {}, |_| {}),
        })
        .size(Auto);
    })
    .ignore_default_theme()
    .title("Tabs")
    .run();
}

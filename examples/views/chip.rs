use vizia::prelude::*;

const CENTER_LAYOUT: &str = "crates/vizia_core/resources/themes/center_layout.css";
#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

#[derive(Clone, Lens)]
struct AppData {
    chip1: String,
    chip2: String,
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to find stylesheet");
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        AppData { chip1: "Chip".to_string(), chip2: "Another Chip".to_string() }.build(cx);

        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Chip::new(cx, AppData::chip1).background_color(Color::from("#00ffff44"));
                Chip::new(cx, AppData::chip2).background_color(Color::from("#ff004444"));
            })
            .class("container");
        })
        .class("container");
    })
    .ignore_default_theme()
    .title("Chip")
    .run();
}

impl Model for AppData {}

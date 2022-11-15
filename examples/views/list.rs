use vizia::prelude::*;

const CENTER_LAYOUT: &str = "crates/vizia_core/resources/themes/center_layout.css";
#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
}

impl Model for AppData {}

fn main() {
    Application::new(|cx| {
        cx.add_theme(include_str!("../resources/list_style.css"));
        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to find stylesheet");
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        let list: Vec<u32> = (10..14u32).collect();
        AppData { list }.build(cx);

        List::new(cx, AppData::list, |cx, _, item| {
            Label::new(cx, item);
        });
    })
    .ignore_default_theme()
    .title("List")
    .run();
}

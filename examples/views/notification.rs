use vizia::prelude::*;

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        Notification::new(
            cx,
            "Notification Title".to_string(),
            Some("This is some information about the notification you just got!".to_string()),
        );
    })
    .ignore_default_theme()
    .title("Popup")
    .run();
}

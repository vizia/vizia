use vizia::prelude::*;

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        PopupData::default().build(cx);

        Button::new(cx, |cx| cx.emit(PopupEvent::Switch), |cx| Label::new(cx, "Open"));

        Popup::new(cx, PopupData::is_open, true, |_| {})
            .on_blur(|cx| cx.emit(PopupEvent::Close))
            .space(Pixels(100.0))
            .size(Pixels(200.0))
            .background_color(Color::red());
    })
    .ignore_default_theme()
    .title("Popup")
    .run();
}

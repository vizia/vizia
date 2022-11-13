use vizia::prelude::*;

const CENTER_LAYOUT: &str = "crates/vizia_core/resources/themes/center_layout.css";
#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to find stylesheet");
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        Tooltip::new(cx, "Tooltip here!", |cx| {
            Label::new(cx, "Subtitle").class("subtitle");
            Label::new(cx, "Very serious tooltip explanation here.");
        })
        .on_ok(|_| println!("Ok!"));

        TooltipSeq::new(cx, "Tooltip here!", |cx| {
            Label::new(cx, "Subtitle").class("subtitle");
            Label::new(cx, "Very serious tooltip explanation here.");
        })
        .on_next(|_| println!("Next!"))
        .on_prev(|_| println!("Prev!"));
    })
    .ignore_default_theme()
    .title("Tooltip")
    .run();
}

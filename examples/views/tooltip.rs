use vizia::prelude::*;

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        VStack::new(cx, |cx| {
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
        .class("container");
    })
    .ignore_default_theme()
    .title("Tooltip")
    .run();
}

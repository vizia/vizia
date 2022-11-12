use vizia::fonts::unicode_names::CHECK;
use vizia::prelude::*;

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        VStack::new(cx, |cx| {
            // Basic Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Button"));
            // Accent Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Another Button")).class("accent");
            // Disabled Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Disabled Button")).disabled(true);
            // Button with Icon
            Button::new(
                cx,
                |_| {},
                |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, CHECK).class("icon");
                        Label::new(cx, "Button");
                    })
                    .size(Auto)
                    .child_space(Stretch(1.0))
                    .col_between(Pixels(2.0))
                },
            );
        })
        .class("container");
    })
    .ignore_default_theme()
    .title("Button")
    .run();
}

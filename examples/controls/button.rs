use std::process::Output;

use vizia::*;

const ICON_PLUS: &str = "\u{2b}";
// const ICON_CHECK: &str = "\u{2713}";

fn no_action(cx: &mut Context) {}

fn main() {
    let window_description =
        WindowDescription::new().with_title("Button").with_inner_size(1280, 720);

    Application::new(window_description, |cx| {
        HStack::new(cx, |cx| {
            // Filled button
            Button::new(cx, no_action, |cx| Label::new(cx, "Button"));
            // Outline button
            Button::new(cx, no_action, |cx| Label::new(cx, "Another Button")).class("outlined");
            // Text button
            Button::new(cx, no_action, |cx| Label::new(cx, "Yet Another Button")).class("text");

            // Filled button with icon
            Button::new(cx, no_action, |cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, ICON_PLUS).class("icon");
                    Label::new(cx, "Button");
                })
                .size(Auto)
                .child_space(Stretch(1.0))
                .col_between(Pixels(2.0))
            });
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(10.0));
    })
    .run();
}

use vizia::fonts::icons_names::CHECK;
use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        HStack::new(cx, |cx| {
            // Basic Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Button"));
            // Accent Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Another Button")).class("accent");
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
        .size(Auto)
        .space(Stretch(1.0))
        .col_between(Pixels(10.0));
    })
    .title("Button")
    .run();
}

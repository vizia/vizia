mod helpers;
use helpers::*;
use vizia::fonts::unicode_names::CHECK;
use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        view_controls(cx);

        HStack::new(cx, |cx| {
            // Basic Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Button"));
            // Accent Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Accent Button")).class("accent");
            // Outline Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Outline Button")).class("outline");
            // Ghost Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Ghost Button")).class("ghost");
            // Button with Icon
            Button::new(
                cx,
                |_| {},
                |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, CHECK).class("icon");
                        Label::new(cx, "Button with Icon");
                    })
                    .size(Auto)
                    .child_space(Stretch(1.0))
                    .col_between(Pixels(4.0))
                },
            );
        })
        .disabled(ControlsData::disabled)
        .class("container");
    })
    .title("Button")
    .inner_size((700, 200))
    .run();
}

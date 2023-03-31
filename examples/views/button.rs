use vizia::icons::ICON_CHECK;
use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        HStack::new(cx, |cx| {
            // Basic Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Simple Button"));
            // Accent Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Accent Button")).class("accent");
            // Button with Icon
            Button::new(
                cx,
                |_| {},
                |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "\u{eabd}").class("icon");
                        Label::new(cx, "Button with Icon");
                    })
                    .size(Auto)
                    .child_space(Stretch(1.0))
                    .col_between(Pixels(2.0))
                },
            );

            Button::new(
                cx,
                |_| {},
                |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, ICON_CHECK).class("icon");
                        // Hidden from layout and display but used for accessibility
                        Label::new(cx, "Icon Button").display(Display::None);
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

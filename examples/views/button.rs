mod helpers;
use helpers::*;

use vizia::icons::ICON_CHECK;
use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        ExamplePage::new(cx, |cx| {
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
                        Label::new(cx, ICON_CHECK).class("icon");
                        Label::new(cx, "Button with Icon");
                    })
                },
            );
        });
    })
    .title("Button")
    .inner_size((700, 200))
    .run();
}

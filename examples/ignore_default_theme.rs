use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        Button::new(cx, |_| {}, |cx| Label::new(cx, "Button"));
    })
    .ignore_default_theme()
    .run();
}

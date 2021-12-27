use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("External CSS"), |cx| {
        cx.add_stylesheet("examples/resources/test.css").expect("Failed to find file");

        Button::new(cx, |_| {}, |cx| Label::new(cx, "Button"));
    })
    .run();
}

use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Basic"), |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "Hello");
            Label::new(cx, "World");
        });
    })
    .run();
}

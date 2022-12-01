use vizia::prelude::*;
fn main() {
    Application::new(|cx| {
        Label::new(cx, "Hello");
        Label::new(cx, "World");
    })
    .title("Stylesheet")
    .run();
}

use vizia::prelude::*;
fn main() {
    Application::new(|cx| {
        Label::new(cx, "Hello");
        Label::new(cx, "World");
        Label::new(cx, "This vizia application is accessible thanks to Accesskit");
    })
    .title("AccessKit")
    .run();
}

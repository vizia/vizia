use vizia::*;

fn main() {
    Application::new(
        WindowDescription::new().with_title("Basic").with_inner_size(1000, 800),
        |cx| {
            HStack::new(cx, |cx| {
                Label::new(cx, "Hello");
                Label::new(cx, "World");
            });
        },
    )
    .run();
}

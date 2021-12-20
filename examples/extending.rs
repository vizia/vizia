use vizia::*;

// Example of extending the behaviour of a view
fn main() {
    Application::new(WindowDescription::new().with_title("Extending"), |cx| {
        Button::new(cx, |_| println!("Pressed"), |cx| Label::new(cx, "Press Me!"))
            .on_hover(cx, |_| println!("Hover"));
    })
    .run();
}

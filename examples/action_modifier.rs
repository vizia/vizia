use vizia::*;

// Example of extending the behaviour of a view
fn main() {
    Application::new(WindowDescription::new().with_title("Extending"), |cx| {
        Label::new(cx, "Press Me!").on_press(|_| println!("You pressed a label!"));
    })
    .run();
}

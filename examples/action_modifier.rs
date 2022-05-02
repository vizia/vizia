use vizia::prelude::*;

// Example of extending the behaviour of a view
fn main() {
    Application::new(|cx| {
        Label::new(cx, "Press Me!").on_press(|_| println!("You pressed a label!"));
    })
    .title("Action Modifiers")
    .run();
}

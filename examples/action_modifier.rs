use vizia::prelude::*;

// Example of extending the behaviour of a view
fn main() {
    Application::new(|cx| {
        Label::new(cx, "Press on me!").on_press(|_| println!("You pressed on a label!"));
        Label::new(cx, "Release on me!").on_release(|_| println!("You released on a label!"));
        Label::new(cx, "Hover on me!").on_hover(|_| println!("You hovered a label!"));
        Label::new(cx, "Right click on me!").on_mouse_down(|_, button| {
            if button == MouseButton::Right {
                println!("You right clicked on a label!")
            }
        });
    })
    .title("Action Modifiers")
    .run();
}

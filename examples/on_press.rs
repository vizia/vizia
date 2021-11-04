
use vizia::*;

// An example of triggering a callback when a button is pressed

fn main() {

    Application::new(|cx|{
        VStack::new(cx, |cx| {
            Button::new(cx, |_| {println!("Pressed!")}, |cx|{
                Label::new(cx, "Press Me!");
            });
        });
    }).run();
}
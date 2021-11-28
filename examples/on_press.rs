
use vizia::*;

// An example of triggering a callback when a button is pressed

fn main() {

    Application::new(WindowDescription::new().with_title("On Press"), |cx|{
        VStack::new(cx, |cx| {
            Button::new(cx, |_| {println!("Pressed!")}, |cx|{
                Label::new(cx, "Press Me!");
            });
        });
    }).run();
}
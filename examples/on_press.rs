
use vizia::*;

fn main() {

    Application::new(|cx|{
        VStack::new().build(cx, |cx| {
            Button::new(|_| {println!("Pressed!")}).build(cx, |cx|{
                Label::new("Press Me!").build(cx);
            });
        });
    }).run();
}
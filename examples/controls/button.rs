use vizia::*;

fn main() {
    Application::new(|cx|{
        Button::new(cx, |_|{}, |_|{}).left(Pixels(50.0));
    }).run();
}
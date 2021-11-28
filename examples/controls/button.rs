use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Button"), |cx|{
        Button::new(cx, |_|{}, |_|{}).left(Pixels(50.0));
    }).run();
}
use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Checkbox"), |cx|{
        Checkbox::new(cx, false).left(Pixels(50.0));
    }).run();
}
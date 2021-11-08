use vizia::*;

fn main() {
    Application::new(|cx|{
        Checkbox::new(cx, false).left(Pixels(50.0));
    }).run();
}
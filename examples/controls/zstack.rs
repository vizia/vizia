use vizia::*;

fn main() {
    Application::new(|cx|{
        ZStack::new(cx, |cx|{
            Element::new(cx).size(Pixels(100.0)).top(Pixels(10.0)).left(Pixels(10.0)).background_color(Color::red());
            Element::new(cx).size(Pixels(100.0)).top(Pixels(20.0)).left(Pixels(20.0)).background_color(Color::green());
            Element::new(cx).size(Pixels(100.0)).top(Pixels(30.0)).left(Pixels(30.0)).background_color(Color::blue());
        });
    }).run();
}

use vizia::*;

fn main() {

    Application::new(|cx|{
VStack::new()
    .width(Pixels(500.0))
    .height(Pixels(500.0))
    .build(cx, |cx| {
        Label::new("Label 1")
            .width(Pixels(100.0))
            .height(Pixels(30.0))
            .background_color(Color::blue()).build(cx);
        Label::new("Label 2")
            .width(Pixels(200.0))
            .height(Pixels(50.0))
            .background_color(Color::green()).build(cx);
    });
    }).run();
}

use vizia::*;

// Example showing inline styling of views

fn main() {

    Application::new(|cx|{
        VStack::new(cx, |cx| {
            Label::new(cx, "Label 1")
                .width(Pixels(100.0))
                .height(Pixels(30.0))
                .background_color(Color::blue());

            Label::new(cx, "Label 2")
                .width(Pixels(200.0))
                .height(Pixels(50.0))
                .background_color(Color::green());
                
        }).width(Pixels(500.0)).height(Pixels(500.0));
    }).run();
}
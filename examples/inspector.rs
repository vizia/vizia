use vizia::*;

fn main() {
    Application::new(WindowDescription::new(), |cx|{
        Inspector::new(cx, |cx|{
            VStack::new(cx, |cx|{
                Element::new(cx)
                .width(Pixels(40.0))
                .height(Pixels(80.0))
                .space(Stretch(1.0))
                .background_color(Color::red());
            })
                .width(Pixels(200.0))
                .height(Pixels(200.0))
                .top(Stretch(1.0))
                .bottom(Pixels(50.0))
                .background_color(Color::blue());

            Element::new(cx)
                .width(Pixels(150.0))
                .height(Pixels(150.0))
                .background_color(Color::blue());
            
            Element::new(cx)
                .width(Pixels(100.0))
                .height(Pixels(100.0))
                .bottom(Percentage(20.0))
                .background_color(Color::blue());
        });
    }).run();
}
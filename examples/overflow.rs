use vizia::*;

fn main() {
    Application::new(WindowDescription::new(), |cx| {
        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Element::new(cx)
                    .size(Pixels(50.0))
                    .left(Pixels(75.0))
                    .background_color(Color::blue());
            })
            .size(Pixels(100.0))
            .left(Pixels(150.0))
            .min_size(Pixels(0.0))
            .background_color(Color::green())
            .overflow(Overflow::Visible);
        })
        .size(Pixels(200.0))
        .background_color(Color::red())
        .min_size(Pixels(0.0))
        .overflow(Overflow::Visible);
    })
    .run();
}

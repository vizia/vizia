use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        Scroll::new(|cx| {
            Element::new(cx).size(Pixels(50.0)).background_color(Color::blue());
        })
        .horizontal_indicator(false)
        .vertical_indicator(false)
        .size(Pixels(100.0))
        .background_color(Color::red())
        .build(cx);
    })
    .run();
}

use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        // Scroll::builder(|cx| {
        //     Element::new(cx).size(Pixels(50.0)).background_color(Color::blue());
        // })
        // .horizontal_indicator(false)
        // .vertical_indicator(false)
        // .build(cx)
        // .size(Pixels(100.0))
        // .background_color(Color::red());
        Button2::new(cx, "Test");
        Button2::new(
            cx,
            Scroll::builder(|cx| {
                Element::new(cx).size(Pixels(50.0)).background_color(Color::blue());
            }),
        );
    })
    .run();
}

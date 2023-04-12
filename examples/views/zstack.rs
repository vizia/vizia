use vizia::prelude::*;

const COLORS: [Color; 3] = [Color::red(), Color::green(), Color::blue()];

fn main() {
    Application::new(|cx| {
        ZStack::new(cx, |cx| {
            for i in 0..3 {
                Element::new(cx)
                    .size(Pixels(100.0))
                    .translate(Pixels(10.0 * i as f32))
                    .background_color(COLORS[i]);
            }
        })
        .child_space(Stretch(1.0));
    })
    .title("ZStack")
    .run();
}

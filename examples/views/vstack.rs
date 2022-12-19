use vizia::prelude::*;

const COLORS: [RGBA; 3] = [RGBA::RED, RGBA::GREEN, RGBA::BLUE];

fn main() {
    Application::new(|cx| {
        VStack::new(cx, |cx| {
            for i in 0..3 {
                Element::new(cx).size(Pixels(100.0)).background_color(COLORS[i]);
            }
        })
        .left(Pixels(10.0))
        .top(Pixels(10.0));
    })
    .title("VStack")
    .run();
}

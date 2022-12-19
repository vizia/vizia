use vizia::prelude::*;

const COLORS: [RGBA; 3] = [RGBA::RED, RGBA::GREEN, RGBA::BLUE];

fn main() {
    Application::new(|cx| {
        HStack::new(cx, |cx| {
            for i in 0..3 {
                Element::new(cx).size(Pixels(100.0)).background_color(COLORS[i]);
            }
        })
        .space(Pixels(10.0));
    })
    .title("HStack")
    .run();
}

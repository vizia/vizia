use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        HStack::new(cx, |cx| {
            Element::new(cx)
                .size(Pixels(50.0))
                .background_color(RGBA::GREEN)
                .position_type(PositionType::SelfDirected)
                .z_index(1);
            Element::new(cx)
                .size(Pixels(50.0))
                .background_color(RGBA::BLUE)
                .position_type(PositionType::SelfDirected)
                .space(Pixels(20.0));
        })
        .size(Pixels(100.0))
        .background_color(RGBA::RED);
    })
    .run()
}

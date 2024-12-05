use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        HStack::new(cx, |cx| {
            Element::new(cx)
                .size(Pixels(50.0))
                .background_color(RGBA::GREEN)
                .position_type(PositionType::Absolute)
                .z_index(1);
            Element::new(cx)
                .size(Pixels(50.0))
                .background_color(RGBA::BLUE)
                .position_type(PositionType::Absolute)
                .space(Pixels(20.0));
        })
        .size(Pixels(100.0))
        .background_color(RGBA::RED);
    })
    .run()
}

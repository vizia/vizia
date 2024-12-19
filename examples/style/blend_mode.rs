use vizia::prelude::*;

const STYLE: &str = r#"

.multiply element {
    blend-mode: multiply;
}

"#;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        HStack::new(cx, |cx| {
            Element::new(cx)
                .size(Pixels(50.0))
                .background_color(Color::green())
                .position_type(PositionType::Absolute);
            Element::new(cx)
                .size(Pixels(50.0))
                .left(Pixels(25.0))
                .background_color(Color::blue())
                .position_type(PositionType::Absolute);
        })
        .class("multiply")
        .size(Pixels(100.0));
    })
    .run()
}

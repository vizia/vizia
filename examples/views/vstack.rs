use vizia::prelude::*;

const COLORS: [Color; 3] = [Color::red(), Color::green(), Color::blue()];

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        VStack::new(cx, |cx| {
            for color in COLORS {
                Element::new(cx).size(Pixels(100.0)).background_color(color);
            }
        })
        .alignment(Alignment::Center);
    })
    .title("VStack")
    .run()
}

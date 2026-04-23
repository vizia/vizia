mod helpers;
use helpers::*;
use vizia::prelude::*;

const COLORS: [Color; 3] = [Color::red(), Color::green(), Color::blue()];

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        ExamplePage::vertical(cx, |cx| {
            ZStack::new(cx, |cx| {
                for (i, color) in COLORS.into_iter().enumerate() {
                    Element::new(cx)
                        .size(Pixels(100.0))
                        .space(Stretch(1.0))
                        .translate(Pixels(10.0 * i as f32))
                        .background_color(color);
                }
            });
        });
    })
    .title(Localized::new("view-title-zstack"))
    .run()
}

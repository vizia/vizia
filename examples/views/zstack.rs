mod helpers;
use helpers::*;
use vizia::prelude::*;

const COLORS: [Color; 3] = [Color::red(), Color::green(), Color::blue()];

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        ExamplePage::vertical(cx, |cx| {
            ZStack::new(cx, |cx| {
                for (i, color) in COLORS.into_iter().enumerate() {
                    HStack::new(cx, |cx| {
                        Element::new(cx).size(Pixels(100.0)).background_color(color);
                    })
                    .size(Auto)
                    .padding_left(Pixels(10.0 * i as f32))
                    .padding_top(Pixels(10.0 * i as f32));
                }
            })
            .size(Auto)
            .padding(Pixels(30.0))
            .background_color(Color::gray());
        });
    })
    .title(Localized::new("view-title-zstack"))
    .run()
}

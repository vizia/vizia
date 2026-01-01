use vizia::prelude::*;

const COLORS: [Color; 3] = [Color::red(), Color::green(), Color::blue()];

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        let size_100 = cx.state(Pixels(100.0));
        let align_center = cx.state(Alignment::Center);
        VStack::new(cx, |cx| {
            for color in COLORS {
                let color_signal = cx.state(color);
                Element::new(cx).size(size_100).background_color(color_signal);
            }
        })
        .alignment(align_center);
        cx.state("VStack")
    });

    app.title(title).run()
}

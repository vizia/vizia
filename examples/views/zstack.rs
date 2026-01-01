use vizia::prelude::*;

const COLORS: [Color; 3] = [Color::red(), Color::green(), Color::blue()];

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        let size_100 = cx.state(Pixels(100.0));
        let align_center = cx.state(Alignment::Center);
        ZStack::new(cx, |cx| {
            for (i, color) in COLORS.into_iter().enumerate() {
                let offset = cx.state(Pixels(10.0 * i as f32));
                let color_signal = cx.state(color);
                Element::new(cx)
                    .size(size_100)
                    .translate(offset)
                    .background_color(color_signal);
            }
        })
        .alignment(align_center);
        cx.state("ZStack")
    });

    app.title(title).run()
}

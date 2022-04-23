use vizia::*;

const COLORS: [Color; 3] = [Color::red(), Color::green(), Color::blue()];

fn main() {
    Application::new(|cx| {
        Label::new(cx, "A zstack arranges its children on top of each other.")
            .width(Stretch(1.0))
            .position_type(PositionType::SelfDirected)
            .space(Pixels(10.0));

        ZStack::new(cx, |cx| {
            for i in 0..3 {
                Element::new(cx)
                    .size(Pixels(100.0))
                    .top(Pixels(10.0 * i as f32))
                    .left(Pixels(10.0 * i as f32))
                    .background_color(COLORS[i]);
            }
        })
        .left(Pixels(10.0))
        .top(Pixels(50.0));
    })
    .title("ZStack")
    .run();
}

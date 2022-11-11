use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        Element::new(cx).size(Pixels(200.0)).space(Pixels(20.0)).background_gradient(
            LinearGradient::new(GradientDirection::LeftToRight)
                .add_stop((Percentage(0.0), Color::rgb(255, 0, 0)))
                .add_stop((Percentage(16.7), Color::rgb(255, 0, 255)))
                .add_stop((Percentage(33.3), Color::rgb(0, 0, 255)))
                .add_stop((Percentage(50.0), Color::rgb(0, 255, 255)))
                .add_stop((Percentage(66.7), Color::rgb(0, 255, 0)))
                .add_stop((Percentage(83.3), Color::rgb(255, 255, 0)))
                .add_stop((Percentage(100.0), Color::rgb(255, 0, 0))),
        );
    })
    .run();
}

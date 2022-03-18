use vizia::*;

// Example showing how to set a custom property on a view
fn main() {
    Application::new(WindowDescription::new().with_title("Test"), |cx| {
        Element::new(cx)
            .size(Pixels(100.0))
            .background_color(Color::red())
            .overflow(Overflow::Visible)
            .translate((50.0, 0.0))
            .scale((1.0, 1.0));
    })
    .run();
}

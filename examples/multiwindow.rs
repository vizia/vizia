use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Primary Window"), |cx| {
        Element::new(cx).size(Pixels(100.0)).background_color(Color::blue());
        Window::new2(cx, WindowDescription::new().with_title("Secondary Window"), |cx| {
            Element::new(cx).size(Pixels(50.0)).space(Pixels(50.0)).background_color(Color::red());
        });
    })
    .run();
}

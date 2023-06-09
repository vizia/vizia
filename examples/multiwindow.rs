pub use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        Element::new(cx).size(Pixels(100.0)).background_color(Color::red());

        Window::new(cx, |cx| {
            Element::new(cx).size(Pixels(50.0)).background_color(Color::blue());
        });
    })
    .run();
}

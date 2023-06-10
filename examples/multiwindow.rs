pub use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        Element::new(cx).size(Pixels(100.0)).background_color(Color::red());

        Window::new(cx, |cx| {
            // Element::new(cx).size(Pixels(50.0)).background_color(Color::blue());
            // Button::new(cx, |cx| cx.emit(WindowEvent::WindowClose), |cx| Label::new(cx, "C2"));
        })
        .title("Secondary")
        .inner_size((400, 400));

        Button::new(cx, |cx| cx.emit(WindowEvent::WindowClose), |cx| Label::new(cx, "C1"));
    })
    .title("Main")
    .run();
}

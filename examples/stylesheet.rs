#[allow(unused)]
use vizia::prelude::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("This example is not supported on wasm - uses filesystem");
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    Application::new(|cx| {
        cx.add_stylesheet("examples/resources/test.css").expect("Failed to find file");

        VStack::new(cx, |cx| {
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Button"));
            Button::new(cx, |cx| cx.emit(WindowEvent::Reload), |cx| Label::new(cx, "Reload"));
        })
        .row_between(Pixels(10.0))
        .space(Pixels(20.0));
    })
    .title("Stylesheet")
    .run();
}

#[allow(unused)]
use vizia::prelude::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("This example is not supported on wasm - uses filesystem");
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    Application::new(|cx| {
        Element::new(cx)
            .size(Pixels(100.0))
            .space(Pixels(20.0))
            .background_color(Color::rgb(255, 0, 0))
            .border_width("10px")
            .border_width(LengthValue::Px(30.0))
            .border_width("50%")
            .border_color(Color::rgb(0,0,0));
    })
    .title("Stylesheet")
    .run();
}

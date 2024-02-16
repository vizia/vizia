#[allow(unused)]
use vizia::prelude::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("This example is not supported on wasm - uses filesystem");
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("examples/resources/themes/test.css"))
            .expect("Failed to add stylesheet");
        HStack::new(cx, |cx| {
            Element::new(cx).class("foo");
        })
        .size(Pixels(200.0))
        .class("bar");
    })
    .run()
}

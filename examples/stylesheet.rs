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
            .space(Pixels(100.0))
            .background_color("red")
            .border_width("10px")
            .border_radius("10px")
            .border_top_left_radius("50%")
            .border_top_right_radius("0")
            .border_top_left_shape("bevel")
            .border_color("#009900")
            .outline_width("2px")
            .outline_offset("5px")
            .outline_color("cyan");
    })
    .title("Stylesheet")
    .run();
}

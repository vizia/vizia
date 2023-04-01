#[allow(unused)]
use vizia::prelude::*;

// const STYLE: &str = r#"
//     element {
//         width: 300px;
//         height: 100px;
//         background-color: green;
//         background-image: linear-gradient(to right, blue, green 20%, red 30%);
//     }
// "#;

// static test: &str = include_str!("resources/test.css");

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("This example is not supported on wasm - uses filesystem");
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    Application::new(|cx| {
        Element::new(cx)
            .size(Pixels(50.0))
            .position_type(PositionType::SelfDirected)
            .background_color(Color::red())
            .top(Percentage(50.0));
        // .translate((Pixels(50.0), Pixels(50.0)));
    })
    .title("Stylesheet")
    .run();
}

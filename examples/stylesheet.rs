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
    // Application::new(|cx| {
    //     // Element::new(cx)
    //     //     .size(Pixels(50.0))
    //     //     .position_type(PositionType::SelfDirected)
    //     //     .background_color(Color::red())
    //     //     .top(Percentage(50.0));
    //     // .translate((Pixels(50.0), Pixels(50.0)));

    // })
    // .title("Stylesheet")
    // .run();
    Application::new(|cx| {
        // Element::new(cx).size(Pixels(100.0)).background_color(Color::red()).space(Pixels(100.0));
        Label::new(cx, "Test").background_color(Color::red()).width(Stretch(1.0));
        // Label::new(cx, "textسلام").width(Pixels(200.0)).background_color(Color::green());
        // Label::new(cx, "سلام").background_color(Color::green());
        // Label::new(cx, "سلام").background_color(Color::green());
        // Button::new(
        //     cx,
        //     |_| {},
        //     |cx| Label::new(cx, "سلام").width(Pixels(100.0)).background_color(Color::red()),
        // );
        // Button::new(
        //     cx,
        //     |_| {},
        //     |cx| Label::new(cx, "some text").width(Pixels(100.0)).background_color(Color::red()),
        // );
    })
    .run();
}

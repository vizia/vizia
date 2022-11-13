#[allow(unused)]
use vizia::prelude::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("This example is not supported on wasm - uses filesystem");
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use vizia::style::Transform2D;

    Application::new(|cx| {
        // cx.add_stylesheet("examples/resources/test.css").expect("Failed to find file");

        // Button::new(cx, |_| {}, |cx| Label::new(cx, "Button"));
        // let mut transform1 = Transform2D::identity();
        // transform1.rotate(0.0);
        // let mut translate = Transform2D::identity();
        // translate.translate(0.0, -100.0);
        // transform1.premultiply(&translate);

        // let mut transform2 = Transform2D::identity();
        // transform2.rotate(30.0);
        // transform2.premultiply(&translate);

        // println!("{}", transform);
        // transform.rotate(45.0);
        HStack::new(cx, |cx| {
            Label::new(cx, "1")
                .background_color(Color::red())
                .space(Pixels(0.0))
                .size(Pixels(50.0))
                .translate((50.0, 0.0))
                .rotate(45.0f32)
                //.transform(transform1)
                .position_type(PositionType::SelfDirected);
        })
        .translate((-20.0, 0.0))
        .width(Pixels(100.0))
        .height(Pixels(50.0))
        .background_color(Color::blue())
        .overflow(Overflow::Hidden);

        // Label::new(cx, "2")
        //     .background_color(Color::red())
        //     .space(Stretch(1.0))
        //     .size(Pixels(50.0))
        //     //.transform(transform2)
        //     .position_type(PositionType::SelfDirected);
    })
    .title("Stylesheet")
    .run();
}

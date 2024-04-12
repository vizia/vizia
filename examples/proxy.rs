#[allow(unused)]
use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview - proxies are winit only");
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Element::new(cx).height(Pixels(200.0));
            })
            .height(Auto)
            .background_color(Color::yellow());
            VStack::new(cx, |cx| {}).background_color(Color::blue());
        })
        .height(Auto)
        .background_color(Color::red());
    })
    .run()
}

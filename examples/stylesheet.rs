#[allow(unused)]
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("examples/resources/themes/test.css"))
            .expect("Failed to add stylesheet");
        let size_200 = cx.state(Pixels(200.0));
        HStack::new(cx, |cx| {
            Element::new(cx).class("foo");
        })
        .size(size_200);
    })
    .ignore_default_theme()
    .run()
}

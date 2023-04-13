mod helpers;
pub use helpers::*;
use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        VStack::new(cx, |cx| {
            Tooltip::new(cx, |cx| {
                Label::new(cx, "Subtitle").class("subtitle");
                Label::new(cx, "Very serious tooltip explanation here.");
            });
        })
        .disabled(ControlsData::disabled)
        .class("container");
    })
    .title("Tooltip")
    .run();
}

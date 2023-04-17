mod helpers;
pub use helpers::*;
use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        ExamplePage::new(cx, |cx| {
            Tooltip::new(cx, |cx| {
                Label::new(cx, "Subtitle").class("subtitle");
                Label::new(cx, "Very serious tooltip explanation here.");
            });
        });
    })
    .title("Tooltip")
    .run();
}

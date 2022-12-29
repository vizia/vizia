mod helpers;
pub use helpers::*;
use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        view_controls(cx);

        VStack::new(cx, |cx| {
            Tooltip::new(cx, "Tooltip here!", |cx| {
                Label::new(cx, "Subtitle").class("subtitle");
                Label::new(cx, "Very serious tooltip explanation here.");
            })
            .on_ok(|_| println!("Ok!"));

            TooltipSeq::new(cx, "Tooltip here!", |cx| {
                Label::new(cx, "Subtitle").class("subtitle");
                Label::new(cx, "Very serious tooltip explanation here.");
            })
            .on_next(|_| println!("Next!"))
            .on_prev(|_| println!("Prev!"));
        })
        .disabled(ControlsData::disabled)
        .class("container");
    })
    .title("Tooltip")
    .run();
}

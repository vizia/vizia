mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        ExamplePage::new(cx, |cx| {
            HStack::new(cx, |cx| {
                ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                    Label::new(cx, "Vertical Scroll").height(Pixels(1000.0)).width(Stretch(1.0));
                })
                .size(Units::Pixels(300.0))
                .class("bg-default");

                ScrollView::new(cx, 0.0, 0.0, true, false, |cx| {
                    Label::new(cx, "Horizontal Scroll").width(Pixels(1000.0)).height(Stretch(1.0));
                })
                .size(Units::Pixels(300.0))
                .class("bg-default");

                ScrollView::new(cx, 0.0, 0.0, true, true, |cx| {
                    Label::new(cx, "Horizontal and Vertical Scroll")
                        .width(Pixels(1000.0))
                        .height(Pixels(1000.0));
                })
                .size(Units::Pixels(300.0))
                .class("bg-default");
            })
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(50.0));
        });
    })
    .title("Scrollview")
    .inner_size((1100, 400))
    .run()
}

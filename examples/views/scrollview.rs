mod helpers;
use helpers::*;
use vizia::prelude::*;
use vizia_core::binding::RatioLens;

fn main() {
    Application::new(|cx| {
        ExamplePage::new(cx, |cx| {
            HStack::new(cx, |cx| {
                // TODO: Link scrollviews to the same scroll data
                // ScrollData {
                //     scroll_x: 0.0,
                //     scroll_y: 0.0,
                //     child_x: 1000.0,
                //     child_y: 300.0,
                //     parent_x: 300.0,
                //     parent_y: 300.0,
                // }
                // .build(cx);

                ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                    Label::new(cx, "Label 2")
                        .height(Pixels(1000.0))
                        .background_color(Color::from("EF5151"));
                })
                .size(Units::Pixels(300.0))
                .class("bg-default");

                ScrollView::new(cx, 0.0, 0.0, true, false, |cx| {
                    Label::new(cx, "Label 2")
                        .width(Pixels(1000.0))
                        .height(Pixels(100.0))
                        .background_color(Color::from("EF5151"));
                })
                .size(Units::Pixels(300.0))
                .class("bg-default");

                ScrollView::new(cx, 0.0, 0.0, true, true, |cx| {
                    Label::new(cx, "Label 2")
                        .width(Pixels(1000.0))
                        .height(Pixels(100.0))
                        .background_color(Color::from("EF5151"));
                    Label::new(cx, "Label 2")
                        .height(Pixels(900.0))
                        .background_color(Color::from("EF5151"));
                })
                .size(Units::Pixels(300.0))
                .class("bg-default");

                // HStack::new(cx, |cx| {
                //     ScrollView::custom(cx, false, false, ScrollData::root, |cx| {
                //         Label::new(cx, "Label 1")
                //             .width(Units::Pixels(1000.0))
                //             .background_color(Color::from("51AFEF"));
                //     })
                //     .size(Units::Pixels(300.0));
                //     ScrollView::custom(cx, false, false, ScrollData::root, |cx| {
                //         Label::new(cx, "Label 2")
                //             .width(Units::Pixels(1000.0))
                //             .background_color(Color::from("EF5151"));
                //     })
                //     .size(Units::Pixels(300.0));
                // });
                // Scrollbar::new(
                //     cx,
                //     ScrollData::scroll_x,
                //     RatioLens::new(ScrollData::parent_x, ScrollData::child_x),
                //     Orientation::Horizontal,
                //     |cx, scroll| {
                //         cx.emit(ScrollEvent::SetX(scroll));
                //     },
                // )
                // .width(Units::Pixels(600.0));
            })
            .size(Stretch(1.0))
            .space(Pixels(0.0))
            .child_space(Stretch(1.0))
            .col_between(Pixels(50.0));
        });
    })
    .title("Scrollview")
    .inner_size((1100, 400))
    .run();
}

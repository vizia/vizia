use vizia::prelude::*;
use vizia_core::state::RatioLens;

fn main() {
    Application::new(|cx| {
        // A basic scroll view containing two labels.
        ScrollView::new(cx, ScrollViewSettings::default(), |cx| {
            Label::new(cx, "Label 1").width(Units::Pixels(1000.0)).background_color(Color::green());
            Label::new(cx, "Label 2").height(Units::Pixels(1000.0)).background_color(Color::blue());
        })
        .size(Units::Pixels(300.0));

        // Custom scroll data that custom scroll views can bind to.
        ScrollData {
            scroll_x: 0.0,
            scroll_y: 0.0,
            child_x: 1000.0,
            child_y: 300.0,
            parent_x: 300.0,
            parent_y: 300.0,
        }
        .build(cx);

        // Three custom scroll views all bound to the same scroll data.
        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                ScrollView::custom(
                    cx,
                    ScrollViewSettings {
                        scrollbar_x: false,
                        scrollbar_y: false,
                        ..Default::default()
                    },
                    ScrollData::root,
                    |cx| {
                        Label::new(cx, "Custom scroll view 1")
                            .width(Units::Pixels(1000.0))
                            .background_color(Color::red());
                    },
                )
                .size(Units::Pixels(300.0));

                ScrollView::custom(
                    cx,
                    ScrollViewSettings {
                        scrollbar_x: false,
                        scrollbar_y: false,
                        ..Default::default()
                    },
                    ScrollData::root,
                    |cx| {
                        Label::new(cx, "Custom scroll view 2")
                            .width(Units::Pixels(1000.0))
                            .background_color(Color::green());
                    },
                )
                .size(Units::Pixels(300.0));

                // A custom scroll view that is not binding to the `scroll_x` lens of the scroll data.
                // This disables the horizontal scrolling of the scroll view while still allowing it to scroll vertically.
                ScrollView::custom(
                    cx,
                    ScrollViewSettings {
                        scrollbar_x: false,
                        scrollbar_y: false,
                        scroll_x: false,
                        ..Default::default()
                    },
                    ScrollData::root,
                    |cx| {
                        Label::new(cx, "Custom scroll view 3")
                            .width(Units::Pixels(1000.0))
                            .height(Units::Pixels(1000.0))
                            .background_color(Color::blue());
                    },
                )
                .size(Units::Pixels(300.0));

                // A custom scrollbar used to scroll the custom views vertically.
                Scrollbar::new(
                    cx,
                    ScrollData::scroll_y,
                    RatioLens::new(ScrollData::parent_y, ScrollData::child_y),
                    Orientation::Vertical,
                    |cx, scroll| {
                        cx.emit(ScrollEvent::SetY(scroll));
                    },
                )
                .width(Units::Pixels(14.0))
                .height(Stretch(1.0));
            })
            .width(Auto)
            .height(Auto);

            // A custom scrollbar used to scroll the custom views horizontally.
            Scrollbar::new(
                cx,
                ScrollData::scroll_x,
                RatioLens::new(ScrollData::parent_x, ScrollData::child_x),
                Orientation::Horizontal,
                |cx, scroll| {
                    cx.emit(ScrollEvent::SetX(scroll));
                },
            )
            .right(Pixels(14.0))
            .height(Units::Pixels(14.0));
        })
        .width(Auto)
        .height(Auto);
    })
    .title("Scrollview")
    .inner_size((600, 614))
    .run();
}

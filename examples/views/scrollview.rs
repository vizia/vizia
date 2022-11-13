use vizia::prelude::*;
use vizia_core::state::RatioLens;

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        ScrollView::new(cx, 0.0, 0.0, true, true, |cx| {
            Label::new(cx, "Label 1")
                .width(Units::Pixels(1000.0))
                .background_color(Color::from("51AFEF"));
            Label::new(cx, "Label 2")
                .height(Units::Pixels(1000.0))
                .background_color(Color::from("EF5151"));
        })
        .size(Units::Pixels(300.0));
        ScrollData {
            scroll_x: 0.0,
            scroll_y: 0.0,
            child_x: 1000.0,
            child_y: 300.0,
            parent_x: 300.0,
            parent_y: 300.0,
        }
        .build(cx);
        HStack::new(cx, |cx| {
            ScrollView::custom(cx, false, false, ScrollData::root, |cx| {
                Label::new(cx, "Label 1")
                    .width(Units::Pixels(1000.0))
                    .background_color(Color::from("51AFEF"));
            })
            .size(Units::Pixels(300.0));
            ScrollView::custom(cx, false, false, ScrollData::root, |cx| {
                Label::new(cx, "Label 2")
                    .width(Units::Pixels(1000.0))
                    .background_color(Color::from("EF5151"));
            })
            .size(Units::Pixels(300.0));
        });
        Scrollbar::new(
            cx,
            ScrollData::scroll_x,
            RatioLens::new(ScrollData::parent_x, ScrollData::child_x),
            Orientation::Horizontal,
            |cx, scroll| {
                cx.emit(ScrollEvent::SetX(scroll));
            },
        )
        .width(Units::Pixels(600.0));
    })
    .ignore_default_theme()
    .title("Scrollview")
    .inner_size((600, 614))
    .run();
}

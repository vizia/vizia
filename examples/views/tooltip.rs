mod helpers;
pub use helpers::*;
use vizia::prelude::*;

const STYLE: &str = r#"
    element.test {
        background-color: rgb(100, 100, 100);
        size: 100px;
        padding: 1s;
    }
"#;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");
        let auto = cx.state(Auto);
        let gap_8 = cx.state(Pixels(8.0));
        let padding_4 = cx.state(Pixels(4.0));
        let arrow_size = cx.state(Length::Value(LengthValue::Px(16.0)));
        let text_top_start = cx.state("Top Start");
        let text_top = cx.state("Top");
        let text_top_end = cx.state("Top End");
        let text_left_start = cx.state("LeftStart");
        let text_left = cx.state("Left");
        let text_left_end = cx.state("LeftEnd");
        let text_right_start = cx.state("RightStart");
        let text_right = cx.state("Right");
        let text_right_end = cx.state("RightEnd");
        let text_bottom_start = cx.state("BottomStart");
        let text_bottom = cx.state("Bottom");
        let text_bottom_end = cx.state("BottomEnd");
        let text_over = cx.state("Over");
        let text_cursor = cx.state("Cursor");
        let placement_top_start = cx.state(Placement::TopStart);
        let placement_top = cx.state(Placement::Top);
        let placement_top_end = cx.state(Placement::TopEnd);
        let placement_left_start = cx.state(Placement::LeftStart);
        let placement_left = cx.state(Placement::Left);
        let placement_left_end = cx.state(Placement::LeftEnd);
        let placement_right_start = cx.state(Placement::RightStart);
        let placement_right = cx.state(Placement::Right);
        let placement_right_end = cx.state(Placement::RightEnd);
        let placement_bottom_start = cx.state(Placement::BottomStart);
        let placement_bottom = cx.state(Placement::Bottom);
        let placement_bottom_end = cx.state(Placement::BottomEnd);
        let placement_over = cx.state(Placement::Over);
        let placement_cursor = cx.state(Placement::Cursor);

        ExamplePage::vertical(cx, |cx| {
            HStack::new(cx, |cx| {
                Element::new(cx)
                    .text(text_top_start)
                    .tooltip(move |cx| {
                        Tooltip::new(cx, |cx| {
                            Label::static_text(cx, "This is a tooltip").padding(padding_4);
                        })
                        .padding(padding_4)
                        .size(auto)
                        .placement(placement_top_start)
                        .arrow_size(arrow_size)
                    })
                    .class("test");

                Element::new(cx)
                    .text(text_top)
                    .tooltip(move |cx| {
                        Tooltip::new(cx, |cx| {
                            Label::static_text(cx, "This is a tooltip").padding(padding_4);
                        })
                        .padding(padding_4)
                        .size(auto)
                        .placement(placement_top)
                    })
                    .class("test");

                Element::new(cx)
                    .text(text_top_end)
                    .tooltip(move |cx| {
                        Tooltip::new(cx, |cx| {
                            Label::static_text(cx, "This is a tooltip").padding(padding_4);
                        })
                        .padding(padding_4)
                        .size(auto)
                        .placement(placement_top_end)
                    })
                    .class("test");
            })
            .size(auto)
            .horizontal_gap(gap_8);

            HStack::new(cx, |cx| {
                Element::new(cx)
                    .text(text_left_start)
                    .tooltip(move |cx| {
                        Tooltip::new(cx, |cx| {
                            Label::static_text(cx, "This is a tooltip").padding(padding_4);
                        })
                        .padding(padding_4)
                        .size(auto)
                        .placement(placement_left_start)
                    })
                    .class("test");

                Element::new(cx)
                    .text(text_left)
                    .tooltip(move |cx| {
                        Tooltip::new(cx, |cx| {
                            Label::static_text(cx, "This is a tooltip").padding(padding_4);
                        })
                        .padding(padding_4)
                        .size(auto)
                        .placement(placement_left)
                    })
                    .class("test");

                Element::new(cx)
                    .text(text_left_end)
                    .tooltip(move |cx| {
                        Tooltip::new(cx, |cx| {
                            Label::static_text(cx, "This is a tooltip").padding(padding_4);
                        })
                        .padding(padding_4)
                        .size(auto)
                        .placement(placement_left_end)
                    })
                    .class("test");
            })
            .size(auto)
            .horizontal_gap(gap_8);

            HStack::new(cx, |cx| {
                Element::new(cx)
                    .text(text_right_start)
                    .tooltip(move |cx| {
                        Tooltip::new(cx, |cx| {
                            Label::static_text(cx, "This is a tooltip").padding(padding_4);
                        })
                        .padding(padding_4)
                        .size(auto)
                        .placement(placement_right_start)
                    })
                    .class("test");

                Element::new(cx)
                    .text(text_right)
                    .tooltip(move |cx| {
                        Tooltip::new(cx, |cx| {
                            Label::static_text(cx, "This is a tooltip").padding(padding_4);
                        })
                        .padding(padding_4)
                        .size(auto)
                        .placement(placement_right)
                    })
                    .class("test");

                Element::new(cx)
                    .text(text_right_end)
                    .tooltip(move |cx| {
                        Tooltip::new(cx, |cx| {
                            Label::static_text(cx, "This is a tooltip").padding(padding_4);
                        })
                        .padding(padding_4)
                        .size(auto)
                        .placement(placement_right_end)
                    })
                    .class("test");
            })
            .size(auto)
            .horizontal_gap(gap_8);

            HStack::new(cx, |cx| {
                Element::new(cx)
                    .text(text_bottom_start)
                    .tooltip(move |cx| {
                        Tooltip::new(cx, |cx| {
                            Label::static_text(cx, "This is a tooltip").padding(padding_4);
                        })
                        .padding(padding_4)
                        .size(auto)
                        .placement(placement_bottom_start)
                    })
                    .class("test");

                Element::new(cx)
                    .text(text_bottom)
                    .tooltip(move |cx| {
                        Tooltip::new(cx, |cx| {
                            Label::static_text(cx, "This is a tooltip").padding(padding_4);
                        })
                        .padding(padding_4)
                        .size(auto)
                        .placement(placement_bottom)
                    })
                    .class("test");

                Element::new(cx)
                    .text(text_bottom_end)
                    .tooltip(move |cx| {
                        Tooltip::new(cx, |cx| {
                            Label::static_text(cx, "This is a tooltip").padding(padding_4);
                        })
                        .padding(padding_4)
                        .size(auto)
                        .placement(placement_bottom_end)
                    })
                    .class("test");
            })
            .size(auto)
            .horizontal_gap(gap_8);

            HStack::new(cx, |cx| {
                Element::new(cx)
                    .text(text_over)
                    .tooltip(move |cx| {
                        Tooltip::new(cx, |cx| {
                            Label::static_text(cx, "This is a tooltip").padding(padding_4);
                        })
                        .padding(padding_4)
                        .size(auto)
                        .placement(placement_over)
                    })
                    .class("test");

                Element::new(cx)
                    .text(text_cursor)
                    .tooltip(move |cx| {
                        Tooltip::new(cx, |cx| {
                            Label::static_text(cx, "This is a tooltip").padding(padding_4);
                        })
                        .padding(padding_4)
                        .size(auto)
                        .placement(placement_cursor)
                    })
                    .class("test");
            })
            .size(auto)
            .horizontal_gap(gap_8);
        });
        (cx.state("Tooltip"), cx.state((800, 800)))
    });

    app.title(title).inner_size(size).run()
}

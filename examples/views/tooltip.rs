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
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        ExamplePage::vertical(cx, |cx| {
            HStack::new(cx, |cx| {
                Element::new(cx)
                    .text(Localized::new("tooltip-placement-top-start"))
                    .tooltip(|cx| {
                        Tooltip::new(cx, |cx| {
                            Label::new(cx, Localized::new("tooltip-text")).padding(Pixels(4.0));
                        })
                        .padding(Pixels(4.0))
                        .size(Auto)
                        .placement(Placement::TopStart)
                    })
                    .class("test");

                Element::new(cx)
                    .text(Localized::new("tooltip-placement-top"))
                    .tooltip(|cx| {
                        Tooltip::new(cx, |cx| {
                            Label::new(cx, Localized::new("tooltip-text")).padding(Pixels(4.0));
                        })
                        .padding(Pixels(4.0))
                        .size(Auto)
                        .placement(Placement::Top)
                    })
                    .class("test");

                Element::new(cx)
                    .text(Localized::new("tooltip-placement-top-end"))
                    .tooltip(|cx| {
                        Tooltip::new(cx, |cx| {
                            Label::new(cx, Localized::new("tooltip-text")).padding(Pixels(4.0));
                        })
                        .padding(Pixels(4.0))
                        .size(Auto)
                        .placement(Placement::TopEnd)
                    })
                    .class("test");
            })
            .size(Auto)
            .horizontal_gap(Pixels(8.0));

            HStack::new(cx, |cx| {
                Element::new(cx)
                    .text(Localized::new("tooltip-placement-left-start"))
                    .tooltip(|cx| {
                        Tooltip::new(cx, |cx| {
                            Label::new(cx, Localized::new("tooltip-text")).padding(Pixels(4.0));
                        })
                        .padding(Pixels(4.0))
                        .size(Auto)
                        .placement(Placement::LeftStart)
                    })
                    .class("test");

                Element::new(cx)
                    .text(Localized::new("tooltip-placement-left"))
                    .tooltip(|cx| {
                        Tooltip::new(cx, |cx| {
                            Label::new(cx, Localized::new("tooltip-text")).padding(Pixels(4.0));
                        })
                        .padding(Pixels(4.0))
                        .size(Auto)
                        .placement(Placement::Left)
                    })
                    .class("test");

                Element::new(cx)
                    .text(Localized::new("tooltip-placement-left-end"))
                    .tooltip(|cx| {
                        Tooltip::new(cx, |cx| {
                            Label::new(cx, Localized::new("tooltip-text")).padding(Pixels(4.0));
                        })
                        .padding(Pixels(4.0))
                        .size(Auto)
                        .placement(Placement::LeftEnd)
                    })
                    .class("test");
            })
            .size(Auto)
            .horizontal_gap(Pixels(8.0));

            HStack::new(cx, |cx| {
                Element::new(cx)
                    .text(Localized::new("tooltip-placement-right-start"))
                    .tooltip(|cx| {
                        Tooltip::new(cx, |cx| {
                            Label::new(cx, Localized::new("tooltip-text")).padding(Pixels(4.0));
                        })
                        .padding(Pixels(4.0))
                        .size(Auto)
                        .placement(Placement::RightStart)
                    })
                    .class("test");

                Element::new(cx)
                    .text(Localized::new("tooltip-placement-right"))
                    .tooltip(|cx| {
                        Tooltip::new(cx, |cx| {
                            Label::new(cx, Localized::new("tooltip-text")).padding(Pixels(4.0));
                        })
                        .padding(Pixels(4.0))
                        .size(Auto)
                        .placement(Placement::Right)
                    })
                    .class("test");

                Element::new(cx)
                    .text(Localized::new("tooltip-placement-right-end"))
                    .tooltip(|cx| {
                        Tooltip::new(cx, |cx| {
                            Label::new(cx, Localized::new("tooltip-text")).padding(Pixels(4.0));
                        })
                        .padding(Pixels(4.0))
                        .size(Auto)
                        .placement(Placement::RightEnd)
                    })
                    .class("test");
            })
            .size(Auto)
            .horizontal_gap(Pixels(8.0));

            HStack::new(cx, |cx| {
                Element::new(cx)
                    .text(Localized::new("tooltip-placement-bottom-start"))
                    .tooltip(|cx| {
                        Tooltip::new(cx, |cx| {
                            Label::new(cx, Localized::new("tooltip-text")).padding(Pixels(4.0));
                        })
                        .padding(Pixels(4.0))
                        .size(Auto)
                        .placement(Placement::BottomStart)
                    })
                    .class("test");

                Element::new(cx)
                    .text(Localized::new("tooltip-placement-bottom"))
                    .tooltip(|cx| {
                        Tooltip::new(cx, |cx| {
                            Label::new(cx, Localized::new("tooltip-text")).padding(Pixels(4.0));
                        })
                        .padding(Pixels(4.0))
                        .size(Auto)
                        .placement(Placement::Bottom)
                    })
                    .class("test");

                Element::new(cx)
                    .text(Localized::new("tooltip-placement-bottom-end"))
                    .tooltip(|cx| {
                        Tooltip::new(cx, |cx| {
                            Label::new(cx, Localized::new("tooltip-text")).padding(Pixels(4.0));
                        })
                        .padding(Pixels(4.0))
                        .size(Auto)
                        .placement(Placement::BottomEnd)
                    })
                    .class("test");
            })
            .size(Auto)
            .horizontal_gap(Pixels(8.0));

            HStack::new(cx, |cx| {
                Element::new(cx)
                    .text(Localized::new("tooltip-placement-over"))
                    .tooltip(|cx| {
                        Tooltip::new(cx, |cx| {
                            Label::new(cx, Localized::new("tooltip-text")).padding(Pixels(4.0));
                        })
                        .padding(Pixels(4.0))
                        .size(Auto)
                        .placement(Placement::Over)
                    })
                    .class("test");

                Element::new(cx)
                    .text(Localized::new("tooltip-placement-cursor"))
                    .tooltip(|cx| {
                        Tooltip::new(cx, |cx| {
                            Label::new(cx, Localized::new("tooltip-text")).padding(Pixels(4.0));
                        })
                        .padding(Pixels(4.0))
                        .size(Auto)
                        .placement(Placement::Cursor)
                    })
                    .class("test");
            })
            .size(Auto)
            .horizontal_gap(Pixels(8.0));
        });
    })
    .title(Localized::new("view-title-tooltip"))
    .inner_size((800, 800))
    .run()
}

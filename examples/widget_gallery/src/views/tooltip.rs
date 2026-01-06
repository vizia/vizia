use vizia::{
    icons::{ICON_INFO_CIRCLE, ICON_TRASH},
    prelude::*,
};

use crate::components::DemoRegion;

pub fn tooltip(cx: &mut Context) {
    let width_140 = cx.state(Pixels(140.0));
    let height_60 = cx.state(Pixels(60.0));
    let gap_8 = cx.state(Pixels(8.0));
    let auto = cx.state(Auto);
    let align_left = cx.state(Alignment::Left);

    VStack::new(cx, move |cx|{

        Markdown::new(cx, "# Tooltip
A tooltip displays supplemental information near its target view. Tooltips are triggered on hover or focus of the target view and dismissed on blur or mouse-out of the target or tooltip container.        
        ");

        Markdown::new(cx, "### Basic tooltip");

        DemoRegion::new(
            cx,
            move |cx| {
                Button::new(cx, |cx |Svg::new(cx, ICON_TRASH))
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Delete");
                    }));
            }, r#"IconButton::new(cx, ICON_TRASH)
    .tooltip(|cx| Tooltip::new(cx, |cx|{
        Label::new(cx, "Delete");
    }));"#
        );

        Markdown::new(cx, "### Tooltip content");

        DemoRegion::new(
            cx,
            move |cx| {
                Button::new(cx, |cx| Svg::new(cx, ICON_TRASH))
                    .tooltip(move |cx| Tooltip::new(cx, move |cx| {
                        HStack::new(cx, move |cx| {
                            Svg::new(cx, ICON_INFO_CIRCLE);
                            Label::new(cx, "Delete");
                        }).size(auto).alignment(align_left);
                    }));
            }, r#"IconButton::new(cx, ICON_TRASH)
    .tooltip(|cx| Tooltip::new(cx, |cx|{
        Label::new(cx, "Delete");
    }));"#
        );

        Markdown::new(cx, "### Tooltip placement");

        DemoRegion::new(
            cx,
            move |cx| {
                let text_variant = cx.state(ButtonVariant::Text);

                VStack::new(cx, move |cx|{
                    Button::new(cx, |cx|{
                        Label::new(cx, "TOP-START")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::TopStart));

                    Button::new(cx, |cx|{
                        Label::new(cx, "LEFT-START")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::LeftStart));

                    Button::new(cx, |cx|{
                        Label::new(cx, "RIGHT-START")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::RightStart));

                    Button::new(cx, |cx|{
                        Label::new(cx, "BOTTOM-START")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::BottomStart));
                }).vertical_gap(gap_8).size(auto);

                VStack::new(cx, move |cx|{
                    Button::new(cx, |cx|{
                        Label::new(cx, "TOP")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::Top));

                    Button::new(cx, |cx|{
                        Label::new(cx, "LEFT")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::Left));

                    Button::new(cx, |cx|{
                        Label::new(cx, "RIGHT")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::Right));

                    Button::new(cx, |cx|{
                        Label::new(cx, "BOTTOM")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::Bottom));
                }).vertical_gap(gap_8).size(auto);

                VStack::new(cx, move |cx|{
                    Button::new(cx, |cx|{
                        Label::new(cx, "TOP-END")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::TopEnd));

                    Button::new(cx, |cx|{
                        Label::new(cx, "LEFT-END")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::LeftEnd));

                    Button::new(cx, |cx: &mut Context|{
                        Label::new(cx, "RIGHT-END")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::RightEnd));

                    Button::new(cx, |cx|{
                        Label::new(cx, "BOTTOM-END")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::BottomEnd));
                }).vertical_gap(gap_8).size(auto);
            }, r#"IconButton::new(cx, ICON_TRASH).tooltip(|cx| Tooltip::new(cx, |cx|{
    Label::new(cx, "Delete");
}));"#
        );

    }).class("panel");
}

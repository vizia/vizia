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

    VStack::new(cx, |cx|{

        Markdown::new(cx, "# Tooltip
A tooltip displays supplemental information near its target view. Tooltips are triggered on hover or focus of the target view and dismissed on blur or mouse-out of the target or tooltip container.        
        ");

        Markdown::new(cx, "### Basic tooltip");

        DemoRegion::new(
            cx,
            |cx| {
                Button::new(cx, |cx |Svg::new(cx, ICON_TRASH))
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::static_text(cx, "Delete");
                    }));
            }, r#"IconButton::new(cx, ICON_TRASH)
    .tooltip(|cx| Tooltip::new(cx, |cx|{
        Label::static_text(cx, "Delete");
    }));"#
        );

        Markdown::new(cx, "### Tooltip content");

        DemoRegion::new(
            cx,
            |cx| {
                Button::new(cx, |cx |Svg::new(cx, ICON_TRASH))
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        HStack::new(cx, |cx|{
                            Svg::new(cx, ICON_INFO_CIRCLE);
                            Label::static_text(cx, "Delete");
                        }).size(auto).alignment(align_left);
                    }));
            }, r#"IconButton::new(cx, ICON_TRASH)
    .tooltip(|cx| Tooltip::new(cx, |cx|{
        Label::static_text(cx, "Delete");
    }));"#
        );

        Markdown::new(cx, "### Tooltip placement");

        DemoRegion::new(
            cx,
            |cx| {
                let text_variant = cx.state(ButtonVariant::Text);

                VStack::new(cx, |cx|{
                    Button::new(cx, |cx|{
                        Label::static_text(cx, "TOP-START")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::static_text(cx, "Tooltip");
                    }).placement(Placement::TopStart));

                    Button::new(cx, |cx|{
                        Label::static_text(cx, "LEFT-START")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::static_text(cx, "Tooltip");
                    }).placement(Placement::LeftStart));

                    Button::new(cx, |cx|{
                        Label::static_text(cx, "RIGHT-START")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::static_text(cx, "Tooltip");
                    }).placement(Placement::RightStart));

                    Button::new(cx, |cx|{
                        Label::static_text(cx, "BOTTOM-START")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::static_text(cx, "Tooltip");
                    }).placement(Placement::BottomStart));
                }).vertical_gap(gap_8).size(auto);

                VStack::new(cx, |cx|{
                    Button::new(cx, |cx|{
                        Label::static_text(cx, "TOP")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::static_text(cx, "Tooltip");
                    }).placement(Placement::Top));

                    Button::new(cx, |cx|{
                        Label::static_text(cx, "LEFT")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::static_text(cx, "Tooltip");
                    }).placement(Placement::Left));

                    Button::new(cx, |cx|{
                        Label::static_text(cx, "RIGHT")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::static_text(cx, "Tooltip");
                    }).placement(Placement::Right));

                    Button::new(cx, |cx|{
                        Label::static_text(cx, "BOTTOM")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::static_text(cx, "Tooltip");
                    }).placement(Placement::Bottom));
                }).vertical_gap(gap_8).size(auto);

                VStack::new(cx, |cx|{
                    Button::new(cx, |cx|{
                        Label::static_text(cx, "TOP-END")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::static_text(cx, "Tooltip");
                    }).placement(Placement::TopEnd));

                    Button::new(cx, |cx|{
                        Label::static_text(cx, "LEFT-END")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::static_text(cx, "Tooltip");
                    }).placement(Placement::LeftEnd));

                    Button::new(cx, |cx: &mut Context|{
                        Label::static_text(cx, "RIGHT-END")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::static_text(cx, "Tooltip");
                    }).placement(Placement::RightEnd));

                    Button::new(cx, |cx|{
                        Label::static_text(cx, "BOTTOM-END")
                    })
                    .variant(text_variant)
                    .width(width_140)
                    .height(height_60)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::static_text(cx, "Tooltip");
                    }).placement(Placement::BottomEnd));
                }).vertical_gap(gap_8).size(auto);
            }, r#"IconButton::new(cx, ICON_TRASH).tooltip(|cx| Tooltip::new(cx, |cx|{
    Label::static_text(cx, "Delete");
}));"#
        );

    }).class("panel");
}

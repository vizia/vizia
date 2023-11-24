use vizia::{icons::ICON_TRASH, prelude::*};

use crate::components::DemoRegion;

pub fn tooltip(cx: &mut Context) {
    VStack::new(cx, |cx|{

        Label::new(cx, "Tooltip").class("title");
        Label::new(cx, "A tooltip displays supplemental information near its target view. Tooltips are triggered on hover or focus of the target view and dismissed on blur or mouse-out of the target or tooltip container.")
            .class("paragraph");

        Label::new(cx, "Basic tooltip").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                IconButton::new(cx, ICON_TRASH).tooltip(|cx| Tooltip::new(cx, |cx|{
                    Label::new(cx, "Delete");
                }));
            },
            |cx| {
                Label::new(cx, r#"IconButton::new(cx, ICON_TRASH).tooltip(|cx| Tooltip::new(cx, |cx|{
    Label::new(cx, "Delete");
}));"#).class("code");
            },
        );

        Label::new(cx, "Tooltip placement").class("header");
        DemoRegion::new(
            cx,
            |cx| {

                VStack::new(cx, |cx|{
                    Button::new(cx, |cx|{
                        Label::new(cx, "TOP-START")
                    })
                    .variant(ButtonVariant::Text)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::TopStart));
                    
                    Button::new(cx, |cx|{
                        Label::new(cx, "LEFT-START")
                    })
                    .variant(ButtonVariant::Text)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::LeftStart));

                    Button::new(cx, |cx|{
                        Label::new(cx, "RIGHT-START")
                    })
                    .variant(ButtonVariant::Text)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::RightStart));

                    Button::new(cx, |cx|{
                        Label::new(cx, "BOTTOM-START")
                    })
                    .variant(ButtonVariant::Text)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::BottomStart));
                }).row_between(Pixels(8.0)).size(Auto);

                VStack::new(cx, |cx|{
                    Button::new(cx, |cx|{
                        Label::new(cx, "TOP")
                    })
                    .variant(ButtonVariant::Text)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::Top));
                    
                    Button::new(cx, |cx|{
                        Label::new(cx, "LEFT")
                    })
                    .variant(ButtonVariant::Text)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::Left));

                    Button::new(cx, |cx|{
                        Label::new(cx, "RIGHT")
                    })
                    .variant(ButtonVariant::Text)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::Right));

                    Button::new(cx, |cx|{
                        Label::new(cx, "BOTTOM")
                    })
                    .variant(ButtonVariant::Text)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::Bottom));
                }).row_between(Pixels(8.0)).size(Auto);

                VStack::new(cx, |cx|{
                    Button::new(cx, |cx|{
                        Label::new(cx, "TOP-END")
                    })
                    .variant(ButtonVariant::Text)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::TopEnd));
                    
                    Button::new(cx, |cx|{
                        Label::new(cx, "LEFT-END")
                    })
                    .variant(ButtonVariant::Text)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::LeftEnd));

                    Button::new(cx, |cx: &mut Context|{
                        Label::new(cx, "RIGHT-END")
                    })
                    .variant(ButtonVariant::Text)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::RightEnd));

                    Button::new(cx, |cx|{
                        Label::new(cx, "BOTTOM-END")
                    })
                    .variant(ButtonVariant::Text)
                    .tooltip(|cx| Tooltip::new(cx, |cx|{
                        Label::new(cx, "Tooltip");
                    }).placement(Placement::BottomEnd));
                }).row_between(Pixels(8.0)).size(Auto);
                
            },
            |cx| {
                Label::new(cx, r#"IconButton::new(cx, ICON_TRASH).tooltip(|cx| Tooltip::new(cx, |cx|{
    Label::new(cx, "Delete");
}));"#).class("code");
            },
        );

    }).class("panel");
}

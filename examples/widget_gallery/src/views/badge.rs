use vizia::{
    icons::{ICON_CLOCK, ICON_USER},
    prelude::*,
};

use crate::components::DemoRegion;

pub fn badge(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("badge")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Basic Badge", |cx| {
            Avatar::new(cx, |cx| {
                Label::new(cx, "GA");
            })
            .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "14")));

            Avatar::new(cx, |cx| {
                Label::new(cx, "Messages");
            })
            .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "42")));
        });

        DemoRegion::new(cx, "Status Variants", |cx| {
            Avatar::new(cx, |cx| {
                Label::new(cx, "GA");
            })
            .badge(|cx| Badge::empty(cx).class("success"));

            Avatar::new(cx, |cx| {
                Label::new(cx, "GA");
            })
            .badge(|cx| Badge::empty(cx).class("warning"));

            Avatar::new(cx, |cx| {
                Label::new(cx, "GA");
            })
            .badge(|cx| Badge::empty(cx).class("error"));
        });

        DemoRegion::new(cx, "Badge Sizes", |cx| {
            Avatar::new(cx, |cx| {
                Label::new(cx, "GA");
            })
            .badge(|cx| {
                Badge::new(cx, |cx| Label::new(cx, "2")).control_size(ControlSize::ExtraSmall)
            });

            Avatar::new(cx, |cx| {
                Label::new(cx, "GA");
            })
            .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "8")).control_size(ControlSize::Small));

            Avatar::new(cx, |cx| {
                Label::new(cx, "GA");
            })
            .badge(|cx| {
                Badge::new(cx, |cx| Label::new(cx, "14")).control_size(ControlSize::Medium)
            });

            Avatar::new(cx, |cx| {
                Label::new(cx, "GA");
            })
            .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "99")).control_size(ControlSize::Large));
        });

        DemoRegion::new(cx, "Badge Placement", |cx| {
            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| {
                Badge::new(cx, |cx| Label::new(cx, "TL")).placement(BadgePlacement::TopLeft)
            });

            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "T")).placement(BadgePlacement::Top));

            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| {
                Badge::new(cx, |cx| Label::new(cx, "TR")).placement(BadgePlacement::TopRight)
            });

            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "L")).placement(BadgePlacement::Left));

            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "R")).placement(BadgePlacement::Right));

            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| {
                Badge::new(cx, |cx| Label::new(cx, "BL")).placement(BadgePlacement::BottomLeft)
            });

            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "B")).placement(BadgePlacement::Bottom));

            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| {
                Badge::new(cx, |cx| Label::new(cx, "BR")).placement(BadgePlacement::BottomRight)
            });
        });

        DemoRegion::new(cx, "Svg Badge", |cx| {
            Avatar::new(cx, |cx| {
                Label::new(cx, "GA");
            })
            .badge(|cx| {
                Badge::new(cx, |cx| Svg::new(cx, ICON_CLOCK))
                    .class("warning")
                    .placement(BadgePlacement::TopLeft)
            });

            Avatar::new(cx, |cx| {
                Label::new(cx, "GA");
            })
            .badge(|cx| {
                Badge::new(cx, |cx| Svg::new(cx, ICON_CLOCK))
                    .class("warning")
                    .placement(BadgePlacement::TopLeft)
            });

            Avatar::new(cx, |cx| {
                Label::new(cx, "GA");
            })
            .badge(|cx| {
                Badge::new(cx, |cx| Svg::new(cx, ICON_CLOCK))
                    .class("warning")
                    .placement(BadgePlacement::TopLeft)
            });
        });
    })
    .class("panel");
}

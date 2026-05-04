use vizia::{
    icons::{ICON_CLOCK, ICON_USER},
    prelude::*,
};

use crate::components::DemoRegion;

pub fn avatar(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("avatar")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Basic Avatar", |cx| {
            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            });
        });

        DemoRegion::new(cx, "Avatar Content", |cx| {
            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .control_size(ControlSize::Medium);

            Avatar::new(cx, |cx| {
                Label::new(cx, "GA");
            })
            .control_size(ControlSize::Medium);

            Avatar::new(cx, |cx| {
                Image::new(cx, "vizia.png");
            })
            .control_size(ControlSize::Medium);
        });

        DemoRegion::new(cx, "Avatar Sizes", |cx| {
            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .control_size(ControlSize::ExtraSmall);

            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .control_size(ControlSize::Small);

            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .control_size(ControlSize::Medium);

            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .control_size(ControlSize::Large);
        });

        DemoRegion::new(cx, "Avatar Variants", |cx| {
            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            });

            Avatar::new(cx, |cx| {
                Label::new(cx, "GA");
            })
            .variant(AvatarVariant::Square);

            Avatar::new(cx, |cx| {
                Image::new(cx, "vizia.png");
            })
            .variant(AvatarVariant::Rounded);
        });

        DemoRegion::new(cx, "Avatar with Badge", |cx| {
            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| {
                Badge::new(cx, |cx| Svg::new(cx, ICON_CLOCK))
                    .class("warning")
                    .placement(BadgePlacement::TopLeft)
            });

            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| Badge::empty(cx).class("error").placement(BadgePlacement::Right));

            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| Badge::empty(cx).class("success").placement(BadgePlacement::BottomLeft));

            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "2")).placement(BadgePlacement::Bottom));
        });
    })
    .class("panel");
}

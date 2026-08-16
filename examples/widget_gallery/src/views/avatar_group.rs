use vizia::{icons::ICON_USER, prelude::*};

use crate::components::DemoRegion;

pub fn avatar_group(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("avatar-group")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, Localized::new("demo-region-avatar-group"), |cx| {
            AvatarGroup::new(cx, |cx| {
                Avatar::new(cx, |cx| {
                    Svg::new(cx, ICON_USER);
                });

                Avatar::new(cx, |cx| {
                    Svg::new(cx, ICON_USER);
                });

                Avatar::new(cx, |cx| {
                    Svg::new(cx, ICON_USER);
                });
            });
        });

        DemoRegion::new(cx, Localized::new("demo-region-avatar-group-overflow"), |cx| {
            AvatarGroup::new(cx, |cx| {
                for initials in ["GA", "AB", "CD", "EF", "GH"] {
                    Avatar::new(cx, |cx| {
                        Label::new(cx, initials);
                    })
                    .control_size(ControlSize::Medium);
                }
            })
            .max_visible(3);
        });
    })
    .class("panel");
}

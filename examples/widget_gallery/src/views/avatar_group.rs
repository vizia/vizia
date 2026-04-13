use vizia::{icons::ICON_USER, prelude::*};

use crate::components::DemoRegion;

pub fn avatar_group(cx: &mut Context) {
    cx.load_image(
        "vizia.png",
        include_bytes!("../../resources/images/vizia-logo-01.png"),
        ImageRetentionPolicy::DropWhenNoObservers,
    );

    VStack::new(cx, |cx|{
        Markdown::new(cx, "# Avatar Group
An avatar group displays multiple avatars stacked together to represent a collection of users or entities.");

        Divider::new(cx);

        DemoRegion::new(cx, "Avatar Group", |cx|{
            AvatarGroup::new(cx, |cx|{
                Avatar::new(cx, |cx|{
                    Svg::new(cx, ICON_USER);
                });

                Avatar::new(cx, |cx|{
                    Svg::new(cx, ICON_USER);
                });

                Avatar::new(cx, |cx|{
                    Svg::new(cx, ICON_USER);
                });
            });
        });


    }).class("panel");
}

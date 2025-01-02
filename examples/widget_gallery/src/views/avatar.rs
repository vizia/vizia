use vizia::{
    icons::{ICON_CLOCK, ICON_USER},
    prelude::*,
};

use crate::components::DemoRegion;

pub fn avatar(cx: &mut Context) {
    cx.load_image(
        "vizia.png",
        include_bytes!("../../assets/vizia-logo-01.png"),
        ImageRetentionPolicy::DropWhenNoObservers,
    );

    VStack::new(cx, |cx|{
        Markdown::new(cx, "# Avatar
An avatar is used to visually represent a person or entity and can contain text, an icon, or an image.
        ");

        Divider::new(cx);

        Markdown::new(cx, "### Basic avatar");
        DemoRegion::new(cx, |cx|{
            Avatar::new(cx, |cx|{
                Svg::new(cx, ICON_USER);
            });
        },r#"Avatar::new(cx, |cx|{
    Svg::new(cx, ICON_USER)
});"#);

        Markdown::new(cx, "### Avatar content
An avatar can contain an icon, text, or an image.");

        DemoRegion::new(cx, |cx|{
            Avatar::new(cx, |cx|{
                Svg::new(cx, ICON_USER);
            });

            Avatar::new(cx, |cx|{
                Label::new(cx, "GA");
            });

            Avatar::new(cx, |cx|{
                Image::new(cx, "vizia.png");
            });
        }, r#"Avatar::new(cx, |cx|{
    Svg::new(cx, ICON_USER);
});

Avatar::new(cx, |cx|{
    Label::new(cx, "GA");
});

Avatar::new(cx, |cx|{
    Image::new(cx, "vizia.png");
});"#);


        Markdown::new(cx, "### Avatar variants
The `variant` modifier can be used to select between a circle (default), square, and rounded avatar shape.
        ");

        DemoRegion::new(cx, |cx|{
            Avatar::new(cx, |cx|{
                Svg::new(cx, ICON_USER);
            });

            Avatar::new(cx, |cx|{
                Label::new(cx, "GA");
            }).variant(AvatarVariant::Square);

            Avatar::new(cx, |cx|{
                Image::new(cx, "vizia.png");
            }).variant(AvatarVariant::Rounded);
        }, r#"Avatar::new(cx, |cx|{
    Svg::new(cx, ICON_USER);
});

Avatar::new(cx, |cx|{
    Label::new(cx, "GA");
}).variant(AvatarVariant::Square);

Avatar::new(cx, |cx|{
    Image::new(cx, "vizia.png");
}).variant(AvatarVariant::Rounded);"#);

        Markdown::new(cx, "### Avatar with badge
The badge modifier can be used to add a badge to an avatar.
        ");

        DemoRegion::new(cx, |cx|{
            Avatar::new(cx, |cx|{
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| Badge::new(cx, |cx| Svg::new(cx, ICON_CLOCK)).class("warning"));

            Avatar::new(cx, |cx|{
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| Badge::empty(cx).class("error"));

            Avatar::new(cx, |cx|{
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| Badge::empty(cx).class("success"));

            Avatar::new(cx, |cx|{
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "2")));
        }, r#"Avatar::new(cx, |cx|{
    Svg::new(cx, ICON_USER);
}).badge(|cx| Badge::new(cx, |cx| Svg::new(cx, ICON_CLOCK)).class("warning"));

Avatar::new(cx, |cx|{
    Svg::new(cx, ICON_USER);
}).badge(|cx| Badge::empty(cx).class("error"));

Avatar::new(cx, |cx|{
    Svg::new(cx, ICON_USER);
}).badge(|cx| Badge::empty(cx).class("success"));

Avatar::new(cx, |cx|{
    Svg::new(cx, ICON_USER);
}).badge(|cx| Badge::new(cx, |cx| Label::new(cx, "2")));"#
        );

    }).class("panel");
}

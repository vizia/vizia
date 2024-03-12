use vizia::{
    icons::{ICON_CLOCK, ICON_USER},
    image,
    prelude::*,
};

use crate::components::DemoRegion;

pub fn avatar(cx: &mut Context) {
    cx.load_image(
        "vizia.png",
        image::load_from_memory_with_format(
            include_bytes!("../../assets/vizia-logo-01.png"),
            image::ImageFormat::Png,
        )
        .unwrap(),
        ImageRetentionPolicy::DropWhenUnusedForOneFrame,
    );

    VStack::new(cx, |cx|{
        Label::new(cx, "Avatar").class("title");
        Label::new(cx, "An avatar is used to visually represent a person or entity and can contain text, an icon, or an image.").class("paragraph");

        Divider::new(cx)
            .top(Pixels(12.0))
            .bottom(Pixels(12.0));

        Label::new(cx, "Avatar").class("header");
        DemoRegion::new(cx, |cx|{
            Avatar::new(cx, |cx|{
                Icon::new(cx, ICON_USER);
            });
        },r#"Avatar::new(cx, |cx|{
    Icon::new(cx, ICON_USER)
});"#);

        Label::new(cx, "Avatar content").class("header");
        Label::new(cx, "An avatar can contain an icon, text, or an image.").class("paragraph");

        DemoRegion::new(cx, |cx|{
            Avatar::new(cx, |cx|{
                Icon::new(cx, ICON_USER);
            });

            Avatar::new(cx, |cx|{
                Label::new(cx, "GA");
            });

            Avatar::new(cx, |cx|{
                Image::new(cx, "vizia.png");
            });
        }, r#"Avatar::new(cx, |cx|{
    Icon::new(cx, ICON_USER);
});

Avatar::new(cx, |cx|{
    Label::new(cx, "GA");
});

Avatar::new(cx, |cx|{
    Image::new(cx, "vizia.png");
});"#);


        Label::new(cx, "Avatar variants").class("header");
        Label::new(cx, "The variant modifier can be used to select between a circle (default), square, and rounded avatar shape.").class("paragraph");

        DemoRegion::new(cx, |cx|{
            Avatar::new(cx, |cx|{
                Icon::new(cx, ICON_USER);
            });

            Avatar::new(cx, |cx|{
                Label::new(cx, "GA");
            }).variant(AvatarVariant::Square);

            Avatar::new(cx, |cx|{
                Image::new(cx, "vizia.png");
            }).variant(AvatarVariant::Rounded);
        }, r#"Avatar::new(cx, |cx|{
    Icon::new(cx, ICON_USER);
});

Avatar::new(cx, |cx|{
    Label::new(cx, "GA");
}).variant(AvatarVariant::Square);

Avatar::new(cx, |cx|{
    Image::new(cx, "vizia.png");
}).variant(AvatarVariant::Rounded);"#);

        Label::new(cx, "Avatar with badge").class("header");
        Label::new(cx, "The badge modifier can be used to add a badge to an avatar.").class("paragraph");


        DemoRegion::new(cx, |cx|{
            Avatar::new(cx, |cx|{
                Icon::new(cx, ICON_USER);
            })
            .badge(|cx| Badge::new(cx, |cx| Icon::new(cx, ICON_CLOCK)).class("warning"));

            Avatar::new(cx, |cx|{
                Icon::new(cx, ICON_USER);
            })
            .badge(|cx| Badge::empty(cx).class("error"));

            Avatar::new(cx, |cx|{
                Icon::new(cx, ICON_USER);
            })
            .badge(|cx| Badge::empty(cx).class("success"));

            Avatar::new(cx, |cx|{
                Icon::new(cx, ICON_USER);
            })
            .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "2")));
        }, r#"Avatar::new(cx, |cx|{
    Icon::new(cx, ICON_USER);
}).badge(|cx| Badge::new(cx, |cx| Icon::new(cx, ICON_CLOCK)).class("warning"));

Avatar::new(cx, |cx|{
    Icon::new(cx, ICON_USER);
}).badge(|cx| Badge::empty(cx).class("error"));

Avatar::new(cx, |cx|{
    Icon::new(cx, ICON_USER);
}).badge(|cx| Badge::empty(cx).class("success"));

Avatar::new(cx, |cx|{
    Icon::new(cx, ICON_USER);
}).badge(|cx| Badge::new(cx, |cx| Label::new(cx, "2")));"#
        );

    }).class("panel");
}

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

    VStack::new(cx, move |cx|{
        Markdown::new(cx, "# Avatar
An avatar is used to visually represent a person or entity and can contain text, an icon, or an image.
        ");

        Divider::new(cx);

        Markdown::new(cx, "### Basic avatar");
        DemoRegion::new(
            cx,
            move |cx|{
            Avatar::new(cx, |cx|{
                Svg::new(cx, ICON_USER);
            });
        },r#"Avatar::new(cx, |cx|{
    Svg::new(cx, ICON_USER)
});"#);

        Markdown::new(cx, "### Avatar content
An avatar can contain an icon, text, or an image.");

        DemoRegion::new(
            cx,
            move |cx|{
            let vizia_png = cx.state("vizia.png");

            Avatar::new(cx, |cx|{
                Svg::new(cx, ICON_USER);
            });

            Avatar::new(cx, |cx|{
                Label::new(cx, "GA");
            });

            Avatar::new(cx, |cx|{
                Image::new(cx, vizia_png);
            });
        }, r#"let vizia_png = cx.state("vizia.png");

Avatar::new(cx, |cx|{
    Svg::new(cx, ICON_USER);
});

Avatar::new(cx, |cx|{
    Label::new(cx, "GA");
});

Avatar::new(cx, |cx|{
    Image::new(cx, vizia_png);
});"#);


        Markdown::new(cx, "### Avatar variants
The `variant` modifier can be used to select between a circle (default), square, and rounded avatar shape.
        ");

        DemoRegion::new(
            cx,
            move |cx|{
            let square = cx.state(AvatarVariant::Square);
            let rounded = cx.state(AvatarVariant::Rounded);
            let vizia_png = cx.state("vizia.png");

            Avatar::new(cx, |cx|{
                Svg::new(cx, ICON_USER);
            });

            Avatar::new(cx, |cx|{
                Label::new(cx, "GA");
            }).variant(square);

            Avatar::new(cx, |cx|{
                Image::new(cx, vizia_png);
            }).variant(rounded);
        }, r#"let square = cx.state(AvatarVariant::Square);
let rounded = cx.state(AvatarVariant::Rounded);
let vizia_png = cx.state("vizia.png");

Avatar::new(cx, |cx|{
    Svg::new(cx, ICON_USER);
});

Avatar::new(cx, |cx|{
    Label::new(cx, "GA");
}).variant(square);

Avatar::new(cx, |cx|{
    Image::new(cx, vizia_png);
}).variant(rounded);"#);

        Markdown::new(cx, "### Avatar with badge
The badge modifier can be used to add a badge to an avatar.
        ");

        DemoRegion::new(
            cx,
            move |cx|{
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
}).badge(|cx| Badge::new(cx, |cx| {
    Label::new(cx, "2");
}));"#
        );

    }).class("panel");
}

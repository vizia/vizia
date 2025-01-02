use vizia::{
    icons::{ICON_CLOCK, ICON_USER},
    prelude::*,
};

use crate::components::DemoRegion;

pub fn avatar_group(cx: &mut Context) {
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

        Markdown::new(cx, "### Basic avatar group");
        DemoRegion::new(cx, |cx|{
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
            
        },r#"Avatar::new(cx, |cx|{
    Svg::new(cx, ICON_USER)
});"#);


    }).class("panel");
}

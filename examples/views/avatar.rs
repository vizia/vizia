mod helpers;
use helpers::*;

use vizia::{
    icons::{ICON_CLOCK, ICON_USER},
    prelude::*,
};

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.load_image(
            "sample-hut",
            include_bytes!("../resources/images/sample-hut-400x300.png"),
            ImageRetentionPolicy::DropWhenNoObservers,
        );

        ExamplePage::vertical(cx, |cx| {
            HStack::new(cx, |cx| {
                Avatar::new(cx, |cx| {
                    Svg::new(cx, ICON_USER);
                })
                .control_size(ControlSize::Small);

                Avatar::new(cx, |cx| {
                    Label::new(cx, "GA");
                })
                .control_size(ControlSize::Medium);

                Avatar::new(cx, |cx| {
                    Image::new(cx, "sample-hut");
                })
                .control_size(ControlSize::Large);
            })
            .height(Auto)
            .width(Stretch(1.0))
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(12.0));

            HStack::new(cx, |cx| {
                Avatar::new(cx, |cx| {
                    Svg::new(cx, ICON_USER);
                });

                Avatar::new(cx, |cx| {
                    Label::new(cx, "SQ");
                })
                .variant(AvatarVariant::Square);

                Avatar::new(cx, |cx| {
                    Image::new(cx, "sample-hut");
                })
                .variant(AvatarVariant::Rounded);
            })
            .height(Auto)
            .width(Stretch(1.0))
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(12.0));

            HStack::new(cx, |cx| {
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
                .badge(|cx| {
                    Badge::new(cx, |cx| Label::new(cx, "2")).placement(BadgePlacement::Bottom)
                });
            })
            .height(Auto)
            .width(Stretch(1.0))
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(12.0));

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
    .title(Localized::new("view-title-avatar"))
    .inner_size((600, 500))
    .run()
}

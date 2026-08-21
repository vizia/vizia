use vizia::prelude::*;

use crate::components::{
    DemoRegion, Skeleton, SkeletonAnimation, SkeletonBone, SkeletonModifiers, SkeletonShape,
    SkeletonVariant,
};

fn loaded_profile(cx: &mut Context) {
    Card::new(cx, |cx| {
        CardContent::new(cx, |cx| {
            Avatar::new(cx, |cx| {
                Label::new(cx, Localized::new("skeleton-profile-initials"));
            })
            .class("skeleton-real-avatar");
            VStack::new(cx, |cx| {
                Label::new(cx, Localized::new("skeleton-profile-name"))
                    .font_weight(FontWeightKeyword::SemiBold);
                Label::new(cx, Localized::new("skeleton-profile-status"))
                    .class("skeleton-real-muted");
            })
            .height(Auto)
            .gap(Pixels(5.0));
        })
        .layout_type(LayoutType::Row)
        .alignment(Alignment::Left);
    })
    .class("skeleton-real-card");
}

fn loaded_article(cx: &mut Context) {
    Card::new(cx, |cx| {
        CardHeader::new(cx, |cx| {
            Label::new(cx, Localized::new("skeleton-article-title"))
                .font_weight(FontWeightKeyword::Bold);
            Label::new(cx, Localized::new("skeleton-article-meta")).class("skeleton-real-muted");
        });
        CardContent::new(cx, |cx| {
            Label::new(cx, Localized::new("skeleton-article-copy")).class("skeleton-real-copy");
        });
    })
    .class("skeleton-real-card");
}

pub fn skeleton(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("skeleton")).class("panel-title");
        Label::new(cx, Localized::new("skeleton").attribute("description"))
            .class("panel-description");
        Divider::new(cx);

        DemoRegion::new(cx, Localized::new("skeleton-demo-interactive"), |cx| {
            let loading = Signal::new(true);
            VStack::new(cx, move |cx| {
                Button::new(cx, |cx| Label::new(cx, Localized::new("skeleton-toggle-loading")))
                    .variant(ButtonVariant::Outline)
                    .on_press(move |_cx| loading.update(|value| *value = !*value));
                Binding::new(cx, loading, move |cx| {
                    if loading.get() {
                        Skeleton::new(cx, SkeletonVariant::Article)
                            .animation(SkeletonAnimation::Blur)
                            .class("skeleton-real-card");
                    } else {
                        loaded_article(cx);
                    }
                });
            })
            .class("skeleton-interactive-demo");
        });

        DemoRegion::new(cx, Localized::new("skeleton-demo-comparisons"), |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, Localized::new("skeleton-profile-caption"))
                    .class("skeleton-example-caption");
                HStack::new(cx, |cx| {
                    Skeleton::new(cx, SkeletonVariant::Profile)
                        .animation(SkeletonAnimation::Wave)
                        .class("skeleton-real-card");
                    loaded_profile(cx);
                })
                .class("skeleton-comparison-row");
                Label::new(cx, Localized::new("skeleton-article-caption"))
                    .class("skeleton-example-caption");
                HStack::new(cx, |cx| {
                    Skeleton::new(cx, SkeletonVariant::Article)
                        .animation(SkeletonAnimation::Shimmer)
                        .class("skeleton-real-card");
                    loaded_article(cx);
                })
                .class("skeleton-comparison-row");
            })
            .class("skeleton-comparisons");
        });

        DemoRegion::new(cx, Localized::new("skeleton-demo-primitives"), |cx| {
            SkeletonBone::new(cx, SkeletonShape::Rectangle).size(Pixels(72.0));
            SkeletonBone::new(cx, SkeletonShape::Rounded).size(Pixels(72.0));
            SkeletonBone::new(cx, SkeletonShape::Circle).size(Pixels(72.0));
            SkeletonBone::new(cx, SkeletonShape::Rounded).width(Pixels(160.0)).height(Pixels(42.0));
        });

        DemoRegion::new(cx, Localized::new("skeleton-demo-presets"), |cx| {
            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    for variant in [
                        SkeletonVariant::Text,
                        SkeletonVariant::Circle,
                        SkeletonVariant::Rectangle,
                        SkeletonVariant::Button,
                        SkeletonVariant::Card,
                    ] {
                        Skeleton::new(cx, variant).class("skeleton-page-item");
                    }
                })
                .class("skeleton-page-row");
                HStack::new(cx, |cx| {
                    for variant in [
                        SkeletonVariant::Profile,
                        SkeletonVariant::List,
                        SkeletonVariant::Table,
                        SkeletonVariant::Article,
                        SkeletonVariant::Media,
                    ] {
                        Skeleton::new(cx, variant).class("skeleton-page-item");
                    }
                })
                .class("skeleton-page-row");
            })
            .class("skeleton-page-grid");
        });

        DemoRegion::new(cx, Localized::new("skeleton-demo-animations"), |cx| {
            for animation in [
                SkeletonAnimation::Pulse,
                SkeletonAnimation::Shimmer,
                SkeletonAnimation::Wave,
                SkeletonAnimation::Blur,
                SkeletonAnimation::Glow,
                SkeletonAnimation::Breathe,
            ] {
                Skeleton::new(cx, SkeletonVariant::Text)
                    .animation(animation)
                    .class("skeleton-animation-sample");
            }
        });
    })
    .class("panel")
    .class("loading-component-page");
}

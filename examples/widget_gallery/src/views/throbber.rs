use vizia::prelude::*;

use crate::components::{
    DemoRegion, GalleryThrobber, SpinnerDirection, TextThrobber, TextThrobberIndicator,
    TextThrobberVariant, ThrobberVariant,
};

pub fn throbber(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("throbber")).class("panel-title");
        Label::new(cx, Localized::new("throbber").attribute("description"))
            .class("panel-description");
        Divider::new(cx);

        DemoRegion::new(cx, Localized::new("throbber-demo-text"), |cx| {
            VStack::new(cx, |cx| {
                TextThrobber::localized_with_indicator(
                    cx,
                    "throbber-working-through-request",
                    TextThrobberIndicator::Dots,
                );
                TextThrobber::localized(
                    cx,
                    "throbber-generating-response",
                    TextThrobberVariant::Cursor,
                );
                TextThrobber::localized(
                    cx,
                    "throbber-loading-workspace",
                    TextThrobberVariant::Blur,
                );
                TextThrobber::localized(cx, "throbber-analysing", TextThrobberVariant::Tracking);
                TextThrobber::localized(
                    cx,
                    "throbber-working-through-request",
                    TextThrobberVariant::Progressive,
                );
            })
            .class("text-throbber-hero");
        });

        DemoRegion::new(cx, Localized::new("throbber-demo-directions"), |cx| {
            HStack::new(cx, |cx| {
                for (name, direction) in [
                    ("throbber-clockwise", SpinnerDirection::Clockwise),
                    ("throbber-counter-clockwise", SpinnerDirection::CounterClockwise),
                ] {
                    VStack::new(cx, move |cx| {
                        GalleryThrobber::new_with_direction(
                            cx,
                            ThrobberVariant::Spinner,
                            direction,
                        );
                        Label::new(cx, Localized::new(name))
                            .class("loading-variant-name")
                            .hoverable(false);
                    })
                    .class("loading-variant-tile");
                }
            })
            .class("spinner-direction-examples");
        });

        DemoRegion::new(cx, Localized::new("throbber-demo-graphic-variants"), |cx| {
            HStack::new(cx, |cx| {
                for (name, variant) in [
                    ("throbber-variant-dots", ThrobberVariant::Dots),
                    ("throbber-variant-ring", ThrobberVariant::Ring),
                    ("throbber-variant-waveform", ThrobberVariant::Bars),
                    ("throbber-variant-orbit", ThrobberVariant::Orbit),
                    ("throbber-variant-pulse", ThrobberVariant::Pulse),
                    ("throbber-variant-spinner", ThrobberVariant::Spinner),
                    ("throbber-variant-grid", ThrobberVariant::Grid),
                    ("throbber-variant-bounce", ThrobberVariant::Bounce),
                    ("throbber-variant-ripple", ThrobberVariant::Ripple),
                    ("throbber-variant-equalizer", ThrobberVariant::Equalizer),
                ] {
                    VStack::new(cx, move |cx| {
                        GalleryThrobber::new(cx, variant);
                        Label::new(cx, Localized::new(name))
                            .class("loading-variant-name")
                            .hoverable(false);
                    })
                    .class("loading-variant-tile");
                }
            })
            .class("loading-variant-grid");
        });

        DemoRegion::new(cx, Localized::new("throbber-demo-text-variants"), |cx| {
            VStack::new(cx, |cx| {
                for row in [
                    [
                        ("throbber-text-loading", TextThrobberVariant::Fade),
                        ("throbber-text-thinking", TextThrobberVariant::Blink),
                        ("throbber-text-preparing", TextThrobberVariant::Blur),
                        ("throbber-text-connecting", TextThrobberVariant::Slide),
                        ("throbber-text-processing", TextThrobberVariant::Pulse),
                    ],
                    [
                        ("throbber-text-indexing", TextThrobberVariant::Tracking),
                        ("throbber-text-syncing", TextThrobberVariant::Bounce),
                        ("throbber-text-generating", TextThrobberVariant::Glow),
                        ("throbber-text-typing", TextThrobberVariant::Cursor),
                        ("throbber-text-waiting", TextThrobberVariant::Ellipsis),
                    ],
                ] {
                    HStack::new(cx, move |cx| {
                        for (key, variant) in row {
                            TextThrobber::localized(cx, key, variant).class("text-loading-sample");
                        }
                    })
                    .class("text-loading-row");
                }
                HStack::new(cx, |cx| {
                    TextThrobber::localized(
                        cx,
                        "throbber-text-composing",
                        TextThrobberVariant::Progressive,
                    )
                    .class("text-loading-sample");
                })
                .class("text-loading-row");
            })
            .class("text-loading-grid");
        });
    })
    .class("panel")
    .class("loading-component-page");
}

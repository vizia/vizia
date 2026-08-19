use vizia::prelude::*;

#[derive(Clone, Copy)]
pub enum ThrobberVariant {
    Dots,
    Ring,
    Bars,
    Orbit,
    Pulse,
    Spinner,
    Grid,
    Bounce,
    Ripple,
    Equalizer,
}

#[derive(Clone, Copy, Default)]
pub enum SpinnerDirection {
    #[default]
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Copy)]
pub enum TextThrobberVariant {
    Static,
    Fade,
    Blink,
    Blur,
    Slide,
    Pulse,
    Tracking,
    Bounce,
    Glow,
    Cursor,
    Ellipsis,
    Progressive,
}

#[derive(Clone, Copy, Default)]
pub enum TextThrobberIndicator {
    #[default]
    None,
    Dots,
    Cursor,
}

pub struct GalleryThrobber;
pub struct TextThrobber;

impl GalleryThrobber {
    pub fn new(cx: &mut Context, variant: ThrobberVariant) -> Handle<'_, Self> {
        Self::new_with_direction(cx, variant, SpinnerDirection::Clockwise)
    }

    pub fn new_with_direction(
        cx: &mut Context,
        variant: ThrobberVariant,
        direction: SpinnerDirection,
    ) -> Handle<'_, Self> {
        Self.build(cx, move |cx| match variant {
            ThrobberVariant::Dots => dots(cx),
            ThrobberVariant::Ring => ring(cx),
            ThrobberVariant::Bars => bars(cx),
            ThrobberVariant::Orbit => orbit(cx),
            ThrobberVariant::Pulse => pulse(cx),
            ThrobberVariant::Spinner => spinner(cx, direction),
            ThrobberVariant::Grid => grid(cx),
            ThrobberVariant::Bounce => bounce(cx),
            ThrobberVariant::Ripple => ripple(cx),
            ThrobberVariant::Equalizer => equalizer(cx),
        })
        .class("gallery-throbber")
    }
}

// The gallery exercises the localized constructors; these literal-text constructors are the
// reusable component API demonstrated by the page.
#[allow(dead_code)]
impl TextThrobber {
    pub fn new<'a>(
        cx: &'a mut Context,
        text: &'static str,
        variant: TextThrobberVariant,
    ) -> Handle<'a, Self> {
        let indicator = match variant {
            TextThrobberVariant::Cursor => TextThrobberIndicator::Cursor,
            TextThrobberVariant::Ellipsis => TextThrobberIndicator::Dots,
            _ => TextThrobberIndicator::None,
        };
        Self::new_composed(cx, text, variant, indicator)
    }

    pub fn with_indicator<'a>(
        cx: &'a mut Context,
        text: &'static str,
        indicator: TextThrobberIndicator,
    ) -> Handle<'a, Self> {
        Self::new_composed(cx, text, TextThrobberVariant::Static, indicator)
    }

    pub fn localized<'a>(
        cx: &'a mut Context,
        key: &'static str,
        variant: TextThrobberVariant,
    ) -> Handle<'a, Self> {
        let indicator = match variant {
            TextThrobberVariant::Cursor => TextThrobberIndicator::Cursor,
            TextThrobberVariant::Ellipsis => TextThrobberIndicator::Dots,
            _ => TextThrobberIndicator::None,
        };
        Self::localized_composed(cx, key, variant, indicator)
    }

    pub fn localized_with_indicator<'a>(
        cx: &'a mut Context,
        key: &'static str,
        indicator: TextThrobberIndicator,
    ) -> Handle<'a, Self> {
        Self::localized_composed(cx, key, TextThrobberVariant::Static, indicator)
    }

    pub fn localized_composed<'a>(
        cx: &'a mut Context,
        key: &'static str,
        variant: TextThrobberVariant,
        indicator: TextThrobberIndicator,
    ) -> Handle<'a, Self> {
        let text = Localized::new(key).to_signal(cx);
        Self.build(cx, move |cx| {
            HStack::new(cx, move |cx| {
                Binding::new(cx, text, move |cx| {
                    let localized_text = text.get();
                    text_content(cx, &localized_text, variant);
                });
            })
            .class("text-throbber-content")
            .class(text_variant_class(variant));
            text_indicator(cx, indicator);
        })
        .class("text-throbber")
        .class(match indicator {
            TextThrobberIndicator::None => "text-throbber-without-indicator",
            TextThrobberIndicator::Dots => "text-throbber-with-dots",
            TextThrobberIndicator::Cursor => "text-throbber-with-cursor",
        })
    }

    pub fn new_composed<'a>(
        cx: &'a mut Context,
        text: &'static str,
        variant: TextThrobberVariant,
        indicator: TextThrobberIndicator,
    ) -> Handle<'a, Self> {
        Self.build(cx, move |cx| {
            HStack::new(cx, move |cx| {
                text_content(cx, text, variant);
            })
            .class("text-throbber-content")
            .class(text_variant_class(variant));
            text_indicator(cx, indicator);
        })
        .class("text-throbber")
        .class(match indicator {
            TextThrobberIndicator::None => "text-throbber-without-indicator",
            TextThrobberIndicator::Dots => "text-throbber-with-dots",
            TextThrobberIndicator::Cursor => "text-throbber-with-cursor",
        })
    }
}

fn text_content(cx: &mut Context, text: &str, variant: TextThrobberVariant) {
    if matches!(variant, TextThrobberVariant::Progressive) {
        for (index, character) in text.chars().enumerate() {
            let character = if character == ' ' { '\u{00a0}' } else { character };
            Label::new(cx, character.to_string())
                .class("text-throbber-letter")
                .class(match index % 12 {
                    0 => "letter-delay-0",
                    1 => "letter-delay-1",
                    2 => "letter-delay-2",
                    3 => "letter-delay-3",
                    4 => "letter-delay-4",
                    5 => "letter-delay-5",
                    6 => "letter-delay-6",
                    7 => "letter-delay-7",
                    8 => "letter-delay-8",
                    9 => "letter-delay-9",
                    10 => "letter-delay-10",
                    _ => "letter-delay-11",
                })
                .hoverable(false);
        }
    } else {
        Label::new(cx, text.to_string()).hoverable(false);
    }
}

fn text_indicator(cx: &mut Context, indicator: TextThrobberIndicator) {
    if matches!(indicator, TextThrobberIndicator::Cursor) {
        Element::new(cx).class("text-throbber-cursor");
    }
    if matches!(indicator, TextThrobberIndicator::Dots) {
        HStack::new(cx, |cx| {
            for class in ["delay-0", "delay-1", "delay-2"] {
                Element::new(cx).class("throbber-dot").class("text-throbber-dot").class(class);
            }
        })
        .class("text-throbber-dots");
    }
}

fn text_variant_class(variant: TextThrobberVariant) -> &'static str {
    match variant {
        TextThrobberVariant::Static => "text-throbber-static",
        TextThrobberVariant::Fade => "text-throbber-fade",
        TextThrobberVariant::Blink => "text-throbber-blink",
        TextThrobberVariant::Blur => "text-throbber-blur",
        TextThrobberVariant::Slide => "text-throbber-slide",
        TextThrobberVariant::Pulse => "text-throbber-pulse",
        TextThrobberVariant::Tracking => "text-throbber-tracking",
        TextThrobberVariant::Bounce => "text-throbber-bounce",
        TextThrobberVariant::Glow => "text-throbber-glow",
        TextThrobberVariant::Cursor => "text-throbber-static",
        TextThrobberVariant::Ellipsis => "text-throbber-static",
        TextThrobberVariant::Progressive => "text-throbber-progressive",
    }
}

impl View for TextThrobber {
    fn element(&self) -> Option<&'static str> {
        Some("text-throbber")
    }
}

impl View for GalleryThrobber {
    fn element(&self) -> Option<&'static str> {
        Some("gallery-throbber")
    }
}

fn dots(cx: &mut Context) {
    HStack::new(cx, |cx| {
        for class in ["delay-0", "delay-1", "delay-2"] {
            Element::new(cx).class("throbber-dot").class(class);
        }
    })
    .class("throbber-dots");
}

fn ring(cx: &mut Context) {
    Element::new(cx).class("throbber-ring");
}

fn bars(cx: &mut Context) {
    HStack::new(cx, |cx| {
        for class in ["delay-0", "delay-1", "delay-2", "delay-3"] {
            Element::new(cx).class("throbber-bar").class(class);
        }
    })
    .class("throbber-bars");
}

fn orbit(cx: &mut Context) {
    ZStack::new(cx, |cx| {
        Element::new(cx).class("throbber-orbit-track");
        Element::new(cx).class("throbber-orbit-dot");
    })
    .class("throbber-orbit");
}

fn pulse(cx: &mut Context) {
    ZStack::new(cx, |cx| {
        Element::new(cx).class("throbber-pulse-wave");
        Element::new(cx).class("throbber-pulse-core");
    })
    .class("throbber-pulse");
}

fn spinner(cx: &mut Context, direction: SpinnerDirection) {
    ZStack::new(cx, |cx| {
        for class in [
            "spinner-0",
            "spinner-1",
            "spinner-2",
            "spinner-3",
            "spinner-4",
            "spinner-5",
            "spinner-6",
            "spinner-7",
            "spinner-8",
            "spinner-9",
            "spinner-10",
            "spinner-11",
        ] {
            ZStack::new(cx, |cx| {
                Element::new(cx).class("throbber-spinner-arm");
            })
            .class("throbber-spinner-segment")
            .class(class);
        }
    })
    .class("throbber-spinner")
    .class(match direction {
        SpinnerDirection::Clockwise => "spinner-clockwise",
        SpinnerDirection::CounterClockwise => "spinner-counter-clockwise",
    });
}

fn grid(cx: &mut Context) {
    VStack::new(cx, |cx| {
        for row in 0..3 {
            HStack::new(cx, move |cx| {
                for col in 0..3 {
                    Element::new(cx).class("throbber-grid-dot").class(match (row + col) % 3 {
                        0 => "delay-0",
                        1 => "delay-1",
                        _ => "delay-2",
                    });
                }
            })
            .class("throbber-grid-row");
        }
    })
    .class("throbber-grid");
}

fn bounce(cx: &mut Context) {
    HStack::new(cx, |cx| {
        for class in ["delay-0", "delay-1", "delay-2"] {
            Element::new(cx).class("throbber-bounce-dot").class(class);
        }
    })
    .class("throbber-bounce");
}

fn ripple(cx: &mut Context) {
    ZStack::new(cx, |cx| {
        Element::new(cx).class("throbber-ripple-wave").class("delay-0");
        Element::new(cx).class("throbber-ripple-wave").class("delay-2");
    })
    .class("throbber-ripple");
}

fn equalizer(cx: &mut Context) {
    HStack::new(cx, |cx| {
        for class in ["equalizer-0", "equalizer-1", "equalizer-2", "equalizer-3", "equalizer-4"] {
            Element::new(cx).class("throbber-equalizer-bar").class(class);
        }
    })
    .class("throbber-equalizer");
}

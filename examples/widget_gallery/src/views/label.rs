use vizia::prelude::*;

use crate::DemoRegion;

pub fn label(cx: &mut Context) {
    VStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, Localized::new("label")).class("panel-title");
            Label::new(cx, Localized::new("label").attribute("description"))
                .class("panel-description");
        })
        .height(Auto)
        .gap(Pixels(4.0));

        Divider::new(cx);

        DemoRegion::new(cx, "Basic Label", |cx| {
            Label::new(cx, "Hello Vizia");
        });

        DemoRegion::new(cx, "Multiline Label", |cx| {
            Label::new(
                cx,
                "This is a long label which will wrap onto multiple lines when the available width is not big enough to fit all of the text on a single line.",
            )
            .width(Pixels(240.0))
            .text_wrap(true);
        });

        DemoRegion::new(cx, "Rich Text Label", |cx| {
            Label::rich(cx, "Hello", |cx| {
                TextSpan::new(cx, " Rich", |cx| {
                    TextSpan::new(cx, " Text", |_| {})
                        .font_slant(FontSlant::Italic)
                        .color(Color::red());
                })
                .font_weight(FontWeightKeyword::Bold);
            });
        });

        DemoRegion::new(cx, "Label with Emoji", |cx| {
            Label::new(cx, "Hello 👋 Vizia 🎉🦀✨");
        });

        DemoRegion::new(cx, "Label with CJK", |cx| {
            Label::new(cx, "こんにちは 你好 안녕하세요");
        });

        DemoRegion::new(cx, "Disabled Label", |cx| {
            Label::new(cx, "Disabled Label").disabled(true);
        });

        DemoRegion::new(cx, "Localized Label", |cx| {
            Label::new(cx, Localized::new("label"));
        });

        DemoRegion::new(cx, "Text Ellipsis", |cx| {
            Label::new(cx, "This label is too long to fit and gets truncated with an ellipsis")
                .width(Pixels(150.0))
                .text_wrap(false)
                .text_overflow(TextOverflow::Ellipsis);
        });
    })
    .class("panel");
}

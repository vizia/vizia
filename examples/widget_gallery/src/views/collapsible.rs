use vizia::prelude::*;

use crate::DemoRegion;

pub fn collapsible(cx: &mut Context) {
    let open = Signal::new(true);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("collapsible")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Controlled Collapsible", move |cx| {
            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Button::new(cx, |cx| Label::new(cx, "Toggle"))
                        .on_press(move |_cx| open.set(!open.get()));
                })
                .height(Auto);

                Collapsible::new(
                    cx,
                    |cx| {
                        Label::new(cx, "Section A — click to expand/collapse").hoverable(false);
                    },
                    |cx| {
                        Label::new(
                            cx,
                            "This content belongs to Section A. \
                             It is shown when the section is open.",
                        )
                        .hoverable(false)
                        .text_wrap(true);
                    },
                )
                .open(open);
            })
            .height(Auto)
            .width(Stretch(1.0))
            .gap(Pixels(8.0));
        });

        DemoRegion::new(cx, "Self-managed Collapsible", |cx| {
            VStack::new(cx, |cx| {
                Collapsible::new(
                    cx,
                    |cx| {
                        Label::new(cx, "Section A — manages its own state").hoverable(false);
                    },
                    |cx| {
                        Label::new(cx, "Section A content.").hoverable(false);
                    },
                );

                Divider::new(cx);

                Collapsible::new(
                    cx,
                    |cx| {
                        Label::new(cx, "Section B — independent from Section A").hoverable(false);
                    },
                    |cx| {
                        Label::new(cx, "Section B content.").hoverable(false);
                    },
                );
            })
            .height(Auto)
            .width(Stretch(1.0));
        });
    })
    .class("panel");
}

use vizia::prelude::*;

use crate::DemoRegion;

pub fn popover(cx: &mut Context) {
    let is_open = Signal::new(false);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("popover")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Basic Popover", move |cx| {
            // The HStack acts as the anchor – Popover is absolutely positioned within it.
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Open Popover"))
                    .on_press(move |_cx| is_open.set(true));

                Binding::new(cx, is_open, move |cx| {
                    if is_open.get() {
                        Popover::new(cx, move |cx| {
                            VStack::new(cx, |cx| {
                                Label::new(cx, "Popover Content");
                                Label::new(cx, "Click outside or press Close to dismiss.");
                                Button::new(cx, |cx| Label::new(cx, "Close"))
                                    .on_press(move |_cx| is_open.set(false));
                            })
                            .padding(Pixels(16.0))
                            .gap(Pixels(8.0))
                            .size(Auto);
                        })
                        .on_blur(move |_cx| is_open.set(false))
                        .placement(Placement::Bottom)
                        .show_arrow(true);
                    }
                });
            })
            .size(Auto);
        });

        DemoRegion::new(cx, "Placement Options", |cx| {
            HStack::new(cx, |cx| {
                for (label, placement) in [
                    ("Top", Placement::Top),
                    ("Top Start", Placement::TopStart),
                    ("Top End", Placement::TopEnd),
                    ("Bottom Start", Placement::BottomStart),
                    ("Bottom", Placement::Bottom),
                    ("Bottom End", Placement::BottomEnd),
                    ("Left Start", Placement::LeftStart),
                    ("Left End", Placement::LeftEnd),
                    ("Left", Placement::Left),
                    ("Right Start", Placement::RightStart),
                    ("Right End", Placement::RightEnd),
                    ("Right", Placement::Right),
                ] {
                    let open = Signal::new(false);
                    HStack::new(cx, move |cx| {
                        Button::new(cx, move |cx| Label::new(cx, label))
                            .on_press(move |_cx| open.set(true));
                        Binding::new(cx, open, move |cx| {
                            if open.get() {
                                Popover::new(cx, move |cx| {
                                    Label::new(cx, format!("Placement: {}", label))
                                        .padding(Pixels(12.0));
                                })
                                .on_blur(move |_cx| open.set(false))
                                .placement(placement)
                                .show_arrow(true);
                            }
                        });
                    })
                    .size(Auto);
                }
            })
            .height(Auto)
            .wrap(LayoutWrap::Wrap)
            .gap(Pixels(8.0));
        });
    })
    .class("panel");
}

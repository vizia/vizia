use vizia::prelude::*;

use crate::DemoRegion;

pub fn popup(cx: &mut Context) {
    let is_open = Signal::new(false);

    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# Popup
A popup displays floating content anchored to a trigger element. \
It closes when focus moves outside.",
        );

        Divider::new(cx);

        Markdown::new(cx, "### Basic Popup");

        DemoRegion::new(cx, "Basic Popup", move |cx| {
            // The HStack acts as the anchor – Popup is absolutely positioned within it.
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Open Popup"))
                    .on_press(move |_cx| is_open.set(true));

                Binding::new(cx, is_open, move |cx| {
                    if is_open.get() {
                        Popover::new(cx, move |cx| {
                            VStack::new(cx, |cx| {
                                Label::new(cx, "Popup Content");
                                Label::new(cx, "Click outside or press Close to dismiss.")
                                    .text_wrap(true);
                                Button::new(cx, |cx| Label::new(cx, "Close"))
                                    .on_press(move |_cx| is_open.set(false));
                            })
                            .padding(Pixels(16.0))
                            .gap(Pixels(8.0))
                            .height(Auto);
                        })
                        .on_blur(move |_cx| is_open.set(false))
                        .placement(Placement::Bottom)
                        .show_arrow(true);
                    }
                });
            })
            .size(Auto);
        });

        Markdown::new(cx, "### Placement Options");

        DemoRegion::new(cx, "Placement Options", |cx| {
            HStack::new(cx, |cx| {
                for (label, placement) in [
                    ("Top", Placement::Top),
                    ("Bottom", Placement::Bottom),
                    ("Left", Placement::Left),
                    ("Right", Placement::Right),
                ] {
                    let open = Signal::new(false);
                    HStack::new(cx, move |cx| {
                        Button::new(cx, move |cx| Label::new(cx, label))
                            .on_press(move |_cx| open.set(true));
                        Binding::new(cx, open, move |cx| {
                            if open.get() {
                                Popover::new(cx, move |cx| {
                                    Label::new(cx, label).padding(Pixels(12.0));
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
            .gap(Pixels(8.0));
        });
    })
    .class("panel");
}

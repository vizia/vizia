use vizia::prelude::*;

use crate::DemoRegion;

pub fn resizable(cx: &mut Context) {
    let width = Signal::new(Pixels(200.0));
    let height = Signal::new(Pixels(100.0));

    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# Resizable
A resizable wraps a view with a draggable handle that lets the user change its size. \
Double-click the handle to reset.",
        );

        Divider::new(cx);

        Markdown::new(cx, "### Horizontal Resize (Right handle)");

        DemoRegion::new(cx, "Horizontal Resize", move |cx| {
            HStack::new(cx, |cx| {
                Resizable::new(
                    cx,
                    width,
                    ResizeStackDirection::Right,
                    move |_cx, w| width.set(Pixels(w)),
                    |cx| {
                        Element::new(cx).class("gallery-resizable-panel").size(Stretch(1.0));
                    },
                )
                .on_reset(move |_cx| width.set(Pixels(200.0)))
                .min_width(Pixels(80.0))
                .max_width(Pixels(460.0))
                .height(Pixels(80.0));

                Element::new(cx).class("gallery-resizable-fill").size(Stretch(1.0));
            })
            .height(Pixels(80.0))
            .width(Stretch(1.0));
        });

        Markdown::new(cx, "### Vertical Resize (Bottom handle)");

        DemoRegion::new(cx, "Vertical Resize", move |cx| {
            VStack::new(cx, |cx| {
                Resizable::new(
                    cx,
                    height,
                    ResizeStackDirection::Bottom,
                    move |_cx, h| height.set(Pixels(h)),
                    |cx| {
                        Element::new(cx).class("gallery-resizable-panel").size(Stretch(1.0));
                    },
                )
                .on_reset(move |_cx| height.set(Pixels(100.0)))
                .min_height(Pixels(40.0))
                .max_height(Pixels(300.0))
                .width(Stretch(1.0));

                Element::new(cx).class("gallery-resizable-fill").height(Stretch(1.0));
            })
            .height(Pixels(200.0))
            .width(Stretch(1.0));
        });
    })
    .class("panel");
}

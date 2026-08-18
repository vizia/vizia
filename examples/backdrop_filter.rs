use vizia::prelude::*;

const STYLE: &str = r#"
    :root {
        background-color: #111318;
        color: #fafafa;
        padding: 40px;
    }

    .demo {
        size: 1s;
    }

    .backdrop {
        size: 1s;
        corner-radius: 36px;
        overflow: hidden;
    }

    .backdrop > * {
        width: 1s;
    }

    .stripe-dark { background-color: #0a0a0a; }
    .stripe-light { background-color: #fafafa; }
    .stripe-green { background-color: #22c55e; }
    .stripe-gray { background-color: #525252; }

    .glass {
        position-type: absolute;
        width: 430px;
        height: 240px;
        space: 1s;
        rotate: -4deg;
        padding: 28px;
        row-gap: 12px;
        background-color: #26262673;
        border: 1px solid #ffffff38;
        corner-radius: 32px;
        overflow: hidden;
        backdrop-filter: blur(28px);
    }

    .popover-anchor {
        position-type: absolute;
        left: 270px;
        top: 174px;
        size: auto;
    }

    .title {
        font-size: 24px;
        font-weight: 700;
    }

    .description {
        width: 1s;
        height: auto;
        color: #d4d4d4;
        font-size: 15px;
    }
"#;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("backdrop filter example CSS must parse");
        let popover_open = Signal::new(true);

        ZStack::new(cx, move |cx| {
            HStack::new(cx, |cx| {
                Element::new(cx).class("stripe-dark");
                Element::new(cx).class("stripe-light");
                Element::new(cx).class("stripe-green");
                Element::new(cx).class("stripe-gray");
            })
            .class("backdrop");

            VStack::new(cx, move |cx| {
                Label::new(cx, "Rounded backdrop filter").class("title");
                Label::new(
                    cx,
                    "The transformed blur must stay inside the rounded panel. The real popover must escape it.",
                )
                .class("description")
                .text_wrap(true);

                HStack::new(cx, move |cx| {
                    Button::new(cx, |cx| Label::new(cx, "Open popover"))
                        .on_press(move |_cx| popover_open.set(true));

                    Binding::new(cx, popover_open, move |cx| {
                        if popover_open.get() {
                            Popover::new(cx, move |cx| {
                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Escaping overlay").class("title");
                                    Label::new(
                                        cx,
                                        "This must remain visible outside the clipped glass panel.",
                                    )
                                    .class("description")
                                    .text_wrap(true);
                                    Button::new(cx, |cx| Label::new(cx, "Close"))
                                        .on_press(move |_cx| popover_open.set(false));
                                })
                                .width(Pixels(230.0))
                                .height(Auto)
                                .padding(Pixels(16.0))
                                .gap(Pixels(10.0));
                            })
                            .placement(Placement::Right)
                            .show_arrow(true);
                        }
                    });
                })
                .class("popover-anchor");
            })
            .class("glass");
        })
        .class("demo");
    })
    .title("Vizia backdrop-filter clipping")
    .inner_size((900, 560))
    .run()
}

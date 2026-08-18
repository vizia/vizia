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
        width: 360px;
        height: 190px;
        space: 1s;
        padding: 28px;
        row-gap: 12px;
        background-color: #26262673;
        border: 1px solid #ffffff38;
        corner-radius: 32px;
        overflow: hidden;
        backdrop-filter: blur(28px);
    }

    .title {
        font-size: 24px;
        font-weight: 700;
    }

    .description {
        color: #d4d4d4;
        font-size: 15px;
    }
"#;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("backdrop filter example CSS must parse");

        ZStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Element::new(cx).class("stripe-dark");
                Element::new(cx).class("stripe-light");
                Element::new(cx).class("stripe-green");
                Element::new(cx).class("stripe-gray");
            })
            .class("backdrop");

            VStack::new(cx, |cx| {
                Label::new(cx, "Rounded backdrop filter").class("title");
                Label::new(
                    cx,
                    "The blur must remain inside this rounded rectangle without a rectangular halo.",
                )
                .class("description");
            })
            .class("glass");
        })
        .class("demo");
    })
    .title("Vizia backdrop-filter clipping")
    .inner_size((760, 480))
    .run()
}

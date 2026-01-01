use vizia::prelude::*;

const STYLE: &str = r#"
    resizable-stack {
        background-color: #b1b1b1;
    }

    resizable-stack.vertical {
        background-color: #878787;
        min-width: 100px;
        max-width: 500px;
    }

    resizable-stack.horizontal {
        min-height: 100px;
        max-height: 500px;
    }

    resizable-stack > resize-handle {
        background-color: #73a3cd;
        opacity: 0;
    }

    resizable-stack:active > resize-handle,
    resizable-stack > resize-handle:hover {
        opacity: 1;
        transition: opacity 200ms 200ms ease-in-out;
    }
"#;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let width = cx.state(Pixels(200.0));
        let height = cx.state(Pixels(200.0));

        ResizableStack::new(
            cx,
            height,
            ResizeStackDirection::Bottom,
            move |cx, h| height.set(cx, Pixels(h)),
            move |cx| {
                ResizableStack::new(
                    cx,
                    width,
                    ResizeStackDirection::Right,
                    move |cx, w| width.set(cx, Pixels(w)),
                    |_cx| {},
                )
                .on_reset(move |cx| {
                    width.set(cx, Pixels(200.0));
                });
            },
        )
        .on_reset(move |cx| {
            height.set(cx, Pixels(200.0));
        });
        (cx.state("Resizable Stack"), cx.state((800, 600)))
    });

    app.title(title).inner_size(size).run()
}

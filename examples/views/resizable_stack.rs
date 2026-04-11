mod helpers;
use helpers::*;
use vizia::prelude::*;

const STYLE: &str = r#"
    .demo-root {
        gap: 16px;
        padding: 16px;
    }

    .demo-column,
    .demo-row {
        gap: 16px;
    }

    .demo-panel {
        child-space: 1s;
        background-color: #dcdcdc;
    }

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

fn build_demo_stack(cx: &mut Context, title: &'static str) {
    Element::new(cx).class("demo-panel");
    Label::new(cx, title);
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let left_width = Signal::new(Pixels(180.0));
        let right_width = Signal::new(Pixels(180.0));
        let top_height = Signal::new(Pixels(140.0));
        let bottom_height = Signal::new(Pixels(140.0));

        ExamplePage::new(cx, |cx| {
            VStack::new(cx, |cx| {
                ResizableStack::new(
                    cx,
                    top_height,
                    ResizeStackDirection::Top,
                    move |_cx, h| top_height.set(Pixels(h)),
                    |cx| build_demo_stack(cx, "Top"),
                )
                .on_reset(move |_cx| {
                    top_height.set(Pixels(140.0));
                });

                ResizableStack::new(
                    cx,
                    bottom_height,
                    ResizeStackDirection::Bottom,
                    move |_cx, h| bottom_height.set(Pixels(h)),
                    |cx| build_demo_stack(cx, "Bottom"),
                )
                .on_reset(move |_cx| {
                    bottom_height.set(Pixels(140.0));
                });
            })
            .class("demo-column")
            .size(Stretch(1.0));

            VStack::new(cx, |cx| {
                ResizableStack::new(
                    cx,
                    left_width,
                    ResizeStackDirection::Left,
                    move |_cx, w| left_width.set(Pixels(w)),
                    |cx| build_demo_stack(cx, "Left"),
                )
                .on_reset(move |_cx| {
                    left_width.set(Pixels(180.0));
                });

                ResizableStack::new(
                    cx,
                    right_width,
                    ResizeStackDirection::Right,
                    move |_cx, w| right_width.set(Pixels(w)),
                    |cx| build_demo_stack(cx, "Right"),
                )
                .on_reset(move |_cx| {
                    right_width.set(Pixels(180.0));
                });
            })
            .class("demo-row")
            .size(Stretch(1.0));
        })
        .class("demo-root")
        .size(Stretch(1.0));
    })
    .title("Resizable Stack")
    .inner_size((800, 600))
    .run()
}

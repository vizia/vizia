//! This example showcases how keyboard inputs are propagated up the tree.
//!
//! Pressing on either a child or a sibling focuses it. If the focus is on a
//! child then the keyboard inputs get propagated up to the `InputView` which
//! prints the pressed key to the console. If the focus is on a sibling then the
//! keyboard inputs also go up the tree, but the `InputView` doesn't receive them
//! because it is not the parent of the siblings.

use vizia::prelude::*;

const STYLE: &str = r#"
    .input_view {
        width: auto;
        height: auto;
    }

    vstack {
        width: auto;
        height: auto;
    }

    .border {
        border-color: black;
        border-width: 1px;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);

        HStack::new(cx, |cx| {
            // View receiving keyboard events
            InputView::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    Label::new(cx, "Child 1");
                    VStack::new(cx, |cx| {
                        Label::new(cx, "Child 2");
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Child 3");
                        })
                        .class("border")
                        .on_press(|cx| cx.focus());
                    })
                    .class("border")
                    .on_press(|cx| cx.focus());
                })
                .class("border")
                .on_press(|cx| cx.focus());
            })
            .class("input_view");

            // Siblings
            Label::new(cx, "Sibling 1").on_press(|cx| cx.focus()).class("border");
            Label::new(cx, "Sibling 2").on_press(|cx| cx.focus()).class("border");
            Label::new(cx, "Sibling 3").on_press(|cx| cx.focus()).class("border");
        })
        .height(Pixels(150.0));
    })
    .title("Key input propagation")
    .run();
}

struct InputView;

impl InputView {
    pub fn new<F>(cx: &mut Context, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        Self {}.build(cx, |cx| {
            (content)(cx);
        })
    }
}

impl View for InputView {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::KeyDown(code, _) => {
                println!("The key {:?} got pressed!", code);
            }
            WindowEvent::KeyUp(code, _) => {
                println!("The key {:?} got released!", code);
            }
            _ => {}
        });
    }
}

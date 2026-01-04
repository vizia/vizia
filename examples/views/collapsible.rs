use vizia::prelude::*;
mod helpers;
use helpers::*;

fn main() -> Result<(), ApplicationError> {
    CollapsibleApp::run()
}

struct CollapsibleApp {
    collapsed: Signal<bool>,
}

impl App for CollapsibleApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            collapsed: cx.state(false),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let collapsed = self.collapsed;

        let content_long =
            "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10";
        let content_short = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";

        ExamplePage::vertical(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "Toggle collapsed"))
                .on_press(move |cx| collapsed.update(cx, |collapsed| *collapsed = !*collapsed));

            VStack::new(cx, |cx| {
                // First collapsible
                Collapsible::new(
                    cx,
                    |cx| {
                        Label::new(cx, "Click me to collapse the content").hoverable(false);
                    },
                    |cx| {
                        Label::new(cx, content_long).hoverable(false);
                    },
                )
                .open(collapsed);

                Divider::new(cx);

                // Second collapsible
                Collapsible::new(
                    cx,
                    |cx| {
                        Label::new(cx, "Click me to collapse the content").hoverable(false);
                    },
                    |cx| {
                        Label::new(cx, content_short).hoverable(false);
                    },
                )
                .open(collapsed);

                Divider::new(cx);

                // Third collapsible with buttons
                Collapsible::new(
                    cx,
                    |cx| {
                        Label::new(cx, "Click me to collapse the content").hoverable(false);
                    },
                    |cx| {
                        Label::new(cx, content_short).hoverable(false);
                        Divider::new(cx);
                        HStack::new(cx, |cx| {
                            Button::new(cx, |cx| Label::new(cx, "CANCEL")).variant(ButtonVariant::Text);
                            Button::new(cx, |cx| Label::new(cx, "SAVE")).variant(ButtonVariant::Text);
                        })
                        .height(Auto)
                        .gap(Pixels(8.0))
                        .padding_right(Pixels(8.0))
                        .alignment(Alignment::Right);
                    },
                )
                .open(collapsed);
            })
            .alignment(Alignment::TopCenter);
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Collapsible"))
    }
}

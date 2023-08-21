mod helpers;
use helpers::*;

use vizia::icons::ICON_CHECK;
use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        ExamplePage::vertical(cx, |cx| {
            HStack::new(cx, |cx| {
                // Basic Button
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Button"));
                // Accent Button
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Accent Button"))
                    .variant(ButtonVariant::Accent);
                // Outline Button
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Outline Button"))
                    .variant(ButtonVariant::Outline);
                // Ghost Button
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Text Button"))
                    .variant(ButtonVariant::Text);
                // Button with Icon
                Button::new(
                    cx,
                    |_| {},
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, ICON_CHECK).class("icon");
                            Label::new(cx, "Button with Icon");
                        })
                    },
                );
            })
            .size(Auto)
            .col_between(Pixels(10.0));

            HStack::new(cx, |cx| {
                IconButton::new(cx, |_| {}, ICON_CHECK);
                IconButton::new(cx, |_| {}, ICON_CHECK).variant(ButtonVariant::Accent);
                IconButton::new(cx, |_| {}, ICON_CHECK).variant(ButtonVariant::Outline);
                IconButton::new(cx, |_| {}, ICON_CHECK).variant(ButtonVariant::Text);
            })
            .size(Auto)
            .col_between(Pixels(10.0));

            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |_| {}, |cx| Label::new(cx, "ONE"));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "TWO"));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "THREE"));
            });

            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |_| {}, |cx| Label::new(cx, "ONE"));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "TWO"));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "THREE"));
            })
            .variant(ButtonVariant::Accent);

            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |_| {}, |cx| Label::new(cx, "ONE"));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "TWO"));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "THREE"));
            })
            .variant(ButtonVariant::Outline);

            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |_| {}, |cx| Label::new(cx, "ONE"));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "TWO"));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "THREE"));
            })
            .variant(ButtonVariant::Text);

            ButtonGroup::new(cx, |cx| {
                IconButton::new(cx, |_| {}, ICON_CHECK);
                IconButton::new(cx, |_| {}, ICON_CHECK);
                IconButton::new(cx, |_| {}, ICON_CHECK);
            });

            ButtonGroup::new(cx, |cx| {
                IconButton::new(cx, |_| {}, ICON_CHECK);
                IconButton::new(cx, |_| {}, ICON_CHECK);
                IconButton::new(cx, |_| {}, ICON_CHECK);
            })
            .variant(ButtonVariant::Accent);

            ButtonGroup::new(cx, |cx| {
                IconButton::new(cx, |_| {}, ICON_CHECK);
                IconButton::new(cx, |_| {}, ICON_CHECK);
                IconButton::new(cx, |_| {}, ICON_CHECK);
            })
            .variant(ButtonVariant::Outline);

            ButtonGroup::new(cx, |cx| {
                IconButton::new(cx, |_| {}, ICON_CHECK);
                IconButton::new(cx, |_| {}, ICON_CHECK);
                IconButton::new(cx, |_| {}, ICON_CHECK);
            })
            .variant(ButtonVariant::Text);
        });
    })
    .title("Button")
    .inner_size((700, 200))
    .run();
}

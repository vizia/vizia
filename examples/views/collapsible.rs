use vizia::prelude::*;
mod helpers;
use helpers::*;

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        let collapsed = cx.state(false);
        let auto = cx.state(Auto);
        let gap_8 = cx.state(Pixels(8.0));
        let padding_right_8 = cx.state(Pixels(8.0));
        let align_right = cx.state(Alignment::Right);
        let align_top_center = cx.state(Alignment::TopCenter);
        let not_hoverable = cx.state(false);

        let content_long =
            "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10";
        let content_short = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        let text_variant = cx.state(ButtonVariant::Text);

        ExamplePage::vertical(cx, |cx| {
            Button::new(cx, |cx| Label::static_text(cx, "Toggle collapsed"))
                .on_press(move |cx| collapsed.update(cx, |collapsed| *collapsed = !*collapsed));

            VStack::new(cx, |cx| {
                // First collapsible
                Collapsible::new(
                    cx,
                    |cx| {
                        Label::static_text(cx, "Click me to collapse the content")
                            .hoverable(not_hoverable);
                    },
                    |cx| {
                        Label::static_text(cx, content_long).hoverable(not_hoverable);
                    },
                )
                .open(collapsed);

                Divider::new(cx);

                // Second collapsible
                Collapsible::new(
                    cx,
                    |cx| {
                        Label::static_text(cx, "Click me to collapse the content")
                            .hoverable(not_hoverable);
                    },
                    |cx| {
                        Label::static_text(cx, content_short).hoverable(not_hoverable);
                    },
                )
                .open(collapsed);

                Divider::new(cx);

                // Third collapsible with buttons
                Collapsible::new(
                    cx,
                    |cx| {
                        Label::static_text(cx, "Click me to collapse the content")
                            .hoverable(not_hoverable);
                    },
                    |cx| {
                        Label::static_text(cx, content_short).hoverable(not_hoverable);
                        Divider::new(cx);
                        HStack::new(cx, |cx| {
                            Button::new(cx, |cx| Label::static_text(cx, "CANCEL"))
                                .variant(text_variant);
                            Button::new(cx, |cx| Label::static_text(cx, "SAVE"))
                                .variant(text_variant);
                        })
                        .height(auto)
                        .gap(gap_8)
                        .padding_right(padding_right_8)
                        .alignment(align_right);
                    },
                )
                .open(collapsed);
            })
            .alignment(align_top_center);
        });
        cx.state("Collapsible")
    });

    app.title(title).run()
}

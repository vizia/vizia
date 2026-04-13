use vizia::prelude::*;

use crate::DemoRegion;

pub fn accordion(cx: &mut Context) {
    let items = Signal::new(vec![
        (
            "What is Vizia?".to_string(),
            "Vizia is a declarative GUI framework for building desktop applications in Rust."
                .to_string(),
        ),
        (
            "How do I style views?".to_string(),
            "Use CSS-like stylesheets and class selectors to customise the appearance of views."
                .to_string(),
        ),
        (
            "Is Vizia reactive?".to_string(),
            "Yes! Vizia uses reactive signals so the UI updates automatically when data changes."
                .to_string(),
        ),
    ]);
    let open_index = Signal::new(Some(0usize));

    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# Accordion
An accordion displays a list of headers that can be expanded one at a time to reveal content.",
        );

        Divider::new(cx);

        DemoRegion::new(cx, "Accordion", move |cx| {
            Accordion::new(cx, items, |_cx, _index, item| {
                let header = item.0;
                let content = item.1;
                AccordionPair::new(
                    move |cx| {
                        Label::new(cx, header.clone()).hoverable(false);
                    },
                    move |cx| {
                        Label::new(cx, content.clone()).hoverable(false).text_wrap(true);
                    },
                )
            })
            .with_open(open_index)
            .width(Stretch(1.0));
        });
    })
    .class("panel");
}

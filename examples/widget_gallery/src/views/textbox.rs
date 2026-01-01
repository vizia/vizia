use vizia::prelude::*;

use crate::DemoRegion;

pub fn textbox(cx: &mut Context) {
    let textbox_text = cx.state("Hello Vizia".to_string());
    let width_100 = cx.state(Pixels(100.0));

    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# Textbox
A textbox can be used to display a string of text which can be edited.        
        ",
        );

        Divider::new(cx);

        Markdown::new(cx, "### Basic textbox");

        DemoRegion::new(
            cx,
            |cx| {
                Textbox::new(cx, textbox_text)
                    .on_submit(move |cx, text, _| textbox_text.set(cx, text.clone()))
                    .width(width_100);
            },
            r#"let textbox_text = cx.state("Hello Vizia".to_string());
let width_100 = cx.state(Pixels(100.0));
Textbox::new(cx, textbox_text)
    .on_submit(|cx, text, _| textbox_text.set(cx, text.clone()))
    .width(width_100);"#,
        );
    })
    .class("panel");
}

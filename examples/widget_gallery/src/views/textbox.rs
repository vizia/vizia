use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Lens)]
pub struct TextboxData {
    text: String,
}

pub enum TextboxEvent {
    SetText(String),
}

impl Model for TextboxData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|textbox_event, _| match textbox_event {
            TextboxEvent::SetText(text) => self.text = text.clone(),
        });
    }
}

pub fn textbox(cx: &mut Context) {
    TextboxData { text: "Hello Vizia".to_string() }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Textbox").class("title");
        Label::new(cx, "A textbox can be used to display a string of text which can be edited.")
            .class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Label").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Textbox::new(cx, TextboxData::text)
                    .on_submit(|cx, text, _| cx.emit(TextboxEvent::SetText(text.clone())))
                    .width(Pixels(100.0));
            },
            r#"Textbox::new(cx, TextboxData::text)
    .on_submit(|cx, text, _| cx.emit(TextboxEvent::SetText(text.clone())))
    .width(Pixels(100.0));"#,
        );
    })
    .class("panel");
}

use vizia::prelude::*;

use crate::DemoRegion;

pub struct TextboxData {
    text: Signal<String>,
}

pub enum TextboxEvent {
    SetText(String),
}

impl Model for TextboxData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|textbox_event, _| match textbox_event {
            TextboxEvent::SetText(text) => self.text.set(text.clone()),
        });
    }
}

pub fn textbox(cx: &mut Context) {
    let text = Signal::new("Hello Vizia".to_string());
    TextboxData { text }.build(cx);

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
            move |cx| {
                Textbox::new(cx, text)
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

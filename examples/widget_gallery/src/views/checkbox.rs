use vizia::prelude::*;

#[derive(Lens)]
pub struct CheckboxData {
    check: bool,
}

pub enum CheckboxEvent {
    Toggle,
}

impl Model for CheckboxData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            CheckboxEvent::Toggle => {
                self.check ^= true;
            }
        });
    }
}

pub fn checkbox(cx: &mut Context) {
    CheckboxData { check: false }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Checkbox").class("title");

        Checkbox::new(cx, CheckboxData::check).on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));

        HStack::new(cx, |cx| {
            Checkbox::new(cx, CheckboxData::check)
                .id("checky")
                .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));
            Label::new(cx, "Checkbox with label").describing("checky");
        })
        .size(Auto)
        .child_top(Stretch(1.0))
        .child_bottom(Stretch(1.0))
        .col_between(Pixels(5.0));
    })
    .class("panel");
}

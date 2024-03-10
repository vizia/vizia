use vizia::prelude::*;

use crate::components::DemoRegion;

#[derive(Lens)]
pub struct SwitchData {
    flag: bool,
}

pub enum SwitchEvent {
    ToggleFlag,
}

impl Model for SwitchData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            SwitchEvent::ToggleFlag => {
                self.flag ^= true;
            }
        });
    }
}

pub fn switch(cx: &mut Context) {
    SwitchData { flag: true }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Switch").class("title");
        Label::new(cx, "...").class("paragraph");

        // Divider here
        Element::new(cx)
            .height(Pixels(1.0))
            .background_color(Color::rgb(210, 210, 210))
            .top(Pixels(12.0))
            .bottom(Pixels(12.0));

        Label::new(cx, "Basic switch").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Switch::new(cx, SwitchData::flag).on_toggle(|cx| cx.emit(SwitchEvent::ToggleFlag));
            },
            r#"Switch::new(cx, SwitchData::flag)
    .on_toggle(|cx| cx.emit(SwitchEvent::ToggleFlag));"#,
        );
    })
    .class("panel");
}

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
        Markdown::new(cx, "# Switch");

        Divider::new(cx);

        Markdown::new(cx, "### Basic switch");

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

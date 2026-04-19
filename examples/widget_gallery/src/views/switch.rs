use vizia::prelude::*;

use crate::components::DemoRegion;

pub struct SwitchData {
    flag: Signal<bool>,
}

pub enum SwitchEvent {
    ToggleFlag,
}

impl Model for SwitchData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            SwitchEvent::ToggleFlag => {
                self.flag.update(|flag| *flag ^= true);
            }
        });
    }
}

pub fn switch(cx: &mut Context) {
    let flag = Signal::new(true);
    SwitchData { flag }.build(cx);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Switch");

        Divider::new(cx);

        Markdown::new(cx, "### Basic switch");

        DemoRegion::new(cx, "Basic Switch", move |cx| {
            Switch::new(cx, flag).on_toggle(|cx| cx.emit(SwitchEvent::ToggleFlag));
        });
    })
    .class("panel");
}

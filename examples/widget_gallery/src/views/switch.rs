use vizia::prelude::*;

use crate::components::DemoRegion;

#[derive(Lens)]
pub struct SwitchData {
    check_a: bool,
    check_b: bool,
    check_c: bool,
    check_d: bool,
}

pub enum SwitchEvent {
    ToggleA,
    ToggleB,
    ToggleC,
    ToggleD,
}

impl Model for SwitchData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            SwitchEvent::ToggleA => {
                self.check_a ^= true;
            }

            SwitchEvent::ToggleB => {
                self.check_b ^= true;
            }

            SwitchEvent::ToggleC => {
                self.check_c ^= true;
            }

            SwitchEvent::ToggleD => {
                self.check_d ^= true;
            }
        });
    }
}

pub fn switch(cx: &mut Context) {
    SwitchData { check_a: true, check_b: false, check_c: false, check_d: true }.build(cx);

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
                Switch::new(cx, SwitchData::check_a).on_toggle(|cx| cx.emit(SwitchEvent::ToggleA));
            },
            |cx| {
                Label::new(
                    cx,
                    r#"Avatar::new(cx, |cx|{
        Icon::new(cx, ICON_USER)
    })"#,
                )
                .class("code");
            },
        );
    })
    .class("panel");
}

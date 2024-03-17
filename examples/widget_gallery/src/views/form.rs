use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Lens)]
pub struct FormData {
    check_a: bool,
    check_b: bool,
    check_c: bool,
}

pub enum FormEvent {
    ToggleA,
    ToggleB,
    ToggleC,
}

impl Model for FormData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            FormEvent::ToggleA => {
                self.check_a ^= true;
            }

            FormEvent::ToggleB => {
                self.check_b ^= true;
            }

            FormEvent::ToggleC => {
                self.check_c ^= true;
            }
        });
    }
}

pub fn form(cx: &mut Context) {
    FormData { check_a: true, check_b: false, check_c: false }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Form Controls and Form Groups").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Form Control").class("header");

        DemoRegion::new(
            cx,
            |cx| {
                FormControl::new(
                    cx,
                    |cx| {
                        Checkbox::new(cx, FormData::check_a)
                            .on_toggle(|cx| cx.emit(FormEvent::ToggleA))
                    },
                    "Label",
                );
            },
            r#"FormControl::new(
    cx,
    |cx| {
        Checkbox::new(cx, FormData::check_a)
            .on_toggle(|cx| cx.emit(FormEvent::ToggleA))
    },
    "Label",
);"#,
        );

        Label::new(cx, "Form Group").class("header");

        DemoRegion::new(
            cx,
            |cx| {
                FormControl::new(
                    cx,
                    |cx| {
                        FormGroup::new(cx, "Gender", |cx| {
                            FormControl::new(
                                cx,
                                |cx| {
                                    Checkbox::new(cx, FormData::check_a)
                                        .on_toggle(|cx| cx.emit(FormEvent::ToggleA))
                                },
                                "Male",
                            );

                            FormControl::new(
                                cx,
                                |cx| {
                                    Checkbox::new(cx, FormData::check_b)
                                        .on_toggle(|cx| cx.emit(FormEvent::ToggleB))
                                },
                                "Female",
                            );

                            FormControl::new(
                                cx,
                                |cx| {
                                    Checkbox::new(cx, FormData::check_c)
                                        .on_toggle(|cx| cx.emit(FormEvent::ToggleC))
                                },
                                "Other",
                            );
                        })
                    },
                    "Label",
                );
            },
            r#"FormGroup::new(cx, "Gender", |cx|{
    FormControl::new(cx, |cx| {
        Checkbox::new(cx, FormData::check_a)
            .on_toggle(|cx| cx.emit(FormEvent::ToggleA))
    }, "Male");

    FormControl::new(cx, |cx| {
        Checkbox::new(cx, FormData::check_b)
            .on_toggle(|cx| cx.emit(FormEvent::ToggleB))
    }, "Female");

    FormControl::new(cx, |cx| {
        Checkbox::new(cx, FormData::check_c)
            .on_toggle(|cx| cx.emit(FormEvent::ToggleC))
    }, "Other");
});"#,
        );
    })
    .class("panel");
}

use vizia::icons::ICON_CODE;
use vizia::prelude::*;

use crate::components::DemoRegion;

#[derive(Lens)]
pub struct CheckboxData {
    check_a: bool,
    check_b: bool,
    check_c: bool,
    check_d: bool,
}

pub enum CheckboxEvent {
    ToggleA,
    ToggleB,
    ToggleC,
    ToggleD,
}

impl Model for CheckboxData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            CheckboxEvent::ToggleA => {
                self.check_a ^= true;
            }

            CheckboxEvent::ToggleB => {
                self.check_b ^= true;
            }

            CheckboxEvent::ToggleC => {
                self.check_c ^= true;
            }

            CheckboxEvent::ToggleD => {
                self.check_d ^= true;
            }
        });
    }
}

pub fn checkbox(cx: &mut Context) {
    CheckboxData { check_a: true, check_b: false, check_c: false, check_d: true }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Checkbox").class("title");
        Label::new(cx, "A checkbox can be used to display a boolean value, or to select one or more items from a set of options.")
            .class("paragraph");

        Label::new(cx, "Basic checkboxes").class("header");

        DemoRegion::new(cx, |cx|{
            Checkbox::new(cx, CheckboxData::check_a)
                .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA));
            Checkbox::new(cx, CheckboxData::check_b)
                .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleB));
            Checkbox::new(cx, CheckboxData::check_c)
                .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleC))
                .disabled(true);
            Checkbox::new(cx, CheckboxData::check_d)
                .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleD))
                .disabled(true);
        }, r#"Checkbox::new(cx, CheckboxData::check_a)
    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA));
Checkbox::new(cx, CheckboxData::check_b)
    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleB));
Checkbox::new(cx, CheckboxData::check_c)
    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleC))
    .disabled(true);
Checkbox::new(cx, CheckboxData::check_d)
    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleD))
    .disabled(true);"#);

        Label::new(cx, "Label").class("header");
        Label::new(cx, "A `FormControl` can be used to add a label to a checkbox. Pressing on the label will also toggle the corresponding checkbox.")
            .class("paragraph");

        DemoRegion::new(cx, |cx|{
            FormControl::new(cx, |cx| {
                Checkbox::new(cx, CheckboxData::check_a)
                    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA))
            }, "Label");

            FormControl::new(cx, |cx| {
                Checkbox::new(cx, CheckboxData::check_b)
                    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleB))
            }, "Embed");

            FormControl::new(cx, |cx| {
                Checkbox::new(cx, CheckboxData::check_c)
                    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleC))
            }, "Disabled")
            .disabled(true);
        }, r#"FormControl::new(cx, |cx| {
    Checkbox::new(cx, CheckboxData::check_a)
        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA))
}, "Label");

FormControl::new(cx, |cx| {
    Checkbox::new(cx, CheckboxData::check_b)
        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleB))
}, "Embed");

FormControl::new(cx, |cx| {
    Checkbox::new(cx, CheckboxData::check_c)
        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleC))
}, "Disabled")
.disabled(true);"#);

        Label::new(cx, "Form Group").class("header");
        Label::new(cx, "The describing modifier can be used to link a label to a particular checkbox. Pressing on the label will then toggle the corresponding checkbox.")
            .class("paragraph");

        DemoRegion::new(cx, |cx|{
            FormGroup::new(cx, "Options", |cx|{
                FormControl::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check_a)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA))
                }, "Left");

                FormControl::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check_b)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleB))
                }, "Middle");

                FormControl::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check_c)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleC))
                }, "Right");
            });
        }, r#"FormGroup::new(cx, "Gender", |cx|{
    FormControl::new(cx, |cx| {
        Checkbox::new(cx, CheckboxData::check_a)
            .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA))
    }, "Male");

    FormControl::new(cx, |cx| {
        Checkbox::new(cx, CheckboxData::check_b)
            .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleB))
    }, "Female");

    FormControl::new(cx, |cx| {
        Checkbox::new(cx, CheckboxData::check_c)
            .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleC))
    }, "Other");
});"#);

        Label::new(cx, "Label placement").class("header");
        Label::new(cx, "The describing modifier can be used to link a label to a particular checkbox. Pressing on the label will then toggle the corresponding checkbox.")
            .class("paragraph");

        DemoRegion::new(cx, |cx|{
            FormGroup::new(cx, "Options", |cx|{
                FormControl::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check_a)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA))
                }, "Left")
                .label_placement(FormPlacement::Start);

                FormControl::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check_b)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleB))
                }, "Middle");

                FormControl::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check_c)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleC))
                }, "Right");
            });
        },r#"FormGroup::new(cx, "Gender", |cx|{
    FormControl::new(cx, |cx| {
        Checkbox::new(cx, CheckboxData::check_a)
            .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA))
    }, "Male");

    FormControl::new(cx, |cx| {
        Checkbox::new(cx, CheckboxData::check_b)
            .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleB))
    }, "Female");

    FormControl::new(cx, |cx| {
        Checkbox::new(cx, CheckboxData::check_c)
            .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleC))
    }, "Other");
});"#);
    })
    .class("panel");
}

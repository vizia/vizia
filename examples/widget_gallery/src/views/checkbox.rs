use vizia::icons::ICON_CODE;
use vizia::prelude::*;

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

        Label::new(cx, r#"Checkbox::new(cx, AppData::flag)
    .on_toggle(|cx| cx.emit(AppEvent::ToggleFlag));"#).class("code");


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
        }, |cx|{
            Label::new(cx, r#"Checkbox::new(cx, CheckboxData::check_a)
    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA));
Checkbox::new(cx, CheckboxData::check_b)
    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleB));
Checkbox::new(cx, CheckboxData::check_c)
    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleC))
    .disabled(true);
Checkbox::new(cx, CheckboxData::check_d)
    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleD))
    .disabled(true);"#).class("code");
        });

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
        }, |cx|{
            Label::new(cx, r#"FormControl::new(cx, |cx| {
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
.disabled(true);"#).class("code");
        });

        Label::new(cx, "Form Group").class("header");
        Label::new(cx, "The describing modifier can be used to link a label to a particular checkbox. Pressing on the label will then toggle the corresponding checkbox.")
            .class("paragraph");

        DemoRegion::new(cx, |cx|{
            FormGroup::new(cx, "Gender", |cx|{
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
            });
        }, |cx|{
            Label::new(cx, r#"FormGroup::new(cx, "Gender", |cx|{
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
});"#).class("code");
        });

        Label::new(cx, "Label placement").class("header");
        Label::new(cx, "The describing modifier can be used to link a label to a particular checkbox. Pressing on the label will then toggle the corresponding checkbox.")
            .class("paragraph");

        DemoRegion::new(cx, |cx|{
            FormGroup::new(cx, "Gender", |cx|{
                FormControl::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check_a)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA))
                }, "Start").label_placement(Placement::Start);

                FormControl::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check_b)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleB))
                }, "Female");

                FormControl::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check_c)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleC))
                }, "Other");
            });
        }, |cx|{
            Label::new(cx, r#"FormGroup::new(cx, "Gender", |cx|{
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
});"#).class("code");
        });
    })
    .class("panel");
}

#[derive(Lens)]
pub struct DemoRegion {
    open: bool,
}

pub enum DemoRegionEvent {
    Toggle,
}

impl DemoRegion {
    pub fn new(
        cx: &mut Context,
        content: impl Fn(&mut Context),
        code: impl Fn(&mut Context),
    ) -> Handle<Self> {
        Self { open: false }.build(cx, |cx| {
            HStack::new(cx, |cx| {
                (content)(cx);
            })
            .class("region");
            // Element::new(cx).class("divider");
            HStack::new(cx, |cx| {
                (code)(cx);
            })
            .height(Auto)
            .display(DemoRegion::open);

            Button::new(cx, |ex| ex.emit(DemoRegionEvent::Toggle), |cx| Icon::new(cx, ICON_CODE))
                .space(Pixels(8.0))
                .left(Stretch(1.0))
                .position_type(PositionType::SelfDirected)
                .tooltip(|cx| {
                    Label::new(cx, "Toggle Dark/Light Mode");
                });
        })
    }
}

impl View for DemoRegion {
    fn element(&self) -> Option<&'static str> {
        Some("demo-region")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            DemoRegionEvent::Toggle => self.open ^= true,
        })
    }
}

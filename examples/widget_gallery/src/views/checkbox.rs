use vizia::prelude::*;

#[derive(Lens)]
pub struct CheckboxData {
    check1: bool,
    check2: bool,
}

pub enum CheckboxEvent {
    Toggle,
}

impl Model for CheckboxData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            CheckboxEvent::Toggle => {
                self.check1 ^= true;
                self.check2 ^= true;
            }
        });
    }
}

pub fn checkbox(cx: &mut Context) {
    CheckboxData { check1: true, check2: false }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Checkbox").class("title");
        Label::new(cx, "A checkbox can be used to display a boolean value, or to select one or more items from a set of options.")
        .class("paragraph");

        Label::new(cx, r#"Checkbox::new(cx, AppData::flag)
    .on_toggle(|cx| cx.emit(AppEvent::ToggleFlag));"#).class("code");


        Label::new(cx, "Basic checkboxes").class("header");

        HStack::new(cx, |cx|{
            Checkbox::new(cx, CheckboxData::check1).on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));
            Checkbox::new(cx, CheckboxData::check2).on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));
            Checkbox::new(cx, CheckboxData::check2).on_toggle(|cx| cx.emit(CheckboxEvent::Toggle)).disabled(true);
            Checkbox::new(cx, CheckboxData::check1).on_toggle(|cx| cx.emit(CheckboxEvent::Toggle)).disabled(true);
        }).class("region");

        Label::new(cx, "Checkbox and label").class("header");
        Label::new(cx, "The describing modifier can be used to link a label to a particular checkbox. Pressing on the label will then toggle the corresponding checkbox.")
        .class("paragraph");
        HStack::new(cx, |cx|{
            VStack::new(cx, |cx|{
                FormControl::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check1)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle))
                }, "Label");

                FormControl::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check2)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle))
                }, "Read-only");

                FormControl::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check1)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle))
                }, "Disabled")
                .disabled(true);
            }).class("group");
        }).class("region");

        HStack::new(cx, |cx|{
            FormGroup::new(cx, "Gender", |cx|{
                FormControl::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check1)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle))
                }, "Male");

                FormControl::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check1)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle))
                }, "Female");

                FormControl::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check1)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle))
                }, "Other");
            });
        }).class("region");
    })
    .class("panel");
}

pub struct DemoRegion {}

impl DemoRegion {
    pub fn new(
        cx: &mut Context,
        content: impl Fn(&mut Context),
        code: impl Fn(&mut Context),
    ) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            (content)(cx);
            Element::new(cx).class("divider");
            (code)(cx);
        })
    }
}

impl View for DemoRegion {}

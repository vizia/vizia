use vizia::prelude::*;

use crate::components::DemoRegion;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Options {
    First,
    Second,
    Third,
}

impl std::fmt::Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match *self {
            Options::First => "First",
            Options::Second => "Second",
            Options::Third => "Third",
        };
        write!(f, "{}", str)
    }
}

#[derive(Lens)]
pub struct RadioData {
    pub option: Options,
}

pub enum RadioEvent {
    SetOption(Options),
}

impl Model for RadioData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|radio_event, _| match radio_event {
            RadioEvent::SetOption(option) => {
                self.option = *option;
            }
        });
    }
}

pub fn radiobutton(cx: &mut Context) {
    RadioData { option: Options::First }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Radiobutton").class("title");
        Label::new(cx, "A radio button can be used to select an option from a set of options.")
        .class("paragraph");

        Label::new(cx, "Basic radiobutton").class("header");

        DemoRegion::new(
            cx,
            |cx| {
                RadioButton::new(cx, RadioData::option.map(|option| *option == Options::First)).on_select(|cx| cx.emit(RadioEvent::SetOption(Options::First)));
                RadioButton::new(cx, RadioData::option.map(|option| *option == Options::Second)).on_select(|cx| cx.emit(RadioEvent::SetOption(Options::Second)));
                RadioButton::new(cx, RadioData::option.map(|option| *option == Options::Third)).on_select(|cx| cx.emit(RadioEvent::SetOption(Options::Third)));
            }, r#"TODO"#
        );

        Label::new(cx, "Radiobutton and label").class("header");
        Label::new(cx, "The describing modifier can be used to link a label to a particular radiobutton. Pressing on the label will then toggle the corresponding radiobutton. Alternatively, a FormControl can be used.")
        .class("paragraph");

        DemoRegion::new(
            cx,
            |cx| {
                VStack::new(cx, |cx|{
                FormControl::new(cx, |cx| {
                    RadioButton::new(cx, RadioData::option.map(|option| *option == Options::First))
                        .on_select(|cx| cx.emit(RadioEvent::SetOption(Options::First)))
                }, "First");

                FormControl::new(cx, |cx| {
                    RadioButton::new(cx, RadioData::option.map(|option| *option == Options::Second))
                        .on_select(|cx| cx.emit(RadioEvent::SetOption(Options::Second)))
                }, "Second");

                FormControl::new(cx, |cx| {
                    RadioButton::new(cx, RadioData::option.map(|option| *option == Options::Third))
                        .on_select(|cx| cx.emit(RadioEvent::SetOption(Options::Third)))
                }, "Third")
                .disabled(true);
            }).class("group");
            }, r#"TODO"#
        );

        Label::new(cx, "Radiogroup").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                FormGroup::new(cx, "Gender", |cx|{
                    FormControl::new(cx, |cx| {
                        RadioButton::new(cx, RadioData::option.map(|option| *option == Options::First))
                            .on_select(|cx: &mut EventContext<'_>| cx.emit(RadioEvent::SetOption(Options::First)))
                    }, "Male");

                    FormControl::new(cx, |cx| {
                        RadioButton::new(cx, RadioData::option.map(|option| *option == Options::Second))
                            .on_select(|cx| cx.emit(RadioEvent::SetOption(Options::Second)))
                    }, "Female");

                    FormControl::new(cx, |cx|{
                        RadioButton::new(cx, RadioData::option.map(|option| *option == Options::Third))
                            .on_select(|cx| cx.emit(RadioEvent::SetOption(Options::Third)))
                    }, "Other");
                });
            }, r#"TODO"#
        );
    })
    .class("panel");
}

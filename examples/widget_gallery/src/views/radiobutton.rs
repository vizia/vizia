use vizia::prelude::*;

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
        Label::new(cx, "A checkbox can be used to display a boolean value, or to select one or more items from a set of options.")
        .class("paragraph");


        Label::new(cx, r#"RadioButton::new(cx, AppData::option.map(|opt| *opt == Opt::First))
    .on_select(|cx| cx.emit(AppEvent::SetOption(Opt::First)));"#).class("code");

        Label::new(cx, "Basic radiobutton").class("header");

        HStack::new(cx, |cx|{
            RadioButton::new(cx, RadioData::option.map(|option| *option == Options::First)).on_select(|cx| cx.emit(RadioEvent::SetOption(Options::First)));
            RadioButton::new(cx, RadioData::option.map(|option| *option == Options::Second)).on_select(|cx| cx.emit(RadioEvent::SetOption(Options::Second)));
            RadioButton::new(cx, RadioData::option.map(|option| *option == Options::Third)).on_select(|cx| cx.emit(RadioEvent::SetOption(Options::Third)));
        }).class("region");

        Label::new(cx, "Radiobutton and label").class("header");
        Label::new(cx, "The describing modifier can be used to link a label to a particular radiobutton. Pressing on the label will then toggle the corresponding radiobutton. Alternatively, a FormControl can be used.")
        .class("paragraph");
        HStack::new(cx, |cx|{
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
        }).class("region");

        Label::new(cx, r#"FormControl::new(cx, |cx| {
    RadioButton::new(cx, AppData::option.map(|opt| *opt == Opt::First))
        .on_select(|cx| cx.emit(RadioEvent::SetOption(Opt::First)))
}, "First");

FormControl::new(cx, |cx| {
    RadioButton::new(cx, AppData::option.map(|opt| *opt == Opt::Second))
        .on_select(|cx| cx.emit(RadioEvent::SetOption(Opt::Second)))
}, "Second");

FormControl::new(cx, |cx| {
    RadioButton::new(cx, AppData::option.map(|opt| *opt == Opt::Third))
        .on_select(|cx| cx.emit(RadioEvent::SetOption(Opt::Third)))
}, "Third");"#).class("code");

        Label::new(cx, "Radiogroup").class("header");
        HStack::new(cx, |cx|{
            FormGroup::new(cx, "Options", |cx|{
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
        }).class("region");
    })
    .class("panel");
}

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
        Markdown::new(cx, "# Radiobutton
A radio button can be used to select an option from a set of options.        
        ");

        Markdown::new(cx, "### Basic radio button");

        DemoRegion::new(
            cx,
            |cx| {
                RadioButton::new(cx, RadioData::option.map(|option| *option == Options::First)).on_select(|cx| cx.emit(RadioEvent::SetOption(Options::First)));
                RadioButton::new(cx, RadioData::option.map(|option| *option == Options::Second)).on_select(|cx| cx.emit(RadioEvent::SetOption(Options::Second)));
                RadioButton::new(cx, RadioData::option.map(|option| *option == Options::Third)).on_select(|cx| cx.emit(RadioEvent::SetOption(Options::Third)));
            }, r#"TODO"#
        );

        Markdown::new(cx, "### Radio button and label
The describing modifier can be used to link a label to a particular radiobutton. Pressing on the label will then toggle the corresponding radiobutton. Alternatively, a FormControl can be used.        
        ").class("header");

        DemoRegion::new(
            cx,
            |cx| {
                VStack::new(cx, |cx|{
                    HStack::new(cx, |cx| {
                        RadioButton::new(cx, RadioData::option.map(|option| *option == Options::First))
                            .on_select(|cx| cx.emit(RadioEvent::SetOption(Options::First)))
                            .id("r1");
                        Label::new(cx, "First").describing("r1");
                    })
                    .size(Auto)
                    .alignment(Alignment::Center)
                    .horizontal_gap(Pixels(8.0));

                    HStack::new(cx, |cx| {
                        RadioButton::new(cx, RadioData::option.map(|option| *option == Options::Second))
                            .on_select(|cx| cx.emit(RadioEvent::SetOption(Options::Second)))
                            .id("r2");
                        Label::new(cx, "Second").describing("r2");
                    })
                    .size(Auto)
                    .alignment(Alignment::Center)
                    .horizontal_gap(Pixels(8.0));

                    HStack::new(cx, |cx| {
                        RadioButton::new(cx, RadioData::option.map(|option| *option == Options::Third))
                            .on_select(|cx| cx.emit(RadioEvent::SetOption(Options::Third)))
                            .id("r3");
                        Label::new(cx, "Third").describing("r3");
                    })
                    .size(Auto)
                    .alignment(Alignment::Center)
                    .horizontal_gap(Pixels(8.0))
                    .disabled(true);
                })
                .vertical_gap(Pixels(4.0))
                .size(Auto);
            }, r#"TODO"#
        );
    })
    .class("panel");
}

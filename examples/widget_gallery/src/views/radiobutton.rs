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

pub fn radiobutton(cx: &mut Context) {
    let selected = cx.state(Options::First);
    let auto = cx.state(Auto);
    let align_center = cx.state(Alignment::Center);
    let gap_8 = cx.state(Pixels(8.0));
    let gap_4 = cx.state(Pixels(4.0));

    VStack::new(cx, move |cx| {
        Markdown::new(cx, "# Radiobutton
A radio button can be used to select an option from a set of options.        
        ");

        Markdown::new(cx, "### Basic radio button");

        DemoRegion::new(
            cx,
            move |cx| {
                let first_selected = selected.drv(cx, |v, _| *v == Options::First);
                let second_selected = selected.drv(cx, |v, _| *v == Options::Second);
                let third_selected = selected.drv(cx, |v, _| *v == Options::Third);

                RadioButton::new(cx, first_selected)
                    .on_select(move |cx| selected.set(cx, Options::First));
                RadioButton::new(cx, second_selected)
                    .on_select(move |cx| selected.set(cx, Options::Second));
                RadioButton::new(cx, third_selected)
                    .on_select(move |cx| selected.set(cx, Options::Third));
            },
            r#"let selected = cx.state(Options::First);

let first_selected = selected.drv(cx, |v, _| *v == Options::First);
let second_selected = selected.drv(cx, |v, _| *v == Options::Second);
let third_selected = selected.drv(cx, |v, _| *v == Options::Third);

RadioButton::new(cx, first_selected)
    .on_select(move |cx| selected.set(cx, Options::First));
RadioButton::new(cx, second_selected)
    .on_select(move |cx| selected.set(cx, Options::Second));
RadioButton::new(cx, third_selected)
    .on_select(move |cx| selected.set(cx, Options::Third));"#
        );

        Markdown::new(cx, "### Radio button and label
The describing modifier can be used to link a label to a particular radiobutton. Pressing on the label will then toggle the corresponding radiobutton. Alternatively, a FormControl can be used.        
        ").class("header");

        DemoRegion::new(
            cx,
            move |cx| {
                VStack::new(cx, move |cx|{
                    HStack::new(cx, move |cx| {
                        let first_selected = selected.drv(cx, |v, _| *v == Options::First);
                        RadioButton::new(cx, first_selected)
                            .on_select(move |cx| selected.set(cx, Options::First))
                            .id("r1");
                        Label::new(cx, "First").describing("r1");
                    })
                    .size(auto)
                    .alignment(align_center)
                    .horizontal_gap(gap_8);

                    HStack::new(cx, move |cx| {
                        let second_selected = selected.drv(cx, |v, _| *v == Options::Second);
                        RadioButton::new(cx, second_selected)
                            .on_select(move |cx| selected.set(cx, Options::Second))
                            .id("r2");
                        Label::new(cx, "Second").describing("r2");
                    })
                    .size(auto)
                    .alignment(align_center)
                    .horizontal_gap(gap_8);

                    HStack::new(cx, move |cx| {
                        let third_selected = selected.drv(cx, |v, _| *v == Options::Third);
                        RadioButton::new(cx, third_selected)
                            .on_select(move |cx| selected.set(cx, Options::Third))
                            .id("r3");
                        Label::new(cx, "Third").describing("r3");
                    })
                    .size(auto)
                    .alignment(align_center)
                    .horizontal_gap(gap_8)
                    .disabled(true);
                })
                .vertical_gap(gap_4)
                .size(auto);
            },
            r#"let selected = cx.state(Options::First);

VStack::new(cx, move |cx| {
    HStack::new(cx, move |cx| {
        let first_selected = selected.drv(cx, |v, _| *v == Options::First);
        RadioButton::new(cx, first_selected)
            .on_select(move |cx| selected.set(cx, Options::First))
            .id("r1");
        Label::new(cx, "First").describing("r1");
    });
});"#
        );
    })
    .class("panel");
}

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

#[derive(Lens, Model, Setter)]
pub struct AppData {
    pub option: Options,
}

fn main() {
    Application::new(|cx| {
        AppData { option: Options::First }.build(cx);

        // Exclusive checkboxes (radio buttons) with labels
        // Only one checkbox can be checked at a time and cannot be unchecked
        VStack::new(cx, |cx| {
            Label::new(cx, "Basic Radiobuttons");
            HStack::new(cx, |cx| {
                for i in 0..3 {
                    let current_option = index_to_option(i);
                    RadioButton::new(
                        cx,
                        AppData::option.map(move |option| *option == current_option),
                    )
                    .disabled(if i == 2 { true } else { false })
                    .on_select(move |cx| cx.emit(AppDataSetter::Option(current_option)));
                }
            })
            .col_between(Pixels(20.0));

            Label::new(cx, "Label").top(Pixels(20.0));

            for i in 0..3 {
                let current_option = index_to_option(i);
                HStack::new(cx, move |cx| {
                    RadioButton::new(
                        cx,
                        AppData::option.map(move |option| *option == current_option),
                    )
                    .on_select(move |cx| cx.emit(AppDataSetter::Option(current_option)))
                    .id(format!("button_{i}"));
                    Label::new(cx, &current_option.to_string()).describing(format!("button_{i}"));
                })
                .disabled(if i == 2 { true } else { false })
                .size(Auto)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(5.0));
            }
        })
        .row_between(Pixels(5.0))
        .space(Stretch(1.0))
        .size(Auto);
    })
    .title("Radiobutton")
    .run();
}

fn index_to_option(index: usize) -> Options {
    match index {
        0 => Options::First,
        1 => Options::Second,
        2 => Options::Third,
        _ => unreachable!(),
    }
}

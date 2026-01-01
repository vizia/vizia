mod helpers;
use helpers::*;
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

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        let selected = cx.state(Options::First);
        let auto = cx.state(Auto);
        let gap_20 = cx.state(Pixels(20.0));
        let gap_10 = cx.state(Pixels(10.0));
        let gap_5 = cx.state(Pixels(5.0));
        let top_20 = cx.state(Pixels(20.0));
        let align_center = cx.state(Alignment::Center);

        // Exclusive checkboxes (radio buttons) with labels
        // Only one checkbox can be checked at a time and cannot be unchecked
        ExamplePage::vertical(cx, |cx| {
            Label::static_text(cx, "Basic Radiobuttons");
            HStack::new(cx, |cx| {
                for i in 0..3 {
                    let current_option = index_to_option(i);
                    let is_selected = cx.derived({
                        let selected = selected;
                        move |store| *selected.get(store) == current_option
                    });
                    RadioButton::new(cx, is_selected)
                        .on_select(move |cx| selected.set(cx, current_option));
                }
            })
            .size(auto)
            .horizontal_gap(gap_20);

            Label::static_text(cx, "Radiobuttons with labels").top(top_20);

            VStack::new(cx, |cx| {
                for i in 0..3 {
                    let current_option = index_to_option(i);
                    HStack::new(cx, move |cx| {
                        let is_selected = cx.derived({
                            let selected = selected;
                            move |store| *selected.get(store) == current_option
                        });
                        RadioButton::new(cx, is_selected)
                            .on_select(move |cx| selected.set(cx, current_option))
                            .id(format!("button_{i}"));

                        let option_label = match current_option {
                            Options::First => "First",
                            Options::Second => "Second",
                            Options::Third => "Third",
                        };
                        Label::static_text(cx, option_label).describing(format!("button_{i}"));
                    })
                    .size(auto)
                    .alignment(align_center)
                    .horizontal_gap(gap_5);
                }
            })
            .vertical_gap(gap_10)
            .size(auto);
        });
        cx.state("Radiobutton")
    });

    app.title(title).run()
}

fn index_to_option(index: usize) -> Options {
    match index {
        0 => Options::First,
        1 => Options::Second,
        2 => Options::Third,
        _ => unreachable!(),
    }
}

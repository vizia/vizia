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

struct RadiobuttonApp {
    selected: Signal<Options>,
}

impl App for RadiobuttonApp {
    fn app_name() -> &'static str {
        "Radiobutton"
    }

    fn new(cx: &mut Context) -> Self {
        Self {
            selected: cx.state(Options::First),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let selected = self.selected;

        // Exclusive checkboxes (radio buttons) with labels
        // Only one checkbox can be checked at a time and cannot be unchecked
        ExamplePage::vertical(cx, move |cx| {
            Label::new(cx, "Basic Radiobuttons");
            HStack::new(cx, |cx| {
                for i in 0..3 {
                    let current_option = index_to_option(i);
                    let is_selected = selected.drv(cx, move |v, _| *v == current_option);
                    RadioButton::new(cx, is_selected)
                        .on_select(move |cx| selected.set(cx, current_option));
                }
            })
            .size(Auto)
            .horizontal_gap(Pixels(20.0));

            Label::new(cx, "Radiobuttons with labels").top(Pixels(20.0));

            VStack::new(cx, |cx| {
                for i in 0..3 {
                    let current_option = index_to_option(i);
                    HStack::new(cx, move |cx| {
                        let is_selected = selected.drv(cx, move |v, _| *v == current_option);
                        RadioButton::new(cx, is_selected)
                            .on_select(move |cx| selected.set(cx, current_option))
                            .id(format!("button_{i}"));

                        let option_label = match current_option {
                            Options::First => "First",
                            Options::Second => "Second",
                            Options::Third => "Third",
                        };
                        Label::new(cx, option_label).describing(format!("button_{i}"));
                    })
                    .size(Auto)
                    .alignment(Alignment::Center)
                    .horizontal_gap(Pixels(5.0));
                }
            })
            .vertical_gap(Pixels(10.0))
            .size(Auto);
        });
        self
    }
}

fn index_to_option(index: usize) -> Options {
    match index {
        0 => Options::First,
        1 => Options::Second,
        2 => Options::Third,
        _ => unreachable!(),
    }
}

fn main() -> Result<(), ApplicationError> {
    RadiobuttonApp::run()
}

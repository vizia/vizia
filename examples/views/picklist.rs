mod helpers;
use helpers::*;
use vizia::prelude::*;

struct PicklistApp {
    options: Signal<Vec<&'static str>>,
    selected_option: Signal<usize>,
    placeholder: Signal<&'static str>,
}

impl App for PicklistApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            options: cx.state(vec![
                "One",
                "Two",
                "Three",
                "Four",
                "Five",
                "Six really long",
                "Seven",
                "Eight",
                "Nine",
                "Ten",
                "Eleven",
                "Twelve",
            ]),
            selected_option: cx.state(usize::MAX),
            placeholder: cx.state("Select an option..."),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let options = self.options;
        let selected_option = self.selected_option;
        let placeholder = self.placeholder;

        ExamplePage::vertical(cx, move |cx| {
            PickList::new(cx, options, selected_option, true)
                .placeholder(placeholder)
                .on_select(move |cx, index| selected_option.set(cx, index))
                .width(Pixels(150.0));

            PickList::new(cx, options, selected_option, true)
                .placeholder(placeholder)
                .on_select(move |cx, index| selected_option.set(cx, index))
                .width(Pixels(100.0));
        });
        self
    }
}

fn main() -> Result<(), ApplicationError> {
    PicklistApp::run()
}

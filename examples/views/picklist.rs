mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    PicklistApp::run()
}

struct PicklistApp {
    options: Signal<Vec<&'static str>>,
    selected_option: Signal<usize>,
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
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let options = self.options;
        let selected_option = self.selected_option;

        ExamplePage::vertical(cx, move |cx| {
            PickList::new(cx, options, selected_option, true)
                .placeholder("Select an option...")
                .on_select(move |cx, index| selected_option.set(cx, index))
                .width(Pixels(150.0));

            PickList::new(cx, options, selected_option, true)
                .placeholder("Select an option...")
                .on_select(move |cx, index| selected_option.set(cx, index))
                .width(Pixels(100.0));
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Picklist"))
    }
}

mod helpers;
use helpers::*;
use vizia::prelude::*;

struct ComboboxApp {
    options: Signal<Vec<&'static str>>,
    selected_option: Signal<usize>,
}

impl App for ComboboxApp {
    fn app_name() -> &'static str {
        "Combobox"
    }

    fn new(cx: &mut Context) -> Self {
        Self {
            options: cx.state(vec![
                "One",
                "Two",
                "Three",
                "Four",
                "Five",
                "Six something long",
                "Seven",
                "Eight",
                "Nine",
                "Ten",
            ]),
            selected_option: cx.state(0usize),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let options = self.options;
        let selected_option = self.selected_option;

        ExamplePage::new(cx, |cx| {
            ComboBox::new(cx, options, selected_option).width(Pixels(140.0)).top(Pixels(100.0));
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((400, 400)))
    }
}

fn main() -> Result<(), ApplicationError> {
    ComboboxApp::run()
}

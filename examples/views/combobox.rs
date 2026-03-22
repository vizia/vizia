mod helpers;
use helpers::*;
use vizia::prelude::*;

struct AppState {
    options: Signal<Vec<&'static str>>,
    selected_option: Signal<usize>,
}

pub enum AppEvent {
    SetOption(usize),
}

impl Model for AppState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetOption(index) => {
                self.selected_option.set(*index);
            }
        });

        let _ = self.options;
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let options = Signal::new(vec![
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
        ]);
        let selected_option = Signal::new(0usize);

        AppState { options, selected_option }.build(cx);

        ExamplePage::new(cx, |cx| {
            ComboBox::new(cx, options, selected_option)
                .on_select(|cx, index| cx.emit(AppEvent::SetOption(index)))
                .width(Pixels(140.0))
                .top(Pixels(100.0));
        });
    })
    .title("Combobox")
    .inner_size((400, 400))
    .run()
}

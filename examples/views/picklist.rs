mod helpers;
use helpers::*;
use vizia::prelude::*;

struct AppState {
    _options: Signal<Vec<Signal<&'static str>>>,
    selected_option: Signal<Option<usize>>,
}

pub enum AppEvent {
    SetOption(usize),
}

impl Model for AppState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetOption(index) => {
                self.selected_option.set(Some(*index));
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let options = Signal::new(
            vec![
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
            ]
            .into_iter()
            .map(Signal::new)
            .collect::<Vec<_>>(),
        );
        let selected_option = Signal::new(None);

        AppState { _options: options, selected_option }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            PickList::new(cx, options, selected_option, true)
                .placeholder("Select an option...")
                .on_select(|cx, index| cx.emit(AppEvent::SetOption(index)))
                .width(Pixels(150.0));
        });
    })
    .title("Picklist")
    .run()
}

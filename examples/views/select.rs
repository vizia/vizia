mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Debug, Clone, Copy)]
struct AppState {
    options: Signal<Vec<&'static str>>,
    selected_option: Signal<Option<usize>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            options: Signal::new(
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
                .collect::<Vec<_>>(),
            ),
            selected_option: Signal::new(None),
        }
    }
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
        let &AppState { options, selected_option } = AppState::new().build(cx);

        ExamplePage::vertical(cx, |cx| {
            Select::new(cx, options, selected_option, true)
                .placeholder("Select an option...")
                .on_select(|cx, index| cx.emit(AppEvent::SetOption(index)))
                .width(Pixels(150.0));
        });
    })
    .title("Select")
    .run()
}

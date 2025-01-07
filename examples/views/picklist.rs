mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Lens)]
struct AppState {
    options: Vec<&'static str>,
    selected_option: usize,
}

pub enum AppEvent {
    SetOption(usize),
}

impl Model for AppState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetOption(index) => {
                self.selected_option = *index;
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppState {
            options: vec![
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
            ],
            selected_option: usize::MAX,
        }
        .build(cx);

        ExamplePage::vertical(cx, |cx| {
            PickList::new(cx, AppState::options, AppState::selected_option, true)
                .placeholder("Select an option...")
                .on_select(|cx, index| cx.emit(AppEvent::SetOption(index)))
                .width(Pixels(150.0));

            PickList::new(cx, AppState::options, AppState::selected_option, true)
                .placeholder("Select an option...")
                .on_select(|cx, index| cx.emit(AppEvent::SetOption(index)))
                .width(Pixels(100.0));
        });
    })
    .title("Picklist")
    .run()
}

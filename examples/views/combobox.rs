mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, Lens)]
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
                "Six something long",
                "Seven",
                "Eight",
                "Nine",
                "Ten",
            ],

            selected_option: 0,
        }
        .build(cx);

        ExamplePage::new(cx, |cx| {
            ComboBox::new(cx, AppState::options, AppState::selected_option)
                .on_select(|cx, index| cx.emit(AppEvent::SetOption(index)))
                .width(Pixels(140.0))
                .top(Pixels(100.0));
        });
    })
    .title("Combobox")
    .inner_size((400, 400))
    .run()
}

mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppState {
    options: Vec<&'static str>,
    options2: Vec<&'static str>,
    selected_option: usize,
    selected_option2: usize,
}

pub enum AppEvent {
    SetOption(usize),
    SetOption2(usize),
}

impl Model for AppState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetOption(index) => {
                self.selected_option = *index;
            }

            AppEvent::SetOption2(index) => {
                self.selected_option2 = *index;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppState {
            options: vec![
                "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
                "Eleven", "Twelve", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
                "Nine", "Ten", "Eleven", "Twelve",
            ],
            options2: vec![
                "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
                "Eleven", "Twelve",
            ],
            selected_option: 0,
            selected_option2: 0,
        }
        .build(cx);

        ExamplePage::new(cx, |cx| {
            PickList::new(cx, AppState::options, AppState::selected_option, true)
                .on_select(|cx, index| cx.emit(AppEvent::SetOption(index)))
                .width(Pixels(140.0));
            PickList::new(cx, AppState::options2, AppState::selected_option2, true)
                .on_select(|cx, index| cx.emit(AppEvent::SetOption2(index)))
                .width(Pixels(140.0));
        });
    })
    .title("Picklist")
    .run();
}

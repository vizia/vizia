mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppState {
    options1: Vec<&'static str>,
    options2: Vec<&'static str>,
    options3: Vec<&'static str>,
    selected_option1: usize,
    selected_option2: usize,
    selected_option3: usize,
}

pub enum AppEvent {
    SetOption1(usize),
    SetOption2(usize),
    SetOption3(usize),
}

impl Model for AppState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetOption1(index) => {
                self.selected_option1 = *index;
            }

            AppEvent::SetOption2(index) => {
                self.selected_option2 = *index;
            }

            AppEvent::SetOption3(index) => {
                self.selected_option3 = *index;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppState {
            options1: vec![
                "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
            ],
            options2: vec![
                "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
            ],
            options3: vec![
                "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
            ],
            selected_option1: 0,
            selected_option2: 0,
            selected_option3: 0,
        }
        .build(cx);

        ExamplePage::new(cx, |cx| {
            // PickList::new(cx, AppState::options, AppState::selected_option, true)
            //     .on_select(|cx, index| cx.emit(AppEvent::SetOption(index)))
            //     .width(Pixels(140.0));
            ComboBox::new(cx, AppState::options1, AppState::selected_option1)
                .on_select(|cx, index| cx.emit(AppEvent::SetOption1(index)))
                .width(Pixels(140.0))
                .top(Pixels(100.0));
            ComboBox::new(cx, AppState::options1, AppState::selected_option1)
                .on_select(|cx, index| cx.emit(AppEvent::SetOption1(index)))
                .width(Pixels(140.0))
                .top(Pixels(400.0));
            ComboBox::new(cx, AppState::options3, AppState::selected_option3)
                .on_select(|cx, index| cx.emit(AppEvent::SetOption3(index)))
                .width(Pixels(140.0));
        });
    })
    .title("Picklist")
    .run();
}

mod helpers;
use chrono::{NaiveDate, Utc};
use helpers::*;
use vizia::prelude::*;

struct AppState {
    date: Signal<NaiveDate>,
}

pub enum AppEvent {
    SetDate(NaiveDate),
}

impl Model for AppState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetDate(date) => {
                self.date.set(*date);
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let date = Signal::new(Utc::now().date_naive());

        AppState { date }.build(cx);

        ExamplePage::new(cx, |cx| {
            Calendar::new(cx, date).on_select(|cx, date| cx.emit(AppEvent::SetDate(date)));
        });
    })
    .title("Calendar")
    .run()
}

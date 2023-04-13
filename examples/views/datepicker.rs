mod helpers;
use chrono::{NaiveDate, Utc};
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppState {
    date: NaiveDate,
}

pub enum AppEvent {
    SetDate(NaiveDate),
}

impl Model for AppState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetDate(date) => {
                println!("Date changed to: {}", date);
                self.date = *date;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppState { date: Utc::now().date_naive() }.build(cx);

        ExamplePage::new(cx, |cx| {
            Datepicker::new(cx, AppState::date)
                .on_select(|cx, date| cx.emit(AppEvent::SetDate(date)));
        });
    })
    .title("Datepicker")
    .run();
}

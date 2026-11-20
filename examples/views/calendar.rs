mod helpers;
use helpers::*;
use jiff::Timestamp;
use jiff::civil::Date;
use jiff::tz::TimeZone;
use vizia::prelude::*;

struct AppState {
    date: Signal<Date>,
}

pub enum AppEvent {
    SetDate(Date),
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
        load_translations(cx);

        Window::new(cx, |cx| {
            let date = Signal::new(Timestamp::now().to_zoned_datetime(TimeZone::UTC).date());

            AppState { date }.build(cx);

            ExamplePage::new(cx, |cx| {
                Calendar::new(cx, date).on_select(|cx, date| cx.emit(AppEvent::SetDate(date)));
            });
        })
        .title(Localized::new("view-title-calendar"));
    })
    .run()
}

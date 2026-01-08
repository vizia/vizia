mod helpers;
use chrono::Utc;
use helpers::*;
use vizia::prelude::*;

struct DatepickerApp {
    date: Signal<chrono::NaiveDate>,
}

impl App for DatepickerApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            date: cx.state(Utc::now().date_naive()),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let date = self.date;
        ExamplePage::new(cx, |cx| {
            Datepicker::new(cx, date).on_select(move |cx, selected| date.set(cx, selected));
        });
        self
    }
}

fn main() -> Result<(), ApplicationError> {
    DatepickerApp::run()
}

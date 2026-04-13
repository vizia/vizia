use crate::components::DemoRegion;
use chrono::{NaiveDate, Utc};
use vizia::prelude::*;

struct CalendarState {
    date: Signal<NaiveDate>,
}

pub enum CalendarEvent {
    SetDate(NaiveDate),
}

impl Model for CalendarState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            CalendarEvent::SetDate(date) => {
                self.date.set(*date);
            }
        });
    }
}

pub fn calendar(cx: &mut Context) {
    let date = Signal::new(Utc::now().date_naive());

    VStack::new(cx, |cx| {
        CalendarState { date }.build(cx);

        Markdown::new(cx, "# Calendar");

        Divider::new(cx);

        Markdown::new(cx, "### Basic calendar");

        DemoRegion::new(cx, "Basic Calendar", move |cx| {
            Calendar::new(cx, date).on_select(|cx, date| cx.emit(CalendarEvent::SetDate(date)));
        });
    })
    .class("panel");
}

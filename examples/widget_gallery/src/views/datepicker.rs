use crate::components::DemoRegion;
use chrono::{NaiveDate, Utc};
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct DatepickerState {
    date: NaiveDate,
}

pub enum DatepickerEvent {
    SetDate(NaiveDate),
}

impl Model for DatepickerState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            DatepickerEvent::SetDate(date) => {
                self.date = *date;
            }
        });
    }
}

pub fn datepicker(cx: &mut Context) {
    VStack::new(cx, |cx| {
        DatepickerState { date: Utc::now().date_naive() }.build(cx);

        Label::new(cx, "Datepicker").class("title");
        Label::new(cx, "todo...").class("paragraph");

        Label::new(cx, "Basic datepicker").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Datepicker::new(cx, DatepickerState::date)
                    .on_select(|cx, date| cx.emit(DatepickerEvent::SetDate(date)));
            },
            |cx| {
                Label::new(cx, r#"Todo"#).class("code");
            },
        );
    })
    .class("panel");
}

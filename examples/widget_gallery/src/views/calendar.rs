use crate::components::DemoRegion;
use jiff::Timestamp;
use jiff::civil::Date;
use jiff::tz::TimeZone;
use vizia::prelude::*;

struct CalendarState {
    date: Signal<Date>,
}

pub enum CalendarEvent {
    SetDate(Date),
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
    let date = Signal::new(Timestamp::now().to_zoned_datetime(TimeZone::UTC).date());

    VStack::new(cx, |cx| {
        CalendarState { date }.build(cx);

        Label::new(cx, Localized::new("calendar")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Basic Calendar", move |cx| {
            Calendar::new(cx, date).on_select(|cx, date| cx.emit(CalendarEvent::SetDate(date)));
        });
    })
    .class("panel");
}

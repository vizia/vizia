use chrono::{Date, Datelike, NaiveDate, TimeZone, Utc};

use crate::prelude::*;

impl<T: TimeZone + 'static> Data for Date<T> {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

pub struct Datepicker<L: Lens<Target = Date<T>>, T: TimeZone + 'static> {
    date: L,
}

pub enum DatepickerEvent {
    N,
}

impl<L: Lens<Target = Date<T>>, T: TimeZone + 'static> Datepicker<L, T> {
    fn is_leap_year(year: i32) -> bool {
        NaiveDate::from_ymd_opt(year, 2, 29).is_some()
    }

    fn last_day_of_month(year: i32, month: u32) -> i64 {
        if month == 12 {
            NaiveDate::from_ymd(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd(year, month + 1, 1)
        }
        .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
        .num_days()
    }

    pub fn new(cx: &mut Context, date: L) -> Handle<Self>
where {
        let today = Utc::today();
        let weekday = today.weekday();
        let day = today.day();
        let month = today.month();
        let year = today.year();
        let days_this_month = Self::last_day_of_month(year, month);
        println!("{}: {} {} {} {} -- {}", today, weekday, day, month, year, days_this_month);

        Self { date: date.clone() }
            .build(cx, |cx| {
                Binding::new(cx, date, |cx, lens| {
                    let date = lens.get(cx);
                    Label::new(cx, &format!("{}", date.day()));
                    Label::new(cx, &format!("{}", date.month()));
                    Label::new(cx, &format!("{}", date.year()));
                    Label::new(
                        cx,
                        &format!("{}", Self::last_day_of_month(date.year(), date.month())),
                    );
                });
            })
            .background_color(Color::rgb(24, 24, 24))
            .keyboard_navigatable(true)
    }
}

impl<L: Lens<Target = Date<T>>, T: TimeZone + 'static> View for Datepicker<L, T> {
    fn element(&self) -> Option<&'static str> {
        Some("datepicker")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            DatepickerEvent::N => {}
        })
    }
}

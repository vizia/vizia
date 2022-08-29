use chrono::{Datelike, NaiveDate, Utc, Weekday};

use crate::prelude::*;

#[derive(Lens, Data, Clone)]
pub struct Date {
    day: u32,
    month: u32,
    year: i32,
}

#[derive(Lens)]
pub struct Datepicker {
    view_date: Date,

    selected_date: Date,

    month_str: String,
}

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const DAYS_HEADER: [&str; 7] = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];

pub enum DatepickerEvent {
    IncrementMonth,
    DecrementMonth,

    IncrementYear,
    DecrementYear,

    SelectDate(Date),
}

impl Datepicker {
    fn is_leap_year(year: i32) -> bool {
        NaiveDate::from_ymd_opt(year, 2, 29).is_some()
    }

    fn first_day_of_month(year: i32, month: u32) -> Weekday {
        NaiveDate::from_ymd(year, month, 1).weekday()
    }

    fn last_day_of_month(year: i32, month: u32) -> u32 {
        if month == 12 {
            NaiveDate::from_ymd(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd(year, month + 1, 1)
        }
        .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
        .num_days() as u32
    }

    fn render_disabled_day(cx: &mut Context, day: u32) {
        Element::new(cx).text(&day.to_string()).class("datepicker-calendar-day-disabled");
    }

    fn render_day(cx: &mut Context, view_date: &Date, selected_date: &Date, current_day: u32) {
        let view_date = view_date.clone();
        Element::new(cx)
            .text(&current_day.to_string())
            .class("datepicker-calendar-day")
            .checked(
                selected_date.day == current_day
                    && selected_date.month == view_date.month
                    && selected_date.year == view_date.year,
            )
            .cursor(CursorIcon::Hand)
            .on_press(move |ex| {
                ex.emit(DatepickerEvent::SelectDate(Date {
                    day: current_day,
                    month: view_date.month,
                    year: view_date.year,
                }))
            });
    }

    fn previous_month_days(year: i32, month: u32) -> u32 {
        if month == 1 {
            Self::last_day_of_month(year - 1, 12)
        } else {
            Self::last_day_of_month(year, month - 1)
        }
    }

    pub fn new(cx: &mut Context) -> Handle<Self>
where {
        let today = Utc::today();
        let day = today.day();
        let month = today.month();
        let year = today.year();

        Self {
            month_str: MONTHS[month as usize - 1].to_string(),
            view_date: Date { day, month, year },
            selected_date: Date { day, month, year },
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                Spinbox::new(cx, Datepicker::month_str, SpinboxKind::Horizontal)
                    .left(Units::Stretch(1.0))
                    .on_increment(|ex| ex.emit(DatepickerEvent::IncrementMonth))
                    .on_decrement(|ex| ex.emit(DatepickerEvent::DecrementMonth));
                Spinbox::new(cx, Datepicker::view_date.then(Date::year), SpinboxKind::Horizontal)
                    .right(Units::Stretch(1.0))
                    .on_increment(|ex| ex.emit(DatepickerEvent::IncrementYear))
                    .on_decrement(|ex| ex.emit(DatepickerEvent::DecrementYear));
            })
            .class("datepicker-header");

            Element::new(cx).class("datepicker-divisor");

            Binding::new(cx, Datepicker::view_date, |cx, view_date| {
                Binding::new(cx, Datepicker::selected_date, move |cx, selected_date| {
                    let view_date = view_date.get(cx);
                    let selected_date = selected_date.get(cx);
                    VStack::new(cx, |cx| {
                        HStack::new(cx, |cx| {
                            for h in DAYS_HEADER {
                                Element::new(cx).text(h).class("datepicker-calendar-header");
                            }
                        })
                        .class("datepicker-calendar-headers");

                        let days_this_month =
                            Self::last_day_of_month(view_date.year, view_date.month);

                        let mut current_rendered_day = 0;

                        let first_day_of_month =
                            Self::first_day_of_month(view_date.year, view_date.month);

                        // First week of month
                        HStack::new(cx, |cx| {
                            let previous_month_days =
                                Self::previous_month_days(view_date.year, view_date.month);
                            let filling_days = first_day_of_month.num_days_from_monday();
                            for i in 0..filling_days {
                                Self::render_disabled_day(
                                    cx,
                                    previous_month_days - (filling_days - i - 1),
                                );
                            }

                            for i in 0..7 - filling_days {
                                current_rendered_day += 1;
                                Self::render_day(
                                    cx,
                                    &view_date,
                                    &selected_date,
                                    current_rendered_day,
                                );
                            }
                        })
                        .class("datepicker-calendar-row");

                        // Filling weeks
                        while current_rendered_day < days_this_month - 7 {
                            HStack::new(cx, |cx| {
                                for i in 0..7 {
                                    current_rendered_day += 1;
                                    Self::render_day(
                                        cx,
                                        &view_date,
                                        &selected_date,
                                        current_rendered_day,
                                    );
                                }
                            })
                            .class("datepicker-calendar-row");
                        }

                        // Last week of month
                        HStack::new(cx, |cx| {
                            let mut new_days = 1;
                            for i in 0..7 {
                                current_rendered_day += 1;
                                if current_rendered_day <= days_this_month {
                                    Self::render_day(
                                        cx,
                                        &view_date,
                                        &selected_date,
                                        current_rendered_day,
                                    );
                                } else {
                                    Self::render_disabled_day(cx, new_days);
                                    new_days += 1;
                                }
                            }
                        })
                        .class("datepicker-calendar-row");
                    })
                    .class("datepicker-calendar");

                    Element::new(cx).class("datepicker-divisor");

                    let year = selected_date.year;
                    let month = selected_date.month;
                    let day = selected_date.day;

                    Label::new(
                        cx,
                        &format!(
                            "{} {} of {} {}",
                            NaiveDate::from_ymd(year, month, day).weekday().to_string(),
                            day,
                            month,
                            year
                        ),
                    )
                    .class("datepicker-selected-date");

                    HStack::new(cx, |cx| {
                        Button::new(cx, |ex| println!("Cancel"), |cx| Label::new(cx, "Cancel"))
                            .class("datepicker-cancel");
                        Button::new(cx, |ex| println!("Apply"), |cx| Label::new(cx, "Apply"))
                            .class("datepicker-apply");
                    })
                    .class("datepicker-actions-container");
                });
            });
        })
        .layout_type(LayoutType::Column)
        .keyboard_navigatable(true)
    }
}

impl View for Datepicker {
    fn element(&self) -> Option<&'static str> {
        Some("datepicker")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            DatepickerEvent::IncrementMonth => {
                if self.view_date.month == 12 {
                    self.view_date.month = 1;
                    self.view_date.year += 1;
                } else {
                    self.view_date.month += 1;
                }

                println!("{}", self.view_date.month);

                self.month_str = MONTHS[self.view_date.month as usize - 1].to_string();
            }

            DatepickerEvent::DecrementMonth => {
                if self.view_date.month == 1 {
                    self.view_date.month = 12;
                    self.view_date.year -= 1;
                } else {
                    self.view_date.month -= 1;
                }
                println!("{}", self.view_date.month);

                self.month_str = MONTHS[self.view_date.month as usize - 1].to_string();
            }

            DatepickerEvent::IncrementYear => {
                self.view_date.year += 1;
            }

            DatepickerEvent::DecrementYear => {
                self.view_date.year -= 1;
            }

            DatepickerEvent::SelectDate(date) => {
                self.selected_date = date.clone();
            }
        })
    }
}

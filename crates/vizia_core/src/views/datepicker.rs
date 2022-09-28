use chrono::{Datelike, NaiveDate, Utc, Weekday};

use crate::prelude::*;

#[derive(Lens, Data, Clone, Copy, Debug)]
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

    on_cancel: Option<Box<dyn Fn(&mut EventContext)>>,
    on_apply: Option<Box<dyn Fn(&mut EventContext, Date)>>,
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
    Apply,
    Cancel,

    IncrementMonth,
    DecrementMonth,

    IncrementYear,
    DecrementYear,

    SelectDate(Date),
}

impl Datepicker {
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

    fn view_month_info(view_date: &Date, month_offset: i32) -> (Weekday, u32) {
        let month = view_date.month;
        let mut year = view_date.year;

        println!("{} {}, {}", year, month, month_offset);

        let mut month = month as i32 + month_offset;

        if month < 1 {
            year -= 1;
            month += 12;
        } else if month > 12 {
            year += 1;
            month -= 12;
        }

        let month = month as u32;

        println!("S{} {}, {}", year, month, month_offset);

        (Self::first_day_of_month(year, month), Self::last_day_of_month(year, month))
    }

    fn get_day_number(y: u32, x: u32, view_date: &Date) -> (u32, bool) {
        let (_, days_prev_month) = Self::view_month_info(&view_date, -1);
        let (first_day_this_month, days_this_month) = Self::view_month_info(&view_date, 0);

        let mut fdtm_i = first_day_this_month as usize as u32;
        if fdtm_i == 0 {
            fdtm_i = 7;
        }

        if y == 0 {
            if x < fdtm_i {
                (days_prev_month - (fdtm_i - x - 1), true)
            } else {
                (x - fdtm_i + 1, false)
            }
        } else {
            let day_number = y * 7 + x - fdtm_i + 1;
            if day_number > days_this_month {
                (day_number - days_this_month, true)
            } else {
                (day_number, false)
            }
        }
    }

    pub fn new<C, A>(cx: &mut Context, on_cancel: C, on_apply: A) -> Handle<Self>
    where
        C: 'static + Fn(&mut EventContext),
        A: 'static + Fn(&mut EventContext, Date),
    {
        let today = Utc::today();
        let day = today.day();
        let month = today.month();
        let year = today.year();

        Self {
            month_str: MONTHS[month as usize - 1].to_string(),
            view_date: Date { day, month, year },
            selected_date: Date { day, month, year },
            on_cancel: Some(Box::new(on_cancel)),
            on_apply: Some(Box::new(on_apply)),
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                Spinbox::new(cx, Datepicker::month_str, SpinboxKind::Horizontal)
                    .width(Units::Pixels(144.0))
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

            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    for h in DAYS_HEADER {
                        Element::new(cx).text(h).class("datepicker-calendar-header");
                    }
                })
                .class("datepicker-calendar-headers");

                for y in 0..6 {
                    HStack::new(cx, |cx| {
                        for x in 0..7 {
                            Label::new(cx, "").bind(
                                Datepicker::view_date,
                                move |handle, view_date| {
                                    handle.bind(
                                        Datepicker::selected_date,
                                        move |handle, selected_date| {
                                            let view_date = view_date.get(handle.cx);
                                            let selected_date = selected_date.get(handle.cx);

                                            let (day_number, disabled) =
                                                Self::get_day_number(y, x, &view_date);

                                            if disabled {
                                                handle
                                                    .text(&day_number.to_string())
                                                    .class("datepicker-calendar-day-disabled");
                                            } else {
                                                handle
                                                    .text(&day_number.to_string())
                                                    .class("datepicker-calendar-day")
                                                    .cursor(CursorIcon::Hand)
                                                    .on_press(move |ex| {
                                                        ex.emit(DatepickerEvent::SelectDate(Date {
                                                            day: day_number,
                                                            month: view_date.month,
                                                            year: view_date.year,
                                                        }))
                                                    })
                                                    .checked(
                                                        selected_date.day == day_number
                                                            && selected_date.month
                                                                == view_date.month
                                                            && selected_date.year == view_date.year,
                                                    );
                                            }
                                        },
                                    );
                                },
                            );
                        }
                    })
                    .class("datepicker-calendar-row");
                }
            })
            .class("datepicker-calendar");

            Element::new(cx).class("datepicker-divisor");

            Binding::new(cx, Datepicker::selected_date, |cx, selected_date| {
                let selected_date = selected_date.get(cx);

                let year = selected_date.year;
                let month = selected_date.month;
                let day = selected_date.day;

                Label::new(
                    cx,
                    &format!(
                        "{} {} of {} {}",
                        NaiveDate::from_ymd(year, month, day).weekday().to_string(),
                        day,
                        MONTHS[month as usize - 1][0..3].to_string(),
                        year
                    ),
                )
                .class("datepicker-selected-date");
            });

            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Cancel"))
                    .class("datepicker-cancel")
                    .on_press(|ex| ex.emit(DatepickerEvent::Cancel));
                Button::new(cx, |cx| Label::new(cx, "Apply"))
                    .on_press(|ex| ex.emit(DatepickerEvent::Apply))
                    .class("datepicker-apply");
            })
            .class("datepicker-actions-container");
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
            DatepickerEvent::Apply => {
                if let Some(callback) = &self.on_apply {
                    (callback)(cx, self.selected_date);
                }
            }

            DatepickerEvent::Cancel => {
                if let Some(callback) = &self.on_cancel {
                    (callback)(cx);
                }
            }

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

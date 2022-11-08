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
    // The current date.
    view_date: Date,

    // The selected date.
    selected_date: Date,

    // The current month.
    month_str: String,

    #[lens(ignore)]
    on_select: Option<Box<dyn Fn(&mut EventContext, Date)>>,
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

    // Given a date and a month offset, returns the first day of the month and the number of days in the month
    fn view_month_info(view_date: &Date, month_offset: i32) -> (Weekday, u32) {
        let month = view_date.month;
        let mut year = view_date.year;

        let mut month = month as i32 + month_offset;

        if month < 1 {
            year -= 1;
            month += 12;
        } else if month > 12 {
            year += 1;
            month -= 12;
        }

        let month = month as u32;

        (Self::first_day_of_month(year, month), Self::last_day_of_month(year, month))
    }

    fn get_day_number(y: u32, x: u32, view_date: &Date) -> (u32, bool) {
        println!("{} {}", x, y);
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

    pub fn new(cx: &mut Context) -> Handle<Self> {
        let today = Utc::today();
        let day = today.day();
        let month = today.month();
        let year = today.year();

        Self {
            month_str: MONTHS[month as usize - 1].to_string(),
            view_date: Date { day, month, year },
            selected_date: Date { day, month, year },
            on_select: None,
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                Spinbox::new(cx, Datepicker::month_str, SpinboxKind::Horizontal)
                    .width(Stretch(3.0))
                    .on_increment(|ex| ex.emit(DatepickerEvent::IncrementMonth))
                    .on_decrement(|ex| ex.emit(DatepickerEvent::DecrementMonth));
                Spinbox::new(cx, Datepicker::view_date.then(Date::year), SpinboxKind::Horizontal)
                    .width(Stretch(2.0))
                    .on_increment(|ex| ex.emit(DatepickerEvent::IncrementYear))
                    .on_decrement(|ex| ex.emit(DatepickerEvent::DecrementYear));
            })
            .class("datepicker-header");

            Element::new(cx).class("datepicker-divisor");

            VStack::new(cx, |cx| {
                // Days of the week
                HStack::new(cx, |cx| {
                    for h in DAYS_HEADER {
                        Label::new(cx, h).class("datepicker-calendar-header");
                    }
                })
                .class("datepicker-calendar-headers");

                // Numbered days in a grid
                HStack::new(cx, |cx| {
                    for y in 0..6 {
                        for x in 0..7 {
                            Label::new(cx, "")
                                .bind(Datepicker::view_date, move |handle, view_date| {
                                    let view_date = view_date.get(handle.cx);

                                    let (day_number, disabled) =
                                        Self::get_day_number(y, x, &view_date);

                                    handle.bind(
                                        Datepicker::selected_date,
                                        move |handle, selected_date| {
                                            let selected_date = selected_date.get(handle.cx);

                                            handle
                                                .text(&day_number.to_string())
                                                .class("datepicker-calendar-day")
                                                .toggle_class(
                                                    "datepicker-calendar-day-disabled",
                                                    disabled,
                                                )
                                                .on_press(move |ex| {
                                                    if !disabled {
                                                        ex.emit(DatepickerEvent::SelectDate(Date {
                                                            day: day_number,
                                                            month: view_date.month,
                                                            year: view_date.year,
                                                        }))
                                                    }
                                                })
                                                .checked(
                                                    !disabled
                                                        && selected_date.day == day_number
                                                        && selected_date.month == view_date.month
                                                        && selected_date.year == view_date.year,
                                                );
                                        },
                                    );
                                })
                                .row_index(y as usize)
                                .col_index(x as usize);
                        }
                    }
                })
                // This shouldn't be needed but apparently grid size isn't propagated up the tree during layout
                .width(Pixels(32.0 * 7.0))
                .height(Pixels(32.0 * 6.0))
                .layout_type(LayoutType::Grid)
                .grid_rows(vec![Pixels(32.0); 6])
                .grid_cols(vec![Pixels(32.0); 7]);
            })
            .class("datepicker-calendar");

            Element::new(cx).class("datepicker-divisor");

            // The currently selected date as a string
            Label::new(
                cx,
                Datepicker::selected_date.map(|selected_date| {
                    let year = selected_date.year;
                    let month = selected_date.month;
                    let day = selected_date.day;

                    format!(
                        "{} {} of {} {}",
                        NaiveDate::from_ymd(year, month, day).weekday().to_string(),
                        day,
                        MONTHS[month as usize - 1][0..3].to_string(),
                        year
                    )
                }),
            )
            .class("datepicker-selected-date");
        })
        .navigable(true)
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

                self.month_str = MONTHS[self.view_date.month as usize - 1].to_string();
            }

            DatepickerEvent::DecrementMonth => {
                if self.view_date.month == 1 {
                    self.view_date.month = 12;
                    self.view_date.year -= 1;
                } else {
                    self.view_date.month -= 1;
                }

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

                if let Some(callback) = &self.on_select {
                    (callback)(cx, self.selected_date);
                }
            }
        })
    }
}

impl<'a> Handle<'a, Datepicker> {
    pub fn on_select<F: 'static + Fn(&mut EventContext, Date)>(self, callback: F) -> Self {
        self.modify(|datepicker: &mut Datepicker| datepicker.on_select = Some(Box::new(callback)))
    }
}

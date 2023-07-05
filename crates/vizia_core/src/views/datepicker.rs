use chrono::{Datelike, NaiveDate, Weekday};

use crate::prelude::*;

use super::spinbox::SpinboxIcons;

/// A control used to select a date.
#[derive(Lens)]
pub struct Datepicker {
    view_date: NaiveDate,
    months: Vec<Localized>,
    selected_month: usize,

    #[lens(ignore)]
    on_select: Option<Box<dyn Fn(&mut EventContext, NaiveDate)>>,
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

const DAYS_HEADER: [&str; 7] =
    ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

pub enum DatepickerEvent {
    IncrementMonth,
    DecrementMonth,
    SelectMonth(usize),

    IncrementYear,
    DecrementYear,
    SelectYear(String),

    SelectDate(NaiveDate),
}

impl Datepicker {
    fn first_day_of_month(year: i32, month: u32) -> Option<Weekday> {
        NaiveDate::from_ymd_opt(year, month, 1).map(|date| date.weekday())
    }

    fn last_day_of_month(year: i32, month: u32) -> Option<u32> {
        if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .map(|date| {
            date.signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap()).num_days()
                as u32
        })
    }

    // Given a date and a month offset, returns the first day of the month and the number of days in the month
    fn view_month_info(view_date: &NaiveDate, month_offset: i32) -> (Weekday, u32) {
        let month = view_date.month();
        let mut year = view_date.year();

        let mut month = month as i32 + month_offset;

        if month < 1 {
            year -= 1;
            month += 12;
        } else if month > 12 {
            year += 1;
            month -= 12;
        }

        let month = month as u32;

        (
            Self::first_day_of_month(year, month).unwrap(),
            Self::last_day_of_month(year, month).unwrap(),
        )
    }

    fn get_day_number(y: u32, x: u32, view_date: &NaiveDate) -> (u32, bool) {
        let (_, days_prev_month) = Self::view_month_info(view_date, -1);
        let (first_day_this_month, days_this_month) = Self::view_month_info(view_date, 0);

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

    pub fn new<L, D>(cx: &mut Context, lens: L) -> Handle<Self>
    where
        L: Lens<Target = D>,
        D: Datelike + Data,
    {
        let view_date = lens.get(cx);

        Self {
            months: MONTHS.iter().map(|m| Localized::new(m)).collect::<Vec<_>>(),
            selected_month: view_date.month() as usize - 1,
            view_date: NaiveDate::from_ymd_opt(view_date.year(), view_date.month(), 1).unwrap(),
            on_select: None,
        }
        .build(cx, move |cx| {
            HStack::new(cx, |cx| {
                Spinbox::custom(
                    cx,
                    |cx| {
                        PickList::new(cx, Datepicker::months, Datepicker::selected_month, false)
                            .on_select(|ex, index| ex.emit(DatepickerEvent::SelectMonth(index)))
                    },
                    SpinboxKind::Horizontal,
                    SpinboxIcons::Chevrons,
                )
                .width(Pixels(131.0))
                .on_increment(|ex| ex.emit(DatepickerEvent::IncrementMonth))
                .on_decrement(|ex| ex.emit(DatepickerEvent::DecrementMonth));
                Spinbox::custom(
                    cx,
                    |cx| {
                        Textbox::new(cx, Datepicker::view_date.map(|date| date.year()))
                            .on_edit(|ex, v| ex.emit(DatepickerEvent::SelectYear(v)))
                            .width(Stretch(1.0))
                    },
                    SpinboxKind::Horizontal,
                    SpinboxIcons::PlusMinus,
                )
                .width(Stretch(1.0))
                .on_increment(|ex| ex.emit(DatepickerEvent::IncrementYear))
                .on_decrement(|ex| ex.emit(DatepickerEvent::DecrementYear));
            })
            .class("datepicker-header");

            Element::new(cx).class("datepicker-divisor");

            VStack::new(cx, move |cx| {
                // Days of the week
                HStack::new(cx, |cx| {
                    for h in DAYS_HEADER {
                        Label::new(cx, Localized::new(h).map(|day| day[0..2].to_string()))
                            .class("datepicker-calendar-header");
                    }
                })
                .class("datepicker-calendar-headers");

                // Numbered days in a grid
                VStack::new(cx, move |cx| {
                    for y in 0..6 {
                        HStack::new(cx, |cx| {
                            for x in 0..7 {
                                Label::new(cx, "").bind(
                                    Datepicker::view_date,
                                    move |handle, view_date| {
                                        let view_date = view_date.get(handle.cx);

                                        let (day_number, disabled) =
                                            Self::get_day_number(y, x, &view_date);

                                        handle.bind(lens, move |handle, selected_date| {
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
                                                        ex.emit(DatepickerEvent::SelectDate(
                                                            NaiveDate::from_ymd_opt(
                                                                view_date.year(),
                                                                view_date.month(),
                                                                day_number,
                                                            )
                                                            .unwrap(),
                                                        ))
                                                    }
                                                })
                                                .checked(
                                                    !disabled
                                                        && selected_date.day() == day_number
                                                        && selected_date.month()
                                                            == view_date.month()
                                                        && selected_date.year() == view_date.year(),
                                                );
                                        });
                                    },
                                );
                            }
                        });
                    }
                })
                // This shouldn't be needed but apparently grid size isn't propagated up the tree during layout
                .width(Pixels(32.0 * 7.0))
                .height(Pixels(32.0 * 6.0));
            })
            .class("datepicker-calendar");
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
                if self.view_date.month() == 12 {
                    self.view_date =
                        NaiveDate::from_ymd_opt(self.view_date.year() + 1, 1, self.view_date.day())
                            .unwrap();
                } else {
                    self.view_date = NaiveDate::from_ymd_opt(
                        self.view_date.year(),
                        self.view_date.month() + 1,
                        self.view_date.day(),
                    )
                    .unwrap();
                }
                self.selected_month = self.view_date.month() as usize;
            }

            DatepickerEvent::DecrementMonth => {
                if self.view_date.month() == 1 {
                    self.view_date = NaiveDate::from_ymd_opt(
                        self.view_date.year() - 1,
                        12,
                        self.view_date.day(),
                    )
                    .unwrap();
                } else {
                    self.view_date = NaiveDate::from_ymd_opt(
                        self.view_date.year(),
                        self.view_date.month() - 1,
                        self.view_date.day(),
                    )
                    .unwrap();
                }
                self.selected_month = self.view_date.month() as usize;
            }

            DatepickerEvent::SelectMonth(month) => {
                self.view_date = NaiveDate::from_ymd_opt(
                    self.view_date.year(),
                    *month as u32 + 1,
                    self.view_date.day(),
                )
                .unwrap();
                self.selected_month = *month;
            }

            DatepickerEvent::IncrementYear => {
                self.view_date += chrono::Duration::days(365);
            }

            DatepickerEvent::DecrementYear => {
                self.view_date -= chrono::Duration::days(365);
            }

            DatepickerEvent::SelectYear(year) => {
                if let Ok(year) = year.parse::<i32>() {
                    self.view_date =
                        NaiveDate::from_ymd_opt(year, self.view_date.month(), self.view_date.day())
                            .unwrap();
                }
            }

            DatepickerEvent::SelectDate(date) => {
                if let Some(callback) = &self.on_select {
                    (callback)(cx, *date);
                }
            }
        })
    }
}

impl<'a> Handle<'a, Datepicker> {
    pub fn on_select<F: 'static + Fn(&mut EventContext, NaiveDate)>(self, callback: F) -> Self {
        self.modify(|datepicker: &mut Datepicker| datepicker.on_select = Some(Box::new(callback)))
    }
}

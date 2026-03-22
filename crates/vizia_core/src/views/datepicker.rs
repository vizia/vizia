use chrono::{Datelike, NaiveDate, Weekday};

use crate::prelude::*;

/// A control used to select a date.
pub struct Datepicker {
    view_date: Signal<NaiveDate>,
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

pub(crate) enum DatepickerEvent {
    IncrementMonth,
    DecrementMonth,
    SelectMonth(usize),

    IncrementYear,
    DecrementYear,
    SelectYear(String),

    SelectDate(NaiveDate),
}

impl Datepicker {
    fn set_view_date(&mut self, year: i32, month: u32, day: u32) {
        self.view_date.set(NaiveDate::from_ymd_opt(year, month, day).unwrap());
    }

    fn shift_month(&mut self, delta: i32) {
        let view_date = self.view_date.get();
        let mut year = view_date.year();
        let mut month = view_date.month() as i32 + delta;

        if month < 1 {
            year -= 1;
            month += 12;
        } else if month > 12 {
            year += 1;
            month -= 12;
        }

        self.set_view_date(year, month as u32, view_date.day());
    }

    fn shift_year(&mut self, delta: i32) {
        let view_date = self.view_date.get();
        self.set_view_date(view_date.year() + delta, view_date.month(), view_date.day());
    }

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

    /// Create a new [Datepicker] view.
    pub fn new<R, D>(cx: &mut Context, date: R) -> Handle<Self>
    where
        R: Res<D> + Clone + 'static,
        D: Datelike + Clone + 'static,
    {
        let selected_date = date.get_value(cx);
        let initial_view_date =
            NaiveDate::from_ymd_opt(selected_date.year(), selected_date.month(), 1).unwrap();
        let view_date = Signal::new(initial_view_date);
        let month_options =
            Signal::new(MONTHS.iter().map(|m| Signal::new(Localized::new(m))).collect::<Vec<_>>());
        let selected_month = view_date.map(|date| date.month() as usize - 1);

        Self { view_date, on_select: None }.build(cx, move |cx| {
            HStack::new(cx, |cx| {
                Spinbox::custom(cx, move |cx| {
                    PickList::new(cx, month_options, selected_month, false)
                        .on_select(|ex, index| ex.emit(DatepickerEvent::SelectMonth(index)))
                        .width(Stretch(1.0))
                })
                .width(Pixels(140.0))
                .on_increment(|ex| ex.emit(DatepickerEvent::IncrementMonth))
                .on_decrement(|ex| ex.emit(DatepickerEvent::DecrementMonth));
                Spinbox::custom(cx, |cx| {
                    let view_date =
                        cx.data::<Datepicker>().map(|datepicker| datepicker.view_date).unwrap();
                    let year = view_date.map(|date| date.year());
                    Textbox::new(cx, year)
                        .width(Stretch(1.0))
                        .padding(Pixels(1.0))
                        .on_edit(|ex, v| ex.emit(DatepickerEvent::SelectYear(v)))
                })
                .width(Pixels(100.0))
                .icons(SpinboxIcons::PlusMinus)
                .on_increment(|ex| ex.emit(DatepickerEvent::IncrementYear))
                .on_decrement(|ex| ex.emit(DatepickerEvent::DecrementYear));
            })
            .class("datepicker-header");

            Divider::new(cx);

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
                                let selected_date = date.clone();
                                let view_date = cx
                                    .data::<Datepicker>()
                                    .map(|datepicker| datepicker.view_date)
                                    .unwrap();
                                Label::new(cx, "").bind(view_date, move |handle, view_date| {
                                    let selected_date = selected_date.clone();

                                    let (day_number, disabled) =
                                        Self::get_day_number(y, x, &view_date);

                                    handle.bind(selected_date, move |handle, selected_date| {
                                        let selected_date = selected_date;

                                        handle
                                            .text(&day_number.to_string())
                                            .class("datepicker-calendar-day")
                                            .navigable(!disabled)
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
                                                    && selected_date.month() == view_date.month()
                                                    && selected_date.year() == view_date.year(),
                                            );
                                    });
                                });
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
    }
}

impl View for Datepicker {
    fn element(&self) -> Option<&'static str> {
        Some("datepicker")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            DatepickerEvent::IncrementMonth => {
                self.shift_month(1);
            }

            DatepickerEvent::DecrementMonth => {
                self.shift_month(-1);
            }

            DatepickerEvent::SelectMonth(month) => {
                let view_date = self.view_date.get();
                self.set_view_date(view_date.year(), *month as u32 + 1, view_date.day());
            }

            DatepickerEvent::IncrementYear => {
                self.shift_year(1);
            }

            DatepickerEvent::DecrementYear => {
                self.shift_year(-1);
            }

            DatepickerEvent::SelectYear(year) => {
                if let Ok(year) = year.parse::<i32>() {
                    let view_date = self.view_date.get();
                    self.set_view_date(year, view_date.month(), view_date.day());
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

impl Handle<'_, Datepicker> {
    /// Set the callback triggered when a date is selected from the [Datepicker] view.
    pub fn on_select<F: 'static + Fn(&mut EventContext, NaiveDate)>(self, callback: F) -> Self {
        self.modify(|datepicker: &mut Datepicker| datepicker.on_select = Some(Box::new(callback)))
    }
}

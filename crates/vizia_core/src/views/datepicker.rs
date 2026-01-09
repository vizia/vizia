use chrono::{Datelike, NaiveDate, Weekday};

use crate::prelude::*;

/// A control used to select a date.
///
/// # Examples
///
/// ```
/// # use vizia_core::prelude::*;
/// # use chrono::NaiveDate;
/// # let mut cx = &mut Context::default();
/// # let date = cx.state(NaiveDate::from_ymd_opt(2025, 1, 15).unwrap());
/// Datepicker::new(cx, date)
///     .on_select(move |cx, selected| {
///         date.set(cx, selected);
///     });
/// ```
pub struct Datepicker {
    view_date: Signal<NaiveDate>,
    selected_month: Signal<usize>,
    year: Signal<i32>,
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
    SetYear(i32),

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

    /// Create a new [Datepicker] view.
    ///
    /// Accepts either a plain `NaiveDate` or a `Signal<NaiveDate>` for reactive state.
    /// Use `.on_select()` to handle date selection.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use chrono::NaiveDate;
    /// # let mut cx = &mut Context::default();
    /// // Static date (read-only display)
    /// Datepicker::new(cx, NaiveDate::from_ymd_opt(2025, 1, 15).unwrap());
    ///
    /// // Reactive with callback
    /// # let date = cx.state(NaiveDate::from_ymd_opt(2025, 1, 15).unwrap());
    /// Datepicker::new(cx, date)
    ///     .on_select(move |cx, selected| {
    ///         date.set(cx, selected);
    ///     });
    /// ```
    pub fn new(cx: &mut Context, selected_date: impl Res<NaiveDate> + 'static) -> Handle<Self> {
        let selected_date = selected_date.into_signal(cx);
        let initial_date = *selected_date.get(cx);

        let months = cx.state(MONTHS.iter().map(|m| Localized::new(m)).collect::<Vec<_>>());
        let selected_month = cx.state(initial_date.month() as usize - 1);
        let view_date = cx
            .state(NaiveDate::from_ymd_opt(initial_date.year(), initial_date.month(), 1).unwrap());

        // Signal for editable year
        let year = cx.state(initial_date.year());

        Self { view_date, selected_month, year, on_select: None }.build(cx, move |cx| {
            HStack::new(cx, |cx| {
                Spinbox::custom(cx, move |cx| {
                    PickList::new(cx, months, selected_month, false)
                        .on_select(|ex, index| ex.emit(DatepickerEvent::SelectMonth(index)))
                        .width(Stretch(1.0))
                })
                .width(Pixels(140.0))
                .on_increment(|ex| ex.emit(DatepickerEvent::IncrementMonth))
                .on_decrement(|ex| ex.emit(DatepickerEvent::DecrementMonth));

                Spinbox::custom(cx, move |cx| {
                    Textbox::new(cx, year)
                        .width(Stretch(1.0))
                        .padding(Pixels(1.0))
                        .on_submit(move |ex, value, _| {
                            ex.emit(DatepickerEvent::SetYear(value));
                        })
                })
                .width(Pixels(100.0))
                .icons(SpinboxIcons::PlusMinus)
                .on_increment(move |ex| {
                    let current = *year.get(ex);
                    year.set(ex, current + 1);
                    ex.emit(DatepickerEvent::IncrementYear);
                })
                .on_decrement(move |ex| {
                    let current = *year.get(ex);
                    year.set(ex, current - 1);
                    ex.emit(DatepickerEvent::DecrementYear);
                });
            })
            .class("datepicker-header");

            Divider::new(cx);

            VStack::new(cx, move |cx| {
                // Days of the week
                HStack::new(cx, |cx| {
                    for h in DAYS_HEADER {
                        let day_label =
                            Localized::new(h).map(|day| day[0..2].to_string()).signal(cx);
                        Label::new(cx, day_label).class("datepicker-calendar-header");
                    }
                })
                .class("datepicker-calendar-headers");

                // Numbered days in a grid
                VStack::new(cx, move |cx| {
                    for y in 0..6 {
                        HStack::new(cx, move |cx| {
                            for x in 0..7 {
                                let day_number = cx.derived({
                                    let view_date = view_date;
                                    move |store| {
                                        let vd_val = *view_date.get(store);
                                        let (day_number, _) = Self::get_day_number(y, x, &vd_val);
                                        day_number
                                    }
                                });
                                let is_disabled = cx.derived({
                                    let view_date = view_date;
                                    move |store| {
                                        let vd_val = *view_date.get(store);
                                        let (_, disabled) = Self::get_day_number(y, x, &vd_val);
                                        disabled
                                    }
                                });
                                let day_text = cx.derived({
                                    let day_number = day_number;
                                    move |store| day_number.get(store).to_string()
                                });
                                let is_selected = cx.derived({
                                    let view_date = view_date;
                                    let selected_date = selected_date;
                                    let day_number = day_number;
                                    let is_disabled = is_disabled;
                                    move |store| {
                                        if *is_disabled.get(store) {
                                            false
                                        } else {
                                            let sd_val = *selected_date.get(store);
                                            let vd_val = *view_date.get(store);
                                            let day_number = *day_number.get(store);
                                            sd_val.day() == day_number
                                                && sd_val.month() == vd_val.month()
                                                && sd_val.year() == vd_val.year()
                                        }
                                    }
                                });
                                let navigable = cx.derived({
                                    let is_disabled = is_disabled;
                                    move |store| !*is_disabled.get(store)
                                });

                                Label::new(cx, day_text)
                                    .class("datepicker-calendar-day")
                                    .navigable(navigable)
                                    .toggle_class("datepicker-calendar-day-disabled", is_disabled)
                                    .checked(is_selected)
                                    .on_press(move |ex| {
                                        if *is_disabled.get(ex) {
                                            return;
                                        }
                                        let day_number = *day_number.get(ex);
                                        let vd_val = *view_date.get(ex);
                                        ex.emit(DatepickerEvent::SelectDate(
                                            NaiveDate::from_ymd_opt(
                                                vd_val.year(),
                                                vd_val.month(),
                                                day_number,
                                            )
                                            .unwrap(),
                                        ))
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
                let current = *self.view_date.get(cx);
                let new_date = if current.month() == 12 {
                    NaiveDate::from_ymd_opt(current.year() + 1, 1, current.day()).unwrap()
                } else {
                    NaiveDate::from_ymd_opt(current.year(), current.month() + 1, current.day())
                        .unwrap()
                };
                self.view_date.set(cx, new_date);
                self.selected_month.set(cx, new_date.month() as usize - 1);
                self.year.set(cx, new_date.year());
            }

            DatepickerEvent::DecrementMonth => {
                let current = *self.view_date.get(cx);
                let new_date = if current.month() == 1 {
                    NaiveDate::from_ymd_opt(current.year() - 1, 12, current.day()).unwrap()
                } else {
                    NaiveDate::from_ymd_opt(current.year(), current.month() - 1, current.day())
                        .unwrap()
                };
                self.view_date.set(cx, new_date);
                self.selected_month.set(cx, new_date.month() as usize - 1);
                self.year.set(cx, new_date.year());
            }

            DatepickerEvent::SelectMonth(month) => {
                let current = *self.view_date.get(cx);
                let new_date =
                    NaiveDate::from_ymd_opt(current.year(), *month as u32 + 1, current.day())
                        .unwrap();
                self.view_date.set(cx, new_date);
                self.selected_month.set(cx, *month);
            }

            DatepickerEvent::IncrementYear => {
                let current = *self.view_date.get(cx);
                let new_date =
                    NaiveDate::from_ymd_opt(current.year() + 1, current.month(), current.day())
                        .unwrap();
                self.view_date.set(cx, new_date);
                self.year.set(cx, new_date.year());
            }

            DatepickerEvent::DecrementYear => {
                let current = *self.view_date.get(cx);
                let new_date =
                    NaiveDate::from_ymd_opt(current.year() - 1, current.month(), current.day())
                        .unwrap();
                self.view_date.set(cx, new_date);
                self.year.set(cx, new_date.year());
            }

            DatepickerEvent::SetYear(new_year) => {
                let current = *self.view_date.get(cx);
                if let Some(new_date) =
                    NaiveDate::from_ymd_opt(*new_year, current.month(), current.day())
                {
                    self.view_date.set(cx, new_date);
                    self.year.set(cx, new_date.year());
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

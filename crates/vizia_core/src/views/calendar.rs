use chrono::{Datelike, Duration, NaiveDate, Weekday};

use crate::{
    icons::{ICON_CHEVRON_LEFT, ICON_CHEVRON_RIGHT},
    prelude::*,
};

/// A control used to select a date.
pub struct Calendar {
    view_date: Signal<NaiveDate>,
    focused_date: Signal<NaiveDate>,
    week_starts_on: Signal<Weekday>,
    keyboard_help: Signal<String>,
    keyboard_help_announced: bool,
    day_cells: Vec<Entity>,
    on_select: Option<Box<dyn Fn(&mut EventContext, NaiveDate)>>,
}

const MONTHS: [&str; 12] =
    ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sept", "Oct", "Nov", "Dec"];

pub enum CalendarEvent {
    IncrementMonth,
    DecrementMonth,
    SelectMonth(usize),

    IncrementYear,
    DecrementYear,
    SelectYear(i32),

    MoveFocusDays(i64),
    MoveFocusLeft,
    MoveFocusRight,
    MoveFocusWeeks(i64),
    MoveFocusWeekStart,
    MoveFocusWeekEnd,
    MoveFocusMonth(i32),
    MoveFocusYear(i32),
    ActivateFocusedDate,
    RegisterDayCell(usize, usize, Entity),
    AnnounceKeyboardHelp,
    RefreshLocalization,

    SelectDate(NaiveDate),
}

impl Calendar {
    fn weekday_from_monday_index(index: u32) -> Weekday {
        match index % 7 {
            0 => Weekday::Mon,
            1 => Weekday::Tue,
            2 => Weekday::Wed,
            3 => Weekday::Thu,
            4 => Weekday::Fri,
            5 => Weekday::Sat,
            _ => Weekday::Sun,
        }
    }

    fn weekday_key(weekday: Weekday) -> &'static str {
        match weekday {
            Weekday::Mon => "Monday",
            Weekday::Tue => "Tuesday",
            Weekday::Wed => "Wednesday",
            Weekday::Thu => "Thursday",
            Weekday::Fri => "Friday",
            Weekday::Sat => "Saturday",
            Weekday::Sun => "Sunday",
        }
    }

    fn weekday_short_key(weekday: Weekday) -> &'static str {
        match weekday {
            Weekday::Mon => "Monday-short",
            Weekday::Tue => "Tuesday-short",
            Weekday::Wed => "Wednesday-short",
            Weekday::Thu => "Thursday-short",
            Weekday::Fri => "Friday-short",
            Weekday::Sat => "Saturday-short",
            Weekday::Sun => "Sunday-short",
        }
    }

    fn first_day_column(first_day: Weekday, week_starts_on: Weekday) -> u32 {
        let first = first_day.num_days_from_monday() as i32;
        let start = week_starts_on.num_days_from_monday() as i32;
        ((7 + first - start) % 7) as u32
    }

    fn weekday_from_token(token: &str) -> Option<Weekday> {
        match token.trim().to_ascii_lowercase().as_str() {
            "monday" => Some(Weekday::Mon),
            "tuesday" => Some(Weekday::Tue),
            "wednesday" => Some(Weekday::Wed),
            "thursday" => Some(Weekday::Thu),
            "friday" => Some(Weekday::Fri),
            "saturday" => Some(Weekday::Sat),
            "sunday" => Some(Weekday::Sun),
            _ => None,
        }
    }

    fn week_start_from_localization(cx: &impl DataContext) -> Weekday {
        let val = Localized::new("calendar-week-start").to_string_local(cx);
        Self::weekday_from_token(&val).unwrap_or(Weekday::Mon)
    }

    fn set_view_date(&mut self, year: i32, month: u32, day: u32) {
        let clamped_day = day.min(Self::last_day_of_month(year, month).unwrap());
        let date = NaiveDate::from_ymd_opt(year, month, clamped_day).unwrap();
        self.view_date.set(date);
        self.focused_date.set(date);
    }

    fn shift_month(&mut self, delta: i32) {
        let focused_date = self.focused_date.get();
        let (year, month) =
            Self::shift_year_month(focused_date.year(), focused_date.month(), delta);
        self.set_view_date(year, month, focused_date.day());
    }

    fn shift_year(&mut self, delta: i32) {
        let focused_date = self.focused_date.get();
        self.set_view_date(focused_date.year() + delta, focused_date.month(), focused_date.day());
    }

    fn shift_year_month(year: i32, month: u32, delta: i32) -> (i32, u32) {
        let mut year = year;
        let mut month = month as i32 + delta;

        while month < 1 {
            year -= 1;
            month += 12;
        }

        while month > 12 {
            year += 1;
            month -= 12;
        }

        (year, month as u32)
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

    fn get_day_number(
        y: u32,
        x: u32,
        view_date: &NaiveDate,
        week_starts_on: Weekday,
    ) -> (u32, bool) {
        let (_, days_prev_month) = Self::view_month_info(view_date, -1);
        let (first_day_this_month, days_this_month) = Self::view_month_info(view_date, 0);

        let fdtm_i = Self::first_day_column(first_day_this_month, week_starts_on);

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

    fn get_cell_date(
        y: u32,
        x: u32,
        view_date: &NaiveDate,
        week_starts_on: Weekday,
    ) -> (NaiveDate, bool) {
        let (day_number, disabled) = Self::get_day_number(y, x, view_date, week_starts_on);

        if !disabled {
            return (
                NaiveDate::from_ymd_opt(view_date.year(), view_date.month(), day_number).unwrap(),
                false,
            );
        }

        if y == 0 {
            let (year, month) = Self::shift_year_month(view_date.year(), view_date.month(), -1);
            (NaiveDate::from_ymd_opt(year, month, day_number).unwrap(), true)
        } else {
            let (year, month) = Self::shift_year_month(view_date.year(), view_date.month(), 1);
            (NaiveDate::from_ymd_opt(year, month, day_number).unwrap(), true)
        }
    }

    fn day_cell_index(y: usize, x: usize) -> usize {
        y * 7 + x
    }

    fn focused_day_cell_index(&self) -> Option<usize> {
        let focused_date = self.focused_date.get();
        let view_date = self.view_date.get();

        if focused_date.year() != view_date.year() || focused_date.month() != view_date.month() {
            return None;
        }

        let first_day = Self::first_day_of_month(view_date.year(), view_date.month()).unwrap();
        let first_day_col = Self::first_day_column(first_day, self.week_starts_on.get()) as usize;
        Some(first_day_col + focused_date.day() as usize - 1)
    }

    fn focus_focused_day(&self, cx: &mut EventContext) {
        if let Some(index) = self.focused_day_cell_index()
            && let Some(entity) = self.day_cells.get(index).copied()
            && !entity.is_null()
        {
            cx.with_current(entity, |cx| cx.focus());
        }
    }

    fn focus_focused_day_with_visibility(&self, cx: &mut EventContext, focus_visible: bool) {
        if let Some(index) = self.focused_day_cell_index()
            && let Some(entity) = self.day_cells.get(index).copied()
            && !entity.is_null()
        {
            cx.with_current(entity, |cx| cx.focus_with_visibility(focus_visible));
        }
    }

    fn move_focused_by_days(&mut self, delta_days: i64) {
        let focused_date = self.focused_date.get();
        let next_date = focused_date.checked_add_signed(Duration::days(delta_days)).unwrap();
        self.view_date.set(next_date);
        self.focused_date.set(next_date);
    }

    fn move_focused_by_months(&mut self, delta_months: i32) {
        let focused_date = self.focused_date.get();
        let (year, month) =
            Self::shift_year_month(focused_date.year(), focused_date.month(), delta_months);
        self.set_view_date(year, month, focused_date.day());
    }

    fn move_focused_by_years(&mut self, delta_years: i32) {
        let focused_date = self.focused_date.get();
        self.set_view_date(
            focused_date.year() + delta_years,
            focused_date.month(),
            focused_date.day(),
        );
    }

    fn focus_is_on_calendar_day(&self, cx: &EventContext) -> bool {
        let focused = cx.focused();
        self.day_cells.iter().any(|cell| {
            !cell.is_null() && (focused == *cell || focused.is_descendant_of(cx.tree, *cell))
        })
    }

    /// Create a new [Calendar] view.
    pub fn new<R, D>(cx: &mut Context, date: R) -> Handle<Self>
    where
        R: Res<D> + Clone + 'static,
        D: Datelike + Clone + 'static,
    {
        let selected_date = date.get_value(cx);
        let selected_date_signal = date.to_signal(cx);
        let initial_view_date =
            NaiveDate::from_ymd_opt(selected_date.year(), selected_date.month(), 1).unwrap();
        let view_date = Signal::new(initial_view_date);
        let focused_date = Signal::new(
            NaiveDate::from_ymd_opt(
                selected_date.year(),
                selected_date.month(),
                selected_date.day(),
            )
            .unwrap(),
        );
        let month_options =
            Signal::new(MONTHS.iter().map(|m| Localized::new(m)).collect::<Vec<_>>());
        let month_year_heading_date =
            view_date.map(|date| date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_millis());
        let week_starts_on = Signal::new(Self::week_start_from_localization(cx));
        let keyboard_help = Signal::new(String::new());
        let selected_month = view_date.map(|date| Some(date.month() as usize - 1));
        let year_start = selected_date.year() - 50;
        let year_end = selected_date.year() + 50;
        let year_options =
            Signal::new((year_start..=year_end).map(|year| year.to_string()).collect::<Vec<_>>());
        let selected_year = view_date.map(move |date| {
            let index = date.year() - year_start;
            if (0..=(year_end - year_start)).contains(&index) { Some(index as usize) } else { None }
        });

        Self {
            view_date,
            focused_date,
            week_starts_on,
            keyboard_help,
            keyboard_help_announced: false,
            day_cells: vec![Entity::null(); 42],
            on_select: None,
        }
        .build(cx, move |cx| {
            let calendar_entity = cx.current();
            let locale = cx.environment().locale;
            locale.set_or_bind(cx, move |cx, _| {
                cx.emit_to(calendar_entity, CalendarEvent::RefreshLocalization);
            });

            Keymap::from(vec![
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowUp),
                    KeymapEntry::new("Calendar Focus Previous Week", |cx| {
                        cx.emit(CalendarEvent::MoveFocusWeeks(-1))
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowDown),
                    KeymapEntry::new("Calendar Focus Next Week", |cx| {
                        cx.emit(CalendarEvent::MoveFocusWeeks(1))
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowLeft),
                    KeymapEntry::new("Calendar Focus Left Day", |cx| {
                        cx.emit(CalendarEvent::MoveFocusLeft)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowRight),
                    KeymapEntry::new("Calendar Focus Right Day", |cx| {
                        cx.emit(CalendarEvent::MoveFocusRight)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Home),
                    KeymapEntry::new("Calendar Focus Week Start", |cx| {
                        cx.emit(CalendarEvent::MoveFocusWeekStart)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::End),
                    KeymapEntry::new("Calendar Focus Week End", |cx| {
                        cx.emit(CalendarEvent::MoveFocusWeekEnd)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::PageUp),
                    KeymapEntry::new("Calendar Previous Month", |cx| {
                        cx.emit(CalendarEvent::MoveFocusMonth(-1))
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::PageDown),
                    KeymapEntry::new("Calendar Next Month", |cx| {
                        cx.emit(CalendarEvent::MoveFocusMonth(1))
                    }),
                ),
                (
                    KeyChord::new(Modifiers::SHIFT, Code::PageUp),
                    KeymapEntry::new("Calendar Previous Year", |cx| {
                        cx.emit(CalendarEvent::MoveFocusYear(-1))
                    }),
                ),
                (
                    KeyChord::new(Modifiers::SHIFT, Code::PageDown),
                    KeymapEntry::new("Calendar Next Year", |cx| {
                        cx.emit(CalendarEvent::MoveFocusYear(1))
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Space),
                    KeymapEntry::new("Calendar Activate Date", |cx| {
                        cx.emit(CalendarEvent::ActivateFocusedDate)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Enter),
                    KeymapEntry::new("Calendar Activate Date", |cx| {
                        cx.emit(CalendarEvent::ActivateFocusedDate)
                    }),
                ),
            ])
            .build(cx);

            HStack::new(cx, move |cx| {
                Button::new(cx, |cx| Svg::new(cx, ICON_CHEVRON_LEFT))
                    .on_press(|ex| ex.emit(CalendarEvent::DecrementMonth))
                    .variant(ButtonVariant::Text)
                    .name(Localized::new("calendar-previous-month"))
                    .class("month-nav");
                HStack::new(cx, move |cx| {
                    Select::new(cx, month_options, selected_month, true)
                        .on_select(|ex, index| ex.emit(CalendarEvent::SelectMonth(index)));
                    Select::new(cx, year_options, selected_year, true).on_select(
                        move |ex, index| {
                            ex.emit(CalendarEvent::SelectYear(year_start + index as i32));
                        },
                    );
                })
                .class("calendar-controls-select");

                Button::new(cx, |cx| Svg::new(cx, ICON_CHEVRON_RIGHT))
                    .on_press(|ex| ex.emit(CalendarEvent::IncrementMonth))
                    .variant(ButtonVariant::Text)
                    .name(Localized::new("calendar-next-month"))
                    .class("month-nav");
            })
            .class("calendar-controls");

            VStack::new(cx, move |cx| {
                Label::new(
                    cx,
                    Localized::new("calendar-month-year-heading")
                        .arg("date", month_year_heading_date),
                )
                .class("calendar-month-year-heading")
                .role(Role::Label)
                .display(Display::None)
                .live(Live::Polite);

                Label::new(cx, keyboard_help)
                    .class("calendar-keyboard-help")
                    .role(Role::Label)
                    .display(Display::None)
                    .live(Live::Polite);

                // Days of the week
                HStack::new(cx, |cx| {
                    for x in 0..7 {
                        let week_starts_on = cx.data::<Calendar>().week_starts_on;
                        Label::new(cx, "")
                            .bind(week_starts_on, move |handle| {
                                let week_starts_on = week_starts_on.get();
                                let weekday = Self::weekday_from_monday_index(
                                    week_starts_on.num_days_from_monday() + x,
                                );
                                handle
                                    .text(Localized::new(Self::weekday_short_key(weekday)))
                                    .name(Localized::new(Self::weekday_key(weekday)));
                            })
                            .class("calendar-dow")
                            .role(Role::ColumnHeader);
                    }
                })
                .class("calendar-header")
                .role(Role::Row);

                // Numbered days in a grid
                VStack::new(cx, move |cx| {
                    for y in 0..6 {
                        HStack::new(cx, |cx| {
                            for x in 0..7 {
                                let selected_date = selected_date_signal;
                                let view_date = cx.data::<Calendar>().view_date;
                                let focused_date = cx.data::<Calendar>().focused_date;
                                let week_starts_on = cx.data::<Calendar>().week_starts_on;
                                Label::new(cx, "").bind(view_date, move |handle| {
                                    let view_date = view_date.get();
                                    let selected_date = selected_date;
                                    let focused_date = focused_date;
                                    handle.bind(week_starts_on, move |handle| {
                                        let week_starts_on = week_starts_on.get();
                                        let (cell_date, disabled) =
                                            Self::get_cell_date(y, x, &view_date, week_starts_on);
                                        let day_number = cell_date.day();

                                        handle.bind(selected_date, move |handle| {
                                            let selected_date = selected_date.get();
                                            handle.bind(focused_date, move |handle| {
                                                let focused_date = focused_date.get();
                                                let is_focused = focused_date == cell_date;

                                                handle
                                                    .text(day_number.to_string())
                                                    .class("calendar-day")
                                                    .role(Role::GridCell)
                                                    .name(
                                                        Localized::new("calendar-day-cell-name")
                                                            .arg(
                                                                "date",
                                                                cell_date
                                                                    .and_hms_opt(0, 0, 0)
                                                                    .unwrap()
                                                                    .and_utc()
                                                                    .timestamp_millis(),
                                                            ),
                                                    )
                                                    .disabled(disabled)
                                                    .navigable(!disabled && is_focused)
                                                    .toggle_class("calendar-day-disabled", disabled)
                                                    .toggle_class(
                                                        "calendar-day-focused",
                                                        is_focused,
                                                    )
                                                    .on_build(move |cx| {
                                                        cx.emit(CalendarEvent::RegisterDayCell(
                                                            y as usize,
                                                            x as usize,
                                                            cx.current(),
                                                        ));
                                                    })
                                                    .on_focus_in(|ex| {
                                                        ex.emit(CalendarEvent::AnnounceKeyboardHelp)
                                                    })
                                                    .on_press(move |ex| {
                                                        if !disabled {
                                                            ex.emit(CalendarEvent::SelectDate(
                                                                cell_date,
                                                            ));
                                                        }
                                                    })
                                                    .toggle_class(
                                                        "calendar-day-selected",
                                                        !disabled
                                                            && selected_date.day()
                                                                == cell_date.day()
                                                            && selected_date.month()
                                                                == cell_date.month()
                                                            && selected_date.year()
                                                                == cell_date.year(),
                                                    )
                                                    .selected(
                                                        !disabled
                                                            && selected_date.day()
                                                                == cell_date.day()
                                                            && selected_date.month()
                                                                == cell_date.month()
                                                            && selected_date.year()
                                                                == cell_date.year(),
                                                    );
                                            });
                                        });
                                    });
                                });
                            }
                        })
                        .role(Role::Row);
                    }
                })
                // This shouldn't be needed but apparently grid size isn't propagated up the tree during layout
                .width(Pixels(32.0 * 7.0))
                .height(Pixels(32.0 * 6.0));
            })
            .class("calendar-body")
            .role(Role::Grid)
            .name(
                Localized::new("calendar-month-year-heading").arg("date", month_year_heading_date),
            );
        })
    }
}

impl View for Calendar {
    fn element(&self) -> Option<&'static str> {
        Some("calendar")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            CalendarEvent::IncrementMonth => {
                self.shift_month(1);
            }

            CalendarEvent::DecrementMonth => {
                self.shift_month(-1);
            }

            CalendarEvent::SelectMonth(month) => {
                let focused_date = self.focused_date.get();
                self.set_view_date(focused_date.year(), *month as u32 + 1, focused_date.day());
            }

            CalendarEvent::IncrementYear => {
                self.shift_year(1);
            }

            CalendarEvent::DecrementYear => {
                self.shift_year(-1);
            }

            CalendarEvent::SelectYear(year) => {
                let focused_date = self.focused_date.get();
                self.set_view_date(*year, focused_date.month(), focused_date.day());
            }

            CalendarEvent::MoveFocusDays(delta) => {
                if self.focus_is_on_calendar_day(cx) {
                    self.move_focused_by_days(*delta);
                    self.focus_focused_day_with_visibility(cx, true);
                }
            }

            CalendarEvent::MoveFocusLeft => {
                if self.focus_is_on_calendar_day(cx) {
                    let delta = if cx.environment().direction.get() == Direction::RightToLeft {
                        1
                    } else {
                        -1
                    };
                    self.move_focused_by_days(delta);
                    self.focus_focused_day_with_visibility(cx, true);
                }
            }

            CalendarEvent::MoveFocusRight => {
                if self.focus_is_on_calendar_day(cx) {
                    let delta = if cx.environment().direction.get() == Direction::RightToLeft {
                        -1
                    } else {
                        1
                    };
                    self.move_focused_by_days(delta);
                    self.focus_focused_day_with_visibility(cx, true);
                }
            }

            CalendarEvent::MoveFocusWeeks(delta) => {
                if self.focus_is_on_calendar_day(cx) {
                    self.move_focused_by_days(*delta * 7);
                    self.focus_focused_day_with_visibility(cx, true);
                }
            }

            CalendarEvent::MoveFocusWeekStart => {
                if self.focus_is_on_calendar_day(cx) {
                    let focused_date = self.focused_date.get();
                    let offset = (7 + focused_date.weekday().num_days_from_monday() as i64
                        - self.week_starts_on.get().num_days_from_monday() as i64)
                        % 7;
                    self.move_focused_by_days(-offset);
                    self.focus_focused_day_with_visibility(cx, true);
                }
            }

            CalendarEvent::MoveFocusWeekEnd => {
                if self.focus_is_on_calendar_day(cx) {
                    let focused_date = self.focused_date.get();
                    let offset = 6
                        - ((7 + focused_date.weekday().num_days_from_monday() as i64
                            - self.week_starts_on.get().num_days_from_monday() as i64)
                            % 7);
                    self.move_focused_by_days(offset);
                    self.focus_focused_day_with_visibility(cx, true);
                }
            }

            CalendarEvent::MoveFocusMonth(delta) => {
                if self.focus_is_on_calendar_day(cx) {
                    self.move_focused_by_months(*delta);
                    self.focus_focused_day_with_visibility(cx, true);
                }
            }

            CalendarEvent::MoveFocusYear(delta) => {
                if self.focus_is_on_calendar_day(cx) {
                    self.move_focused_by_years(*delta);
                    self.focus_focused_day_with_visibility(cx, true);
                }
            }

            CalendarEvent::ActivateFocusedDate => {
                if self.focus_is_on_calendar_day(cx) {
                    let focused_date = self.focused_date.get();
                    if focused_date.month() == self.view_date.get().month()
                        && focused_date.year() == self.view_date.get().year()
                    {
                        if let Some(callback) = &self.on_select {
                            (callback)(cx, focused_date);
                        }
                    }
                }
            }

            CalendarEvent::RegisterDayCell(y, x, entity) => {
                let index = Self::day_cell_index(*y, *x);
                if let Some(day_cell) = self.day_cells.get_mut(index) {
                    *day_cell = *entity;
                }
            }

            CalendarEvent::AnnounceKeyboardHelp => {
                if !self.keyboard_help_announced {
                    self.keyboard_help
                        .set(Localized::new("calendar-keyboard-help").to_string_local(cx));
                    self.keyboard_help_announced = true;
                }
            }

            CalendarEvent::RefreshLocalization => {
                self.week_starts_on.set(Self::week_start_from_localization(cx));
                self.keyboard_help.set(String::new());
                self.keyboard_help_announced = false;
            }

            CalendarEvent::SelectDate(date) => {
                self.focused_date.set(*date);
                self.view_date.set(*date);
                self.focus_focused_day(cx);
                if let Some(callback) = &self.on_select {
                    (callback)(cx, *date);
                }
            }
        });

        event.map(|window_event, meta| match window_event {
            WindowEvent::KeyDown(code, _)
                if self.focus_is_on_calendar_day(cx)
                    && matches!(
                        code,
                        Code::ArrowUp
                            | Code::ArrowDown
                            | Code::ArrowLeft
                            | Code::ArrowRight
                            | Code::Home
                            | Code::End
                            | Code::PageUp
                            | Code::PageDown
                            | Code::Space
                            | Code::Enter
                    ) =>
            {
                meta.consume();
            }

            _ => {}
        });
    }
}

impl Handle<'_, Calendar> {
    /// Set the callback triggered when a date is selected from the [Calendar] view.
    pub fn on_select<F: 'static + Fn(&mut EventContext, NaiveDate)>(self, callback: F) -> Self {
        self.modify(|calendar: &mut Calendar| calendar.on_select = Some(Box::new(callback)))
    }
}

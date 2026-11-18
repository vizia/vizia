use jiff::ToSpan;
use jiff::civil::{Date, Weekday};

use crate::{
    icons::{ICON_CHEVRON_LEFT, ICON_CHEVRON_RIGHT},
    localization::FluentCivilDateTime,
    prelude::*,
};

/// A control used to select a date.
pub struct Calendar {
    view_date: Signal<Date>,
    focused_date: Signal<Date>,
    week_starts_on: Signal<Weekday>,
    keyboard_help: Signal<String>,
    keyboard_help_announced: bool,
    day_cells: Vec<Entity>,
    on_select: Option<Box<dyn Fn(&mut EventContext, Date)>>,
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

    SelectDate(Date),
}

impl Calendar {
    fn weekday_from_monday_index(index: u32) -> Weekday {
        Weekday::from_monday_zero_offset((index % 7) as i8).unwrap()
    }

    fn weekday_key(weekday: Weekday) -> &'static str {
        match weekday {
            Weekday::Monday => "Monday",
            Weekday::Tuesday => "Tuesday",
            Weekday::Wednesday => "Wednesday",
            Weekday::Thursday => "Thursday",
            Weekday::Friday => "Friday",
            Weekday::Saturday => "Saturday",
            Weekday::Sunday => "Sunday",
        }
    }

    fn weekday_short_key(weekday: Weekday) -> &'static str {
        match weekday {
            Weekday::Monday => "Monday-short",
            Weekday::Tuesday => "Tuesday-short",
            Weekday::Wednesday => "Wednesday-short",
            Weekday::Thursday => "Thursday-short",
            Weekday::Friday => "Friday-short",
            Weekday::Saturday => "Saturday-short",
            Weekday::Sunday => "Sunday-short",
        }
    }

    fn first_day_column(first_day: Weekday, week_starts_on: Weekday) -> u32 {
        let first = i32::from(first_day.to_monday_zero_offset());
        let start = i32::from(week_starts_on.to_monday_zero_offset());
        ((7 + first - start) % 7) as u32
    }

    fn weekday_from_token(token: &str) -> Option<Weekday> {
        match token.trim().to_ascii_lowercase().as_str() {
            "monday" => Some(Weekday::Monday),
            "tuesday" => Some(Weekday::Tuesday),
            "wednesday" => Some(Weekday::Wednesday),
            "thursday" => Some(Weekday::Thursday),
            "friday" => Some(Weekday::Friday),
            "saturday" => Some(Weekday::Saturday),
            "sunday" => Some(Weekday::Sunday),
            _ => None,
        }
    }

    fn week_start_from_localization(cx: &impl DataContext) -> Weekday {
        let val = Localized::new("calendar-week-start").to_string_local(cx);
        Self::weekday_from_token(&val).unwrap_or(Weekday::Monday)
    }

    fn set_view_date(&mut self, date: Date) {
        self.view_date.set(date);
        self.focused_date.set(date);
    }

    /// Moves the focus by whole months, clamping the day to the length of the new month.
    fn shift_month(&mut self, delta: i32) {
        let date = self.focused_date.get().checked_add(delta.months()).unwrap();
        self.set_view_date(date);
    }

    /// Moves the focus by whole years, clamping the day to the length of the new month.
    fn shift_year(&mut self, delta: i32) {
        let date = self.focused_date.get().checked_add(delta.years()).unwrap();
        self.set_view_date(date);
    }

    // Given a date and a month offset, returns the first day of the month and the number of days in the month
    fn view_month_info(view_date: &Date, month_offset: i32) -> (Weekday, u32) {
        let month = view_date.first_of_month().checked_add(month_offset.months()).unwrap();

        (month.weekday(), u32::try_from(month.days_in_month()).unwrap())
    }

    fn get_day_number(y: u32, x: u32, view_date: &Date, week_starts_on: Weekday) -> (u32, bool) {
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

    fn get_cell_date(y: u32, x: u32, view_date: &Date, week_starts_on: Weekday) -> (Date, bool) {
        let (day_number, disabled) = Self::get_day_number(y, x, view_date, week_starts_on);

        let month_offset = match (disabled, y) {
            (false, _) => 0,
            (true, 0) => -1,
            (true, _) => 1,
        };
        let month = view_date.first_of_month().checked_add(month_offset.months()).unwrap();

        (month.with().day(i8::try_from(day_number).unwrap()).build().unwrap(), disabled)
    }

    fn day_cell_index(y: usize, x: usize) -> usize {
        y * 7 + x
    }

    fn focused_day_cell_index(&self) -> Option<usize> {
        let focused_date = self.focused_date.get();
        let view_date = self.view_date.get();

        if focused_date.first_of_month() != view_date.first_of_month() {
            return None;
        }

        let first_day = view_date.first_of_month().weekday();
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
        let next_date = focused_date.checked_add(delta_days.days()).unwrap();
        self.view_date.set(next_date);
        self.focused_date.set(next_date);
    }

    fn move_focused_by_months(&mut self, delta_months: i32) {
        self.shift_month(delta_months);
    }

    fn move_focused_by_years(&mut self, delta_years: i32) {
        self.shift_year(delta_years);
    }

    /// The signed number of days from `date` to the first day of its week.
    fn days_to_week_start(date: Date, week_starts_on: Weekday) -> i64 {
        -((7 + i64::from(date.weekday().to_monday_zero_offset())
            - i64::from(week_starts_on.to_monday_zero_offset()))
            % 7)
    }

    /// The signed number of days from `date` to the last day of its week.
    fn days_to_week_end(date: Date, week_starts_on: Weekday) -> i64 {
        6 + Self::days_to_week_start(date, week_starts_on)
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
        D: Into<Date> + Clone + 'static,
    {
        let selected_date: Date = date.get_value(cx).into();
        let selected_date_signal = date.to_signal(cx);
        let view_date = Signal::new(selected_date.first_of_month());
        let focused_date = Signal::new(selected_date);
        let month_options =
            Signal::new(MONTHS.iter().map(|m| Localized::new(m)).collect::<Vec<_>>());
        let month_year_heading_date =
            view_date.map(|date| FluentCivilDateTime(date.at(0, 0, 0, 0)));
        let week_starts_on = Signal::new(Self::week_start_from_localization(cx));
        let keyboard_help = Signal::new(String::new());
        let selected_month = view_date.map(|date| Some(date.month() as usize - 1));
        let year_start = i32::from(selected_date.year()) - 50;
        let year_end = i32::from(selected_date.year()) + 50;
        let year_options =
            Signal::new((year_start..=year_end).map(|year| year.to_string()).collect::<Vec<_>>());
        let selected_year = view_date.map(move |date| {
            let index = i32::from(date.year()) - year_start;
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
                                    u32::from(
                                        week_starts_on.to_monday_zero_offset().unsigned_abs(),
                                    ) + x,
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
                                            let selected_date: Date = selected_date.get().into();
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
                                                                FluentCivilDateTime(
                                                                    cell_date.at(0, 0, 0, 0),
                                                                ),
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
                                                        !disabled && selected_date == cell_date,
                                                    )
                                                    .selected(
                                                        !disabled && selected_date == cell_date,
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
                let months = *month as i32 + 1 - i32::from(self.focused_date.get().month());
                self.shift_month(months);
            }

            CalendarEvent::IncrementYear => {
                self.shift_year(1);
            }

            CalendarEvent::DecrementYear => {
                self.shift_year(-1);
            }

            CalendarEvent::SelectYear(year) => {
                let years = *year - i32::from(self.focused_date.get().year());
                self.shift_year(years);
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
                    let offset = Self::days_to_week_start(focused_date, self.week_starts_on.get());
                    self.move_focused_by_days(offset);
                    self.focus_focused_day_with_visibility(cx, true);
                }
            }

            CalendarEvent::MoveFocusWeekEnd => {
                if self.focus_is_on_calendar_day(cx) {
                    let focused_date = self.focused_date.get();
                    let offset = Self::days_to_week_end(focused_date, self.week_starts_on.get());
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
                        && let Some(callback) = &self.on_select
                    {
                        (callback)(cx, focused_date);
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
    pub fn on_select<F: 'static + Fn(&mut EventContext, Date)>(self, callback: F) -> Self {
        self.modify(|calendar: &mut Calendar| calendar.on_select = Some(Box::new(callback)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i16, month: i8, day: i8) -> Date {
        Date::new(year, month, day).unwrap()
    }

    fn calendar(focused: Date, week_starts_on: Weekday) -> Calendar {
        Calendar {
            view_date: Signal::new(focused.first_of_month()),
            focused_date: Signal::new(focused),
            week_starts_on: Signal::new(week_starts_on),
            keyboard_help: Signal::new(String::new()),
            keyboard_help_announced: false,
            day_cells: vec![Entity::null(); 42],
            on_select: None,
        }
    }

    #[test]
    fn month_length_handles_leap_years_and_december() {
        let days = |year, month| Calendar::view_month_info(&date(year, month, 1), 0).1;

        assert_eq!(days(2024, 2), 29);
        assert_eq!(days(2023, 2), 28);
        assert_eq!(days(1900, 2), 28);
        assert_eq!(days(2000, 2), 29);
        assert_eq!(days(2023, 12), 31);
        assert_eq!(days(2023, 4), 30);
    }

    #[test]
    fn first_day_of_month_is_the_weekday_of_the_first() {
        let first_day = |year, month| Calendar::view_month_info(&date(year, month, 1), 0).0;

        assert_eq!(first_day(2024, 2), Weekday::Thursday);
        assert_eq!(first_day(2023, 12), Weekday::Friday);
        assert_eq!(first_day(2023, 10), Weekday::Sunday);
        assert_eq!(Calendar::view_month_info(&date(2024, 2, 17), 0), (Weekday::Thursday, 29));
    }

    #[test]
    fn first_day_column_counts_from_the_week_start() {
        assert_eq!(Calendar::first_day_column(Weekday::Thursday, Weekday::Monday), 3);
        assert_eq!(Calendar::first_day_column(Weekday::Thursday, Weekday::Sunday), 4);
        assert_eq!(Calendar::first_day_column(Weekday::Sunday, Weekday::Monday), 6);
        assert_eq!(Calendar::first_day_column(Weekday::Monday, Weekday::Saturday), 2);
        assert_eq!(Calendar::first_day_column(Weekday::Wednesday, Weekday::Wednesday), 0);
    }

    #[test]
    fn view_month_info_wraps_across_years() {
        assert_eq!(Calendar::view_month_info(&date(2024, 1, 1), -1), (Weekday::Friday, 31));
        assert_eq!(Calendar::view_month_info(&date(2023, 12, 1), 1), (Weekday::Monday, 31));
        assert_eq!(Calendar::view_month_info(&date(2024, 3, 1), -1), (Weekday::Thursday, 29));
    }

    #[test]
    fn cell_dates_for_a_leap_february_with_a_monday_start() {
        let view = date(2024, 2, 1);
        let cell = |y, x| Calendar::get_cell_date(y, x, &view, Weekday::Monday);

        assert_eq!(cell(0, 0), (date(2024, 1, 29), true));
        assert_eq!(cell(0, 2), (date(2024, 1, 31), true));
        assert_eq!(cell(0, 3), (date(2024, 2, 1), false));
        assert_eq!(cell(4, 3), (date(2024, 2, 29), false));
        assert_eq!(cell(4, 4), (date(2024, 3, 1), true));
        assert_eq!(cell(5, 6), (date(2024, 3, 10), true));
    }

    #[test]
    fn cell_dates_for_a_leap_february_with_a_sunday_start() {
        let view = date(2024, 2, 1);
        let cell = |y, x| Calendar::get_cell_date(y, x, &view, Weekday::Sunday);

        assert_eq!(cell(0, 0), (date(2024, 1, 28), true));
        assert_eq!(cell(0, 4), (date(2024, 2, 1), false));
        assert_eq!(cell(4, 4), (date(2024, 2, 29), false));
        assert_eq!(cell(4, 5), (date(2024, 3, 1), true));
    }

    #[test]
    fn cell_dates_roll_over_year_boundaries() {
        let december = date(2023, 12, 1);
        assert_eq!(
            Calendar::get_cell_date(5, 6, &december, Weekday::Monday),
            (date(2024, 1, 7), true)
        );

        let january = date(2025, 1, 1);
        assert_eq!(
            Calendar::get_cell_date(0, 0, &january, Weekday::Monday),
            (date(2024, 12, 30), true)
        );
        assert_eq!(
            Calendar::get_cell_date(0, 2, &january, Weekday::Monday),
            (date(2025, 1, 1), false)
        );
    }

    #[test]
    fn shifting_the_month_clamps_the_day_and_wraps_the_year() {
        let mut calendar = calendar(date(2024, 1, 31), Weekday::Monday);

        calendar.shift_month(1);
        assert_eq!(calendar.view_date.get(), date(2024, 2, 29));
        assert_eq!(calendar.focused_date.get(), date(2024, 2, 29));

        calendar.shift_month(-2);
        assert_eq!(calendar.focused_date.get(), date(2023, 12, 29));

        calendar.shift_month(1);
        calendar.shift_month(1);
        assert_eq!(calendar.focused_date.get(), date(2024, 2, 29));
    }

    #[test]
    fn shifting_the_year_clamps_leap_days() {
        let mut calendar = calendar(date(2024, 2, 29), Weekday::Monday);

        calendar.shift_year(1);
        assert_eq!(calendar.focused_date.get(), date(2025, 2, 28));

        calendar.shift_year(-1);
        assert_eq!(calendar.focused_date.get(), date(2024, 2, 28));
    }

    #[test]
    fn moving_the_focus_by_days_crosses_month_and_year_boundaries() {
        let mut calendar = calendar(date(2023, 12, 31), Weekday::Monday);

        calendar.move_focused_by_days(1);
        assert_eq!(calendar.focused_date.get(), date(2024, 1, 1));
        assert_eq!(calendar.view_date.get(), date(2024, 1, 1));

        calendar.move_focused_by_days(-1);
        assert_eq!(calendar.focused_date.get(), date(2023, 12, 31));

        calendar.move_focused_by_days(60);
        assert_eq!(calendar.focused_date.get(), date(2024, 2, 29));

        calendar.move_focused_by_days(-366);
        assert_eq!(calendar.focused_date.get(), date(2023, 2, 28));
    }

    #[test]
    fn moving_the_focus_by_months_and_years_clamps_the_day() {
        let mut calendar = calendar(date(2024, 3, 31), Weekday::Monday);

        calendar.move_focused_by_months(-1);
        assert_eq!(calendar.focused_date.get(), date(2024, 2, 29));

        calendar.move_focused_by_years(-1);
        assert_eq!(calendar.focused_date.get(), date(2023, 2, 28));
    }

    #[test]
    fn the_focused_day_cell_follows_the_week_start() {
        let calendar = calendar(date(2024, 2, 14), Weekday::Monday);
        assert_eq!(calendar.focused_day_cell_index(), Some(3 + 13));

        calendar.week_starts_on.set(Weekday::Sunday);
        assert_eq!(calendar.focused_day_cell_index(), Some(4 + 13));

        calendar.view_date.set(date(2024, 3, 1));
        assert_eq!(calendar.focused_day_cell_index(), None);
    }

    #[test]
    fn week_start_and_end_moves_follow_the_week_start() {
        let weekdays = [
            Weekday::Monday,
            Weekday::Tuesday,
            Weekday::Wednesday,
            Weekday::Thursday,
            Weekday::Friday,
            Weekday::Saturday,
            Weekday::Sunday,
        ];

        // 2024-02-12 is a Monday, so the seven dates cover every weekday.
        for day in 12..=18 {
            let focused = date(2024, 2, day);

            for week_starts_on in weekdays {
                // Walk back to the first day of the week, then forward to the day before the next one.
                let mut start = focused;
                while start.weekday() != week_starts_on {
                    start = start.checked_add((-1).days()).unwrap();
                }
                let mut end = focused;
                while end.checked_add(1.day()).unwrap().weekday() != week_starts_on {
                    end = end.checked_add(1.day()).unwrap();
                }

                let mut moved = calendar(focused, week_starts_on);
                moved.move_focused_by_days(Calendar::days_to_week_start(focused, week_starts_on));
                assert_eq!(
                    moved.focused_date.get(),
                    start,
                    "{focused} starting {week_starts_on:?}"
                );

                let mut moved = calendar(focused, week_starts_on);
                moved.move_focused_by_days(Calendar::days_to_week_end(focused, week_starts_on));
                assert_eq!(moved.focused_date.get(), end, "{focused} starting {week_starts_on:?}");
            }
        }
    }

    #[test]
    fn week_start_tokens_match_weekday_names_case_insensitively() {
        assert_eq!(Calendar::weekday_from_token(" Sunday "), Some(Weekday::Sunday));
        assert_eq!(Calendar::weekday_from_token("monday"), Some(Weekday::Monday));
        assert_eq!(Calendar::weekday_from_token("someday"), None);
        assert_eq!(Calendar::weekday_from_monday_index(8), Weekday::Tuesday);
    }
}

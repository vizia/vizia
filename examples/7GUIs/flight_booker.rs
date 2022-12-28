use std::str::FromStr;
use vizia::prelude::*;

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

use chrono::{NaiveDate, ParseError};

const STYLE: &str = r#"
    
    /*
    * {
        border-width: 1px;
        border-color: red;
    }
    */
    


    textbox.invalid {
        background-color: #AA0000;
    }
"#;

#[derive(Clone)]
pub struct SimpleDate(NaiveDate);

impl Data for SimpleDate {
    fn same(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::fmt::Display for SimpleDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y:%m:%d"))
    }
}

impl FromStr for SimpleDate {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NaiveDate::parse_from_str(s, "%Y:%m:%d").map(|date| SimpleDate(date))
    }
}

#[derive(Lens)]
pub struct AppData {
    options: Vec<&'static str>,
    selected_option: usize,
    start_date: SimpleDate,
    end_date: SimpleDate,
}

pub enum AppEvent {
    SetChoice(usize),
    SetStartDate(SimpleDate),
    SetEndDate(SimpleDate),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetChoice(choice) => {
                self.selected_option = *choice;
            }

            AppEvent::SetStartDate(date) => {
                self.start_date = date.clone();
            }

            AppEvent::SetEndDate(date) => {
                self.end_date = date.clone();
            }
        });
    }
}

impl AppData {
    pub fn new() -> Self {
        Self {
            options: vec!["one-way flight", "return flight"],
            // choice: "one-way flight".to_string(),
            selected_option: 0,
            start_date: SimpleDate(NaiveDate::from_ymd_opt(2022, 02, 12).unwrap()),
            end_date: SimpleDate(NaiveDate::from_ymd_opt(2022, 02, 26).unwrap()),
        }
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");
        cx.add_theme(STYLE);

        AppData::new().build(cx);
        VStack::new(cx, |cx| {
            PickList::new(cx, AppData::options, AppData::selected_option, true)
                .on_select(|cx, index| cx.emit(AppEvent::SetChoice(index)))
                .width(Pixels(150.0));

            Textbox::new(cx, AppData::start_date)
                .on_edit(|cx, text| {
                    if let Ok(val) = text.parse::<SimpleDate>() {
                        cx.emit(AppEvent::SetStartDate(val));
                        cx.toggle_class("invalid", false);
                    } else {
                        cx.toggle_class("invalid", true);
                    }
                })
                .width(Pixels(150.0));

            Textbox::new(cx, AppData::end_date)
                .on_edit(|cx, text| {
                    if let Ok(val) = text.parse::<SimpleDate>() {
                        cx.emit(AppEvent::SetEndDate(val));
                        cx.toggle_class("invalid", false);
                    } else {
                        cx.toggle_class("invalid", true);
                    }
                })
                .width(Pixels(150.0))
                .disabled(AppData::selected_option.map(|choice| *choice == 0));

            Button::new(cx, |_| {}, |cx| Label::new(cx, "Book").width(Stretch(1.0)))
                .width(Pixels(150.0));
        })
        .row_between(Pixels(10.0))
        .child_space(Stretch(1.0));
    })
    .title("Flight Booker")
    .inner_size((250, 250))
    .ignore_default_theme()
    .run();
}

use std::str::FromStr;

use vizia::fonts::icons_names::DOWN;
use vizia::prelude::*;

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
    choice: String,
    start_date: SimpleDate,
    end_date: SimpleDate,
}

pub enum AppEvent {
    SetChoice(String),
    SetStartDate(SimpleDate),
    SetEndDate(SimpleDate),
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetChoice(choice) => {
                self.choice = choice.clone();
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
            choice: "one-way flight".to_string(),
            start_date: SimpleDate(NaiveDate::from_ymd(2022, 02, 12)),
            end_date: SimpleDate(NaiveDate::from_ymd(2022, 02, 26)),
        }
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);
        AppData::new().build(cx);
        VStack::new(cx, |cx| {
            Dropdown::new(
                cx,
                move |cx|
                // A Label and an Icon
                HStack::new(cx, move |cx|{
                    Label::new(cx, AppData::choice)
                        .width(Stretch(1.0))
                        .text_wrap(false);
                    Label::new(cx, DOWN).font("icons").left(Pixels(5.0)).right(Pixels(5.0));
                }).width(Stretch(1.0)),
                // List of options
                move |cx| {
                    List::new(cx, AppData::options, |cx, _, item| {
                        Label::new(cx, item)
                            .width(Stretch(1.0))
                            .child_top(Stretch(1.0))
                            .child_bottom(Stretch(1.0))
                            .bind(AppData::choice, move |handle, choice| {
                                let selected = item.get(handle.cx) == choice.get(handle.cx);
                                handle.background_color(if selected {
                                    Color::from("#f8ac14")
                                } else {
                                    Color::white()
                                });
                            })
                            .on_press(move |cx| {
                                cx.emit(AppEvent::SetChoice(item.get(cx).to_string().to_owned()));
                                cx.emit(PopupEvent::Close);
                            });
                    });
                },
            )
            .width(Pixels(150.0));

            Textbox::new(cx, AppData::start_date)
                .on_edit(|cx, text| {
                    if let Ok(val) = text.parse::<SimpleDate>() {
                        cx.emit(AppEvent::SetStartDate(val));
                        cx.current.toggle_class(cx, "invalid", false);
                    } else {
                        cx.current.toggle_class(cx, "invalid", true);
                    }
                })
                .width(Pixels(150.0));

            Textbox::new(cx, AppData::end_date)
                .on_edit(|cx, text| {
                    if let Ok(val) = text.parse::<SimpleDate>() {
                        cx.emit(AppEvent::SetEndDate(val));
                        cx.current.toggle_class(cx, "invalid", false);
                    } else {
                        cx.current.toggle_class(cx, "invalid", true);
                    }
                })
                .width(Pixels(150.0))
                .disabled(AppData::choice.map(|choice| choice == "one-way flight"));

            Button::new(cx, |_| {}, |cx| Label::new(cx, "Book").width(Stretch(1.0)))
                .width(Pixels(150.0));
        })
        .row_between(Pixels(10.0))
        .child_space(Stretch(1.0));
    })
    .title("Flight Booker")
    .inner_size((250, 250))
    .run();
}

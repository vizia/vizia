use std::str::FromStr;

use vizia::*;

use chrono::{NaiveDate, ParseError};

const ICON_DOWN_OPEN: &str = "\u{e75c}";

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
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetChoice(choice) => {
                    self.choice = choice.clone();
                }

                AppEvent::SetStartDate(date) => {
                    self.start_date = date.clone();
                }
            }
        }
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
    let window_description =
        WindowDescription::new().with_title("Flight Booker").with_inner_size(250, 250);
    Application::new(window_description, |cx|{
        AppData::new().build(cx);
        VStack::new(cx, |cx|{
            Dropdown::new(cx, move |cx|
                // A Label and an Icon
                HStack::new(cx, move |cx|{
                    Binding::new(cx, AppData::choice, |cx, choice|{
                        Label::new(cx, choice)
                            .width(Stretch(1.0));
                    });
                    Label::new(cx, ICON_DOWN_OPEN).font("icons").left(Pixels(5.0)).right(Pixels(5.0));
                }).width(Stretch(1.0)),
                move |cx|{
                List::new(cx, AppData::options, |cx, _, item|{
                    VStack::new(cx, move |cx|{
                        Binding::new(cx, AppData::choice, move |cx, choice|{
                            let selected = *item.get(cx) == *choice.get(cx);
                            let item = item.clone();
                            Label::new(cx, item.clone())
                                .width(Stretch(1.0))
                                .background_color(if selected {Color::from("#f8ac14")} else {Color::white()})
                                .on_press(move |cx| {
                                    cx.emit(AppEvent::SetChoice(item.get(cx).to_string().to_owned()));
                                    cx.emit(PopupEvent::Close);
                                });
                        });
                    }).height(Auto);
                });
            }).width(Pixels(150.0));

            Textbox::new(cx, AppData::start_date)
                .on_edit(|cx, text|{
                    if let Ok(val) = text.parse::<SimpleDate>() {
                        cx.emit(AppEvent::SetStartDate(val));
                    }
                })
                .width(Pixels(150.0));

            Binding::new(cx, AppData::choice, |cx, choice|{
                let disabled = *choice.get(cx) == "one-way flight";
                Textbox::new(cx, AppData::end_date)
                    .width(Pixels(150.0))
                    .disabled(disabled);
            });

            Button::new(cx, |_|{}, |cx|{
                Label::new(cx, "Book")
                    .width(Stretch(1.0))
            })
            .width(Pixels(150.0));
        })
        .row_between(Pixels(10.0))
        .child_space(Stretch(1.0));
    }).run();
}

use chrono::NaiveDate;
use vizia::prelude::*;

const STYLE: &str = r#"
    .container {
        child-space: 1s;
        row-between: 10px;
    }

    .container > * {
        width: 150px;
    }

    textbox:invalid {
        background-color: #AA0000;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    options: Vec<&'static str>,
    selected_option: usize,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

pub enum AppEvent {
    SetChoice(usize),
    SetStartDate(NaiveDate),
    SetEndDate(NaiveDate),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetChoice(choice) => self.selected_option = *choice,
            AppEvent::SetStartDate(date) => self.start_date = date.clone(),
            AppEvent::SetEndDate(date) => self.end_date = date.clone(),
        });
    }
}

fn input_box<L: Lens<Target = NaiveDate>>(
    cx: &mut Context,
    date_lens: L,
    message: impl Fn(NaiveDate) -> AppEvent + Send + Sync + 'static,
) {
    Textbox::new(cx, date_lens.map(|date| format!("{}", date.format("%Y:%m:%d"))))
        .validate(|text| NaiveDate::parse_from_str(&text, "%Y:%m:%d").is_ok())
        .on_submit(move |ex, text, _| {
            if let Ok(val) = NaiveDate::parse_from_str(&text, "%Y:%m:%d") {
                ex.emit(message(val));
            }
        })
        .class("input");
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        AppData {
            options: vec!["one-way flight", "return flight"],
            selected_option: 0,
            start_date: NaiveDate::from_ymd_opt(2022, 02, 12).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2022, 02, 26).unwrap(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            PickList::new(cx, AppData::options, AppData::selected_option, true)
                .on_select(|cx, index| cx.emit(AppEvent::SetChoice(index)));

            input_box(cx, AppData::start_date, AppEvent::SetStartDate);
            input_box(cx, AppData::end_date, AppEvent::SetEndDate);

            Button::new(cx, |_| {}, |cx| Label::new(cx, "Book"));
        })
        .class("container");
    })
    .title("Flight Booker")
    .inner_size((250, 250))
    .run();
}

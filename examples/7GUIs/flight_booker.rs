use chrono::NaiveDate;
use vizia::prelude::*;

const STYLE: &str = r#"
    .container {
        padding: 1s;
        vertical-gap: 10px;
    }

    .container > * {
        width: 150px;
    }

    textbox:invalid {
        background-color: #AA0000;
    }
"#;

pub struct AppData {
    selected_option: Signal<usize>,
    start_date: Signal<NaiveDate>,
    end_date: Signal<NaiveDate>,
}

pub enum AppEvent {
    SetChoice(usize),
    SetStartDate(NaiveDate),
    SetEndDate(NaiveDate),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetChoice(choice) => self.selected_option.set(*choice),
            AppEvent::SetStartDate(date) => self.start_date.set(*date),
            AppEvent::SetEndDate(date) => self.end_date.set(*date),
        });
    }
}

fn input_box<R>(
    cx: &mut Context,
    date_value: R,
    message: impl Fn(NaiveDate) -> AppEvent + Send + Sync + 'static,
) where
    R: Res<String> + Clone + 'static,
{
    Textbox::new(cx, date_value)
        .validate(|text| NaiveDate::parse_from_str(text, "%Y:%m:%d").is_ok())
        .on_submit(move |ex, text, _| {
            if let Ok(val) = NaiveDate::parse_from_str(&text, "%Y:%m:%d") {
                ex.emit(message(val));
            }
        })
        .class("input");
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let options = Signal::new(["one-way flight", "return flight"].map(Signal::new).to_vec());
        let selected_option = Signal::new(0usize);
        let start_date = Signal::new(NaiveDate::from_ymd_opt(2022, 2, 12).unwrap());
        let end_date = Signal::new(NaiveDate::from_ymd_opt(2022, 2, 26).unwrap());

        let start_date_text =
            Memo::new(move |_| format!("{}", start_date.get().format("%Y:%m:%d")));
        let end_date_text = Memo::new(move |_| format!("{}", end_date.get().format("%Y:%m:%d")));

        AppData { selected_option, start_date, end_date }.build(cx);

        VStack::new(cx, |cx| {
            PickList::new(cx, options, selected_option, true)
                .on_select(|cx, index| cx.emit(AppEvent::SetChoice(index)));

            input_box(cx, start_date_text, AppEvent::SetStartDate);
            input_box(cx, end_date_text, AppEvent::SetEndDate);

            Button::new(cx, |cx| Label::new(cx, "Book"));
        })
        .class("container");
    })
    .title("Flight Booker")
    .inner_size((250, 250))
    .run()
}

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

struct FlightBookerApp {
    options: Signal<Vec<&'static str>>,
    selected_option: Signal<usize>,
    start_date: Signal<NaiveDate>,
    end_date: Signal<NaiveDate>,
}

impl App for FlightBookerApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            options: cx.state(vec!["one-way flight", "return flight"]),
            selected_option: cx.state(0usize),
            start_date: cx.state(NaiveDate::from_ymd_opt(2022, 2, 12).unwrap()),
            end_date: cx.state(NaiveDate::from_ymd_opt(2022, 2, 26).unwrap()),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let options = self.options;
        let selected_option = self.selected_option;
        let start_date = self.start_date;
        let end_date = self.end_date;

        let start_text = start_date.drv(cx, |v, _| v.format("%Y:%m:%d").to_string());
        let end_text = end_date.drv(cx, |v, _| v.format("%Y:%m:%d").to_string());

        VStack::new(cx, move |cx| {
            PickList::new(cx, options, selected_option, true)
                .on_select(move |cx, index| selected_option.set(cx, index));

            Textbox::new(cx, start_text)
                .validate(|text| NaiveDate::parse_from_str(text, "%Y:%m:%d").is_ok())
                .on_submit(move |cx, text, _| {
                    if let Ok(val) = NaiveDate::parse_from_str(&text, "%Y:%m:%d") {
                        start_date.set(cx, val);
                    }
                })
                .class("input");

            Textbox::new(cx, end_text)
                .validate(|text| NaiveDate::parse_from_str(text, "%Y:%m:%d").is_ok())
                .on_submit(move |cx, text, _| {
                    if let Ok(val) = NaiveDate::parse_from_str(&text, "%Y:%m:%d") {
                        end_date.set(cx, val);
                    }
                })
                .class("input");

            Button::new(cx, |cx| Label::new(cx, "Book"));
        })
        .class("container");

        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Flight Booker").inner_size((250, 250)))
    }
}

fn main() -> Result<(), ApplicationError> {
    FlightBookerApp::run()
}

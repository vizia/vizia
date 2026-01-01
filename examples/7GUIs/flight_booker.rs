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

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let options = cx.state(vec!["one-way flight", "return flight"]);
        let selected_option = cx.state(0usize);
        let start_date = cx.state(NaiveDate::from_ymd_opt(2022, 2, 12).unwrap());
        let end_date = cx.state(NaiveDate::from_ymd_opt(2022, 2, 26).unwrap());
        let title = cx.state("Flight Booker".to_string());
        let size = cx.state((250, 250));

        // Derived signals for formatted dates
        let start_text = cx.derived({
            let start_date = start_date;
            move |s| start_date.get(s).format("%Y:%m:%d").to_string()
        });
        let end_text = cx.derived({
            let end_date = end_date;
            move |s| end_date.get(s).format("%Y:%m:%d").to_string()
        });

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

            Button::new(cx, |cx| Label::static_text(cx, "Book"));
        })
        .class("container");
        (title, size)
    });

    app.title(title).inner_size(size).run()
}

mod helpers;
use chrono::Utc;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        let date = cx.state(Utc::now().date_naive());

        ExamplePage::new(cx, |cx| {
            Datepicker::new(cx, date).on_select(move |cx, selected| date.set(cx, selected));
        });
        cx.state("Datepicker")
    });

    app.title(title).run()
}

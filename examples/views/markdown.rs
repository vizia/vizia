use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        ScrollView::new(cx, |cx| {
            Markdown::new(
                cx,
                "
| Month    | Savings |
| -------- | ------- |
| January  | $250    |
| February | $80     |
| March    | $420    |
",
            );
        });
        cx.state("Markdown")
    });

    app.title(title).run()
}

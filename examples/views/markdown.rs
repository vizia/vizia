use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        //         Markdown::new(
        //             cx,
        //             r#"
        // # This is a heading

        // ## This is a subheading

        // This is **some strong text**. ~~And some more text~~.

        // - List A
        //   - List A1
        //     - Item
        //   - List A2
        //     - Item with *emphasis*

        // ```
        // fn some_code() {

        // }
        // ```

        // A [link](https://github.com/vizia/vizia)
        // "#,
        //         );
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
    })
    .title(Localized::new("view-title-markdown"))
    .run()
}

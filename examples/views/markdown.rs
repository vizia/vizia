use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {}

impl Model for AppData {}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData {}.build(cx);

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
                r#"
some `code` inline
"#,
            );
        });
    })
    .title("Markdown")
    .run()
}

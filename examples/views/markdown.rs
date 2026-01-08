use vizia::prelude::*;

struct MarkdownApp;

impl App for MarkdownApp {
    fn new(_cx: &mut Context) -> Self {
        Self
    }

    fn on_build(self, cx: &mut Context) -> Self {
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
        self
    }
}

fn main() -> Result<(), ApplicationError> {
    MarkdownApp::run()
}

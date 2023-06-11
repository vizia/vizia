mod helpers;
pub use helpers::*;
use vizia::prelude::*;

const STYLE: &str = r#"
    element {
        background-color: rgb(100, 100, 100);
        size: 100px;
        child-space: 1s;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        ExamplePage::new(cx, |cx| {
            Element::new(cx).text("Hover Me").tooltip(|cx| {
                Label::new(cx, "Basic Tooltip");
            });
        });
    })
    .title("Tooltip")
    .run();
}

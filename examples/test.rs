use vizia::prelude::*;

const STYLE: &str = r#"
    label {
        font-size: 10;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);

        cx.style().font_size.insert(Entity::root(), 15.0);

        VStack::new(cx, |cx| {
            Label::new(cx, "one");
            Label::new(cx, "two");
            Label::new(cx, "three");
            Label::new(cx, "four");
            Label::new(cx, "five");
            Label::new(cx, "six");
        })
        .space(Stretch(1.0))
        .row_between(Pixels(10.0));
    })
    .title("Test")
    .run();
}

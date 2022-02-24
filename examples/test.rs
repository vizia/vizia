use vizia::*;

const STYLE: &str = r#"
    .test {
        display: contents;
    }
"#;

fn main() {
    Application::new(WindowDescription::new().with_title("Test"), |cx| {
        cx.add_theme(STYLE);

        HStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                HStack::new(cx, |_| {}).size(Stretch(1.0)).background_color(Color::blue());
            })
            .class("test")
            //.display(Display::Contents)
            .size(Pixels(80.0))
            .background_color(Color::green());
        })
        .size(Pixels(100.0))
        .background_color(Color::red());
    })
    .run();
}

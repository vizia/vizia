use vizia::*;

const STYLE: &str = r#"
    .test {
        display: contents;
    }
"#;

fn main() {
    let mut window_description = WindowDescription::new();
    window_description.resizable = false;
    Application::new(window_description, |cx| {
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

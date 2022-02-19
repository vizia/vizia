use vizia::*;

const STYLE: &str = r#"
    .test {
        width: 100px;
        height: 100px;
        background-color: red;
        transition: background-color 1.0 0.0;
    }

    .test:hover {
        background-color: blue;
        transition: background-color 1.0 0.0;
    }
"#;

fn main() {
    let window_description = WindowDescription::new();
    Application::new(window_description, |cx| {
        cx.add_theme(STYLE);

        Element::new(cx).class("test");

        Checkbox::new(cx, false);
    })
    .run();
}

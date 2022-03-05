use vizia::*;

const STYLE: &str = r#"
    #lbl1 {
        background-color: red;
    }
"#;

fn main() {
    let mut window_description = WindowDescription::new();
    window_description.resizable = false;
    Application::new(window_description, |cx| {
        cx.add_theme(STYLE);

        Label::new(cx, "Label 1").id("lbl1");
        Label::new(cx, "Label 2").id("lbl2");
        Label::new(cx, "Label 3").id("lbl3");
    })
    .run();
}

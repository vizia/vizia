use vizia::prelude::*;

const STYLE: &str = r#"
    .font_size {
        font-size: x-large;
    }

    .font_color {
        color: red;
    }

    .font_weight {
        font-weight: bold;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);
        Label::new(cx, "Font Size").class("font_size");
        Label::new(cx, "Font Color").class("font_color");
        Label::new(cx, "Font Weight").class("font_weight");
    })
    .run();
}

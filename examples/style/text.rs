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

    .font_style {
        font-style: italic;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);
        Label::new(cx, "Font Size").class("font_size");
        Label::new(cx, "Font Color").class("font_color");
        Label::new(cx, "Font Weight").class("font_weight");
        Label::new(cx, "Font Style").class("font_style");
        Label::new(cx, "Font Family").class("font_family");
        Label::new(cx, "Text Wrap").class("text_wrap");
    })
    .run();
}

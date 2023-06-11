use vizia::prelude::*;

const STYLE: &str = r#"
    .font_size {
        font-size: xx-large;
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


    .font_stretch {
        font-stretch: ultra-condensed;
    }

    .caret_color:checked .textbox_content {
        caret-color: #00FF00;
        selection-color: #c8646488;
    }

"#;

#[derive(Lens, Setter, Model)]
pub struct AppData {
    text: String,
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        AppData { text: "This text is editable!".to_string() }.build(cx);

        Label::new(cx, "Font Size").class("font_size");
        Label::new(cx, "Font Color").class("font_color");
        Label::new(cx, "Font Weight").class("font_weight");
        Label::new(cx, "Font Style").class("font_style");
        Label::new(cx, "Font Stretch").class("font_stretch");
        Textbox::new(cx, AppData::text)
            .on_edit(|cx, text| cx.emit(AppDataSetter::Text(text)))
            .width(Pixels(200.0))
            .class("caret_color");
    })
    .run();
}

use vizia::prelude::*;

const STYLE: &str = r#"
    .example_1 {
        text-stroke-width: 2px;
    }

    .example_3 {
        text-stroke-width: 2px;
        text-stroke-style: stroke;
    }

    .example_5 {
        text-stroke: 2px stroke;
    }
"#;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        VStack::new(cx, |cx| {
            Label::new(cx, "This is the standard text.").class("example_0");

            Label::new(cx, "This is text with a 2px stroke in CSS").class("example_1");
            Label::new(cx, "This is text with a 2px stroke in code")
                .class("example_2")
                .text_stroke_width(Pixels(2.0));

            Label::new(cx, "This is text that is only a 2px stroke in CSS").class("example_3");
            Label::new(cx, "This is text that is only a 2px stroke in code")
                .class("example_4")
                .text_stroke_width(Pixels(2.0))
                .text_stroke_style(TextStrokeStyle::Stroke);

            Label::new(cx, "This is text that is only a 2px stroke in CSS using the shorthand")
                .class("example_5");
        })
        .font_size(40.0)
        .row_between(Pixels(10.0))
        .child_space(Pixels(10.0));
    })
    .run()
}

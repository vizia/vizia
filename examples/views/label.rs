use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    text: String,
    value: f32,
}

impl Model for AppData {}

fn main() {
    Application::new(|cx| {
        AppData {
            text: String::from("As well as model data which implements ToString:"),
            value: 3.141592,
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "A label can display a static string of text.");

            Label::new(cx, AppData::text);

            Label::new(cx, AppData::value);

            Label::new(cx, "Text which is too long for the label will be wrapped.")
                .width(Pixels(200.0));

            Label::new(cx, "Unless text wrapping is disabled.")
                .width(Pixels(200.0))
                .text_wrap(false);
        })
        .child_space(Stretch(1.0))
        .row_between(Pixels(20.0));
    })
    .title("Label")
    .run();
}

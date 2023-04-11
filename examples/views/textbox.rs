use vizia::prelude::*;

#[derive(Lens, Setter, Model)]
pub struct AppData {
    text: String,
}

fn main() {
    Application::new(|cx| {
        AppData { text: "This is some text that spans multiple lines because it's really long and doesn't fit within the bounds of the textbox.\nThis is some text that spans multiple lines because it's really long and doesn't fit within the bounds of the textbox.".to_string() }.build(cx);

        Textbox::new(cx, AppData::text)
            // .text_wrap(true)
            .width(Pixels(100.0))
            .height(Pixels(200.0))
            // .height(Auto)
            // .size(Auto)
            // .top(Pixels(50.0))
            // .left(Pixels(50.0))
            .on_edit(|cx, text| cx.emit(AppDataSetter::Text(text)))
            .on_build(|cx| {
                cx.emit(TextEvent::StartEdit);
            });

        Textbox::new_multiline(
            cx,
            StaticLens::new(
                &"This text is editable, but will reset on blur. Good luck editing it, haha!",
            ),
            true,
        )
        .width(Pixels(200.0))
        .height(Pixels(200.0));
    })
    .title("Textbox")
    .run();
}

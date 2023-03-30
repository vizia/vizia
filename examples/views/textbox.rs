use vizia::prelude::*;
use vizia_core::state::StaticLens;

#[derive(Lens, Setter, Model)]
pub struct AppData {
    text: String,
}

fn main() {
    Application::new(|cx| {
        AppData { text: "This is some text which is too long".to_string() }.build(cx);

        Textbox::new(cx, AppData::text)
            .width(Pixels(100.0))
            // .height(Pixels(30.0))
            .on_edit(|cx, text| cx.emit(AppDataSetter::Text(text)))
            .on_build(|cx| {
                cx.emit(TextEvent::StartEdit);
            });

        // Textbox::new_multiline(
        //     cx,
        //     StaticLens::new(
        //         &"This text is editable, but will reset on blur. Good luck editing it, haha!",
        //     ),
        //     true,
        // )
        // .width(Pixels(200.0))
        // .height(Pixels(200.0));
    })
    .title("Textbox")
    .run();
}

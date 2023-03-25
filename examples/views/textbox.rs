use vizia::prelude::*;
use vizia_core::state::StaticLens;

#[derive(Lens, Setter, Model)]
pub struct AppData {
    text: String,
}

fn main() {
    Application::new(|cx| {
        AppData {
            text: "This is some text that spans four lines\nthanks to a newline and wrapping."
                .to_string(),
        }
        .build(cx);

        Textbox::new_multiline(cx, AppData::text, true)
            .on_edit(|cx, text| cx.emit(AppDataSetter::Text(text)))
            .width(Pixels(160.0))
            .height(Pixels(100.0))
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

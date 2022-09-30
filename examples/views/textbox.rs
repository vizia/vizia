use vizia::prelude::*;

#[derive(Lens, Setter, Model)]
pub struct AppData {
    text: String,
}

fn main() {
    Application::new(|cx| {
        AppData { text: "This text is editable!".to_string() }.build(cx);

        Textbox::new(cx, AppData::text)
            .on_edit(|cx, text| cx.emit(AppDataSetter::Text(text)))
            .width(Pixels(200.0))
            .on_build(|cx| {
                cx.emit(TextEvent::StartEdit);
            });
    })
    .title("Textbox")
    .run();
}

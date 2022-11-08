use vizia::prelude::*;

#[derive(Lens, Setter, Model)]
pub struct AppData {
    text: String,
}

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        AppData { text: "This text is editable!".to_string() }.build(cx);

        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        VStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .on_edit(|cx, text| cx.emit(AppDataSetter::Text(text)))
                .width(Pixels(200.0))
                .on_build(|cx| {
                    cx.emit(TextEvent::StartEdit);
                });

            Textbox::new(cx, AppData::text)
                .on_edit(|cx, text| cx.emit(AppDataSetter::Text(text)))
                .width(Pixels(200.0))
                .on_build(|cx| {
                    cx.emit(TextEvent::StartEdit);
                })
                .disabled(true);

            Textbox::new(cx, AppData::text)
                .on_edit(|cx, text| cx.emit(AppDataSetter::Text(text)))
                .width(Pixels(200.0))
                .on_build(|cx| {
                    cx.emit(TextEvent::StartEdit);
                })
                .class("error");
        })
        .class("container");
    })
    .ignore_default_theme()
    .title("Textbox")
    .run();
}

use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    text: String,
}

#[derive(Debug)]
pub enum AppEvent {
    SetText(String),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetText(text) => {
                self.text = text.clone();
            }
        });
    }
}

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        AppData { text: "This text is editable!".to_string() }.build(cx);

        cx.add_stylesheet(LIGHT_THEME).expect("Failed to find stylesheet");

        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Textbox::new(cx, AppData::text)
                    .on_edit(|cx, text| cx.emit(AppEvent::SetText(text)))
                    .width(Pixels(200.0))
                    .on_build(|cx| {
                        cx.emit(TextEvent::StartEdit);
                    });

                Textbox::new(cx, AppData::text)
                    .on_edit(|cx, text| cx.emit(AppEvent::SetText(text)))
                    .width(Pixels(200.0))
                    .on_build(|cx| {
                        cx.emit(TextEvent::StartEdit);
                    })
                    .disabled(true);

                Textbox::new(cx, AppData::text)
                    .on_edit(|cx, text| cx.emit(AppEvent::SetText(text)))
                    .width(Pixels(200.0))
                    .on_build(|cx| {
                        cx.emit(TextEvent::StartEdit);
                    })
                    .class("error");
            })
            .class("container");
        })
        .class("main");
    })
    .ignore_default_theme()
    .title("Textbox")
    .run();
}

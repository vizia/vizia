use vizia::prelude::*;

#[derive(Lens, Ray)]
pub struct AppData {
    text: String,
}

#[derive(Debug)]
pub enum AppEvent {
    SetText(String),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.take::<AppDataRay, _>(|mut app_event, _| app_event.strike(self));
    }
}

fn main() {
    Application::new(|cx| {
        AppData { text: "This text is editable!".to_string() }.build(cx);

        Textbox::new(cx, AppData::text)
            .on_edit(|cx, text| cx.emit(AppEvent::SetText(text)))
            .width(Pixels(200.0))
            .on_build(|cx| {
                cx.emit(TextEvent::StartEdit);
            });
    })
    .title("Textbox")
    .run();
}

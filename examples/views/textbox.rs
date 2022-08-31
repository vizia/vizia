use vizia::prelude::*;

#[derive(Lens, Ray)]
pub struct AppData {
    text: String,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.take::<AppDataRay>().map(|ray| ray.apply(self));
    }
}

fn main() {
    Application::new(|cx| {
        AppData { text: "This text is editable!".to_string() }.build(cx);

        Textbox::new(cx, AppData::text)
            .on_edit(|cx, text| cx.emit(AppDataRay::Text(text)))
            .width(Pixels(200.0))
            .on_build(|cx| {
                cx.emit(TextEvent::StartEdit);
            });
    })
    .title("Textbox")
    .run();
}

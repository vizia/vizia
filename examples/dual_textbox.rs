use vizia::*;

#[derive(Lens)]
pub struct AppData {
    text: String,
}

pub enum AppEvent {
    SetText(String),
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetText(text) => {
                    self.text = text.clone();
                }
            }
        }
    }
}

fn main() {
    let window_description = WindowDescription::new().with_title("Textbox");
    Application::new(window_description, |cx| {
        AppData { text: "This text is editable!".to_string() }.build(cx);

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .on_edit(|cx, text| cx.emit(AppEvent::SetText(text)))
                .width(Pixels(200.0))
                .child_left(Pixels(5.0));

            
            Textbox::new(cx, AppData::text)
                .on_edit(|cx, text| cx.emit(AppEvent::SetText(text)))
                .width(Pixels(200.0))
                .child_left(Pixels(5.0));

            Binding::new(cx, AppData::text, |cx, text| {
                Label::new(cx, &text.get(cx).clone())
                    .width(Pixels(200.0))
                    .height(Pixels(30.0))
                    .child_left(Pixels(5.0));
            });
            
        })
        .space(Stretch(1.0))
        .col_between(Pixels(10.0));
    })
    .run();
}
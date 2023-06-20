mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    editable_text: String,
    multiline_text: String,
    non_editable_text: String,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetEditableText(text) => self.editable_text = text.clone(),
            AppEvent::SetMultilineText(text) => self.multiline_text = text.clone(),
        });
    }
}

pub enum AppEvent {
    SetEditableText(String),
    SetMultilineText(String),
}

fn main() {
    Application::new(|cx| {
        AppData {
            editable_text: "This is some editable text".to_string(),
            multiline_text: "This is some text which is editable and spans multiple lines"
                .to_string(),
            non_editable_text: "This text can be selected but not edited".to_string(),
        }
        .build(cx);

        ExamplePage::vertical(cx, |cx| {
            Textbox::new(cx, AppData::editable_text)
                .width(Pixels(300.0))
                .on_edit(|cx, text| cx.emit(AppEvent::SetEditableText(text)));
            Textbox::new_multiline(cx, AppData::multiline_text, true)
                .width(Pixels(300.0))
                .height(Pixels(300.0))
                .on_edit(|cx, text| cx.emit(AppEvent::SetMultilineText(text)));
            Textbox::new(cx, AppData::non_editable_text).width(Pixels(300.0)).read_only(true);
        });
    })
    .title("Textbox")
    .run();
}

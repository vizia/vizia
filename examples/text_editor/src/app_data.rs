use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    pub text: String,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetText(text) => self.text = text.clone(),
        })
    }
}

pub enum AppEvent {
    SetText(String),
}

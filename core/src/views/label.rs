use crate::{Context, Handle, LocalizedStringKey, View};

pub struct Label;

impl Label {
    pub fn new<'a>(cx: &mut Context, text: impl LocalizedStringKey<'a>) -> Handle<Self> {
        Self {}.build2(cx, |_| {}).text(text.key())
    }
}

impl View for Label {
    fn element(&self) -> Option<String> {
        Some("label".to_string())
    }
}

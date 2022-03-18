use crate::{Context, Handle, Res, View};

pub struct Label;

impl Label {
    pub fn new<T>(cx: &mut Context, text: impl Res<T>) -> Handle<Self>
    where
        T: ToString,
    {
        Self {}.build2(cx, |_| {}).text(text)
    }
}

impl View for Label {
    fn element(&self) -> Option<String> {
        Some("label".to_string())
    }
}

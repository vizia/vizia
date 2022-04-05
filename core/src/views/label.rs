use crate::{Context, Handle, Res, View};

pub struct Label;

impl Label {
    pub fn new<'a, T>(cx: &'a mut Context, text: impl Res<T>) -> Handle<'a, Self>
    where
        T: ToString,
    {
        Self {}.build(cx, |_| {}).text(text)
    }
}

impl View for Label {
    fn element(&self) -> Option<String> {
        Some("label".to_string())
    }
}

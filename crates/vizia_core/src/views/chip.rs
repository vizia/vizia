use crate::prelude::*;

pub struct Chip;

impl Chip {
    pub fn new<'a, T>(cx: &'a mut Context, text: impl Res<T>) -> Handle<'a, Self>
    where
        T: ToString,
    {
        Self {}.build(cx, move |cx| {
            Label::new(cx, text);
        })
    }
}

impl View for Chip {
    fn element(&self) -> Option<&'static str> {
        Some("chip")
    }
}

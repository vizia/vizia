use crate::prelude::*;

pub struct Chip;

impl Chip {
    pub fn new<T: Lens>(cx: &mut Context, text: T) -> Handle<Self>
    where
        <T as Lens>::Target: Data + ToString,
    {
        Self {}.build(cx, move |_| {}).text(text)
    }
}

impl View for Chip {
    fn element(&self) -> Option<&'static str> {
        Some("chip")
    }
}

use crate::prelude::*;

pub struct Avatar {}

impl Avatar {
    pub fn new<F>(cx: &mut Context, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        Self {}.build(cx, content)
    }
}

impl View for Avatar {
    fn element(&self) -> Option<&'static str> {
        Some("avatar")
    }
}

impl<'a> Handle<'a, Avatar> {
    pub fn badge<F>(mut self, content: F) -> Self
    where
        F: FnOnce(&mut Context) -> Handle<'_, Badge>,
    {
        let entity = self.entity();

        self.context().with_current(entity, |cx| {
            (content)(cx);
        });

        self
    }
}

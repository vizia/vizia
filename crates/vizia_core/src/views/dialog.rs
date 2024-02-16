use crate::prelude::*;

pub struct Dialog {}

impl Dialog {
    pub fn new(
        cx: &mut Context,
        is_open: impl Lens<Target = bool>,
        content: impl Fn(&mut Context) + 'static,
    ) -> Handle<Self> {
        Self {}.build(cx, move |cx| {
            Binding::new(cx, is_open, move |cx, is_open| {
                if is_open.get(cx) {
                    (content)(cx);
                }
            })
        })
    }
}

impl View for Dialog {
    fn element(&self) -> Option<&'static str> {
        Some("dialog")
    }
}

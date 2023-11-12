use crate::prelude::*;

pub struct Divider {}

impl Divider {
    pub fn horizontal(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_| {})
    }

    pub fn vertical(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_| {}).class("vertical")
    }
}

impl View for Divider {
    fn element(&self) -> Option<&'static str> {
        Some("divider")
    }
}

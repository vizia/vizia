use crate::prelude::*;

pub struct Divider {}

impl Divider {
    /// Creates a dividing line. Orientation is determined by context (default horizontal).
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_| {})
    }

    /// Creates a horizontal dividing line.
    pub fn horizontal(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_| {}).class("horizontal")
    }

    /// Creates a vertical dividing line.
    pub fn vertical(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_| {}).class("vertical")
    }
}

impl View for Divider {
    fn element(&self) -> Option<&'static str> {
        Some("divider")
    }
}

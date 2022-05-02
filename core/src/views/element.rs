use crate::prelude::*;

/// A basic element with no interactivity.
///
///
pub struct Element {}

impl Element {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_| {})
    }
}

impl View for Element {
    fn element(&self) -> Option<String> {
        Some(String::from("element"))
    }
}

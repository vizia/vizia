use crate::prelude::{Context, Handle, View};
use cosmic_text::Cursor;

#[derive(Copy, Clone)]
pub struct TextSpan {
    pub cursor_start: Cursor,
    pub cursor_end: Cursor,
}

impl TextSpan {
    pub fn new(cx: &mut Context, cursor_start: Cursor, cursor_end: Cursor) -> Handle<Self> {
        Self { cursor_end, cursor_start }.build(cx, |_| {})
    }
}

impl View for TextSpan {}

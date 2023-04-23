use crate::prelude::*;

/// A block containing one or more lines of read-only code.
pub struct CodeBlock {}

impl CodeBlock {
    pub fn new<T: Data + ToString>(cx: &mut Context, lens: impl Lens<Target = T>) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            Textbox::new(cx, lens);
        })
    }
}

impl View for CodeBlock {
    fn element(&self) -> Option<&'static str> {
        Some("codeblock")
    }
}

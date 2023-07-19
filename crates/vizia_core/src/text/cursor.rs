#[derive(Debug, Clone, Copy)]
pub struct TextCursor {
    pub line: usize,
    pub offset: usize,
}

impl TextCursor {
    pub fn new(line: usize, offset: usize) -> Self {
        Self { line, offset }
    }
}

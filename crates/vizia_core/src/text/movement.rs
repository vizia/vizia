#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Upstream,
    Downstream,
}

#[derive(Debug, Clone, Copy)]
pub enum Movement {
    Grapheme(Direction),
    Word(Direction),
    Line(Direction),
    ParagraphStart,
    ParagraphEnd,
    Vertical(VerticalMovement),
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalMovement {
    LineUp,
    LineDown,
    PageUp,
    PageDown,
    DocumentStart,
    DocumentEnd,
}

pub enum Direction {
    Left,
    Right,
    Upstream,
    Downstream,
}

pub enum Movement {
    Grapheme(Direction),
    Word(Direction),
    Line(Direction),
    ParagraphStart,
    ParagraphEnd,
    Vertical(VerticalMovement),
}

pub enum VerticalMovement {
    LineUp,
    LineDown,
    PageUp,
    PageDown,
    DocumentStart,
    DocumentEnd,
}

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
    Page(Direction),
    Body(Direction),
    LineStart,
    LineEnd,
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

use log::warn;

use super::{EditableText, Selection};
use crate::text::shaped_text::ShapedText;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Upstream,
    Downstream,
}

impl Direction {
    /// Returns `true` if this direction is byte-wise backwards for
    /// the provided [`WritingDirection`].
    ///
    /// The provided direction *must not be* `WritingDirection::Natural`.
    pub fn is_upstream_for_direction(self, direction: WritingDirection) -> bool {
        assert!(
            !matches!(direction, WritingDirection::Natural),
            "writing direction must be resolved"
        );
        match self {
            Direction::Upstream => true,
            Direction::Downstream => false,
            Direction::Left => matches!(direction, WritingDirection::LeftToRight),
            Direction::Right => matches!(direction, WritingDirection::RightToLeft),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Movement {
    Grapheme(Direction),
    Word(Direction),
    Line(Direction),
    Page(Direction),
    Body(Direction),
    LineStart,
    LineEnd,
    Vertical(VerticalMovement),
    ParagraphStart,
    ParagraphEnd,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VerticalMovement {
    LineUp,
    LineDown,
    PageUp,
    PageDown,
    DocumentStart,
    DocumentEnd,
}

#[derive(Debug, Clone, Copy)]
pub enum WritingDirection {
    LeftToRight,
    RightToLeft,
    Natural,
}

/// Compute the result of a [`Movement`] on a [`Selection`] against a [`ShapedText`].
///
/// If `modify` is true, only the active edge of the selection changes
/// (i.e. shift is held).
pub fn apply_movement<T: EditableText>(
    m: Movement,
    s: Selection,
    text: &T,
    shaped: &ShapedText,
    modify: bool,
) -> Selection {
    let writing_direction = WritingDirection::LeftToRight;
    let layout = &shaped.pre_shaped.parley_layout;
    let parley_selection = s.to_parley(shaped);

    let (offset, h_pos) = match m {
        Movement::Grapheme(d) if d.is_upstream_for_direction(writing_direction) => {
            let next = parley_selection.previous_visual(layout, modify);
            let sel = Selection::from_parley(next, text.len(), None);
            (sel.active, sel.h_pos)
        }
        Movement::Grapheme(_) => {
            let next = parley_selection.next_visual(layout, modify);
            let sel = Selection::from_parley(next, text.len(), None);
            (sel.active, sel.h_pos)
        }
        Movement::Vertical(VerticalMovement::LineUp) => {
            let next = parley_selection.previous_line(layout, modify);
            let sel = Selection::from_parley(next, text.len(), s.h_pos);
            (sel.active, sel.h_pos)
        }
        Movement::Vertical(VerticalMovement::LineDown) => {
            let next = parley_selection.next_line(layout, modify);
            let sel = Selection::from_parley(next, text.len(), s.h_pos);
            (sel.active, sel.h_pos)
        }
        Movement::Vertical(VerticalMovement::DocumentStart) => (0, None),
        Movement::Vertical(VerticalMovement::DocumentEnd) => (text.len(), None),

        Movement::ParagraphStart => {
            let next = parley_selection.hard_line_start(layout, modify);
            let sel = Selection::from_parley(next, text.len(), None);
            (sel.active, sel.h_pos)
        }
        Movement::ParagraphEnd => {
            let next = parley_selection.hard_line_end(layout, modify);
            let sel = Selection::from_parley(next, text.len(), None);
            (sel.active, sel.h_pos)
        }

        Movement::Line(d) if d.is_upstream_for_direction(writing_direction) => {
            let next = parley_selection.line_start(layout, modify);
            let sel = Selection::from_parley(next, text.len(), None);
            (sel.active, sel.h_pos)
        }
        Movement::Line(_) => {
            let next = parley_selection.line_end(layout, modify);
            let sel = Selection::from_parley(next, text.len(), None);
            (sel.active, sel.h_pos)
        }
        Movement::Word(d) if d.is_upstream_for_direction(writing_direction) => {
            let next = parley_selection.previous_visual_word(layout, modify);
            let sel = Selection::from_parley(next, text.len(), None);
            (sel.active, sel.h_pos)
        }
        Movement::Word(_) => {
            let next = parley_selection.next_visual_word(layout, modify);
            let sel = Selection::from_parley(next, text.len(), None);
            (sel.active, sel.h_pos)
        }
        Movement::Vertical(VerticalMovement::PageDown | VerticalMovement::PageUp) => {
            (s.active, s.h_pos)
        }
        Movement::LineStart => {
            let next = parley_selection.line_start(layout, modify);
            let sel = Selection::from_parley(next, text.len(), None);
            (sel.active, sel.h_pos)
        }
        Movement::LineEnd => {
            let next = parley_selection.line_end(layout, modify);
            let sel = Selection::from_parley(next, text.len(), None);
            (sel.active, sel.h_pos)
        }
        other => {
            warn!("unhandled movement {:?}", other);
            (s.anchor, s.h_pos)
        }
    };

    let start = if modify { s.anchor } else { offset };
    Selection::new(start, offset).with_h_pos(h_pos)
}

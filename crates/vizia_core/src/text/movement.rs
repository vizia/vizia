use log::warn;
use skia_safe::textlayout::Paragraph;

use super::{EditableText, Selection};

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

/// Compute the result of a [`Movement`] on a [`Selection`].
///
/// returns a new selection representing the state after the movement.
///
/// If `modify` is true, only the 'active' edge (the `end`) of the selection
/// should be changed; this is the case when the user moves with the shift
/// key pressed.
pub fn apply_movement<T: EditableText>(
    m: Movement,
    s: Selection,
    text: &T,
    paragraph: &Paragraph,
    modify: bool,
) -> Selection {
    // let writing_direction = if crate::piet::util::first_strong_rtl(text.as_str()) {
    //     WritingDirection::RightToLeft
    // } else {
    //     WritingDirection::LeftToRight
    // };

    let writing_direction = WritingDirection::LeftToRight;

    let (offset, h_pos) = match m {
        Movement::Grapheme(d) if d.is_upstream_for_direction(writing_direction) => {
            if s.is_caret() || modify {
                text.prev_grapheme_offset(s.active).map(|off| (off, None)).unwrap_or((0, s.h_pos))
            } else {
                (s.min(), None)
            }
        }
        Movement::Grapheme(_) => {
            if s.is_caret() || modify {
                text.next_grapheme_offset(s.active)
                    .map(|off| (off, None))
                    .unwrap_or((s.active, s.h_pos))
            } else {
                (s.max(), None)
            }
        }
        Movement::Vertical(VerticalMovement::LineUp) => {
            let cluster = paragraph.get_glyph_cluster_at(s.active).unwrap();
            let glyph_bounds = cluster.bounds;
            let line = paragraph.get_line_number_at(s.active).unwrap();
            let h_pos = s.h_pos.unwrap_or(glyph_bounds.x());
            if line == 0 {
                (0, Some(h_pos))
            } else {
                let lm = paragraph.get_line_metrics_at(line).unwrap();
                let up_pos = paragraph
                    .get_closest_glyph_cluster_at((h_pos, glyph_bounds.y() - lm.height as f32))
                    .unwrap();
                let s = if h_pos < up_pos.bounds.center_x() {
                    up_pos.text_range.start
                } else {
                    up_pos.text_range.end
                };
                // if up_pos.is_inside {
                (s, Some(h_pos))
                // } else {
                //     // because we can't specify affinity, moving up when h_pos
                //     // is wider than both the current line and the previous line
                //     // can result in a cursor position at the visual start of the
                //     // current line; so we handle this as a special-case.
                //     let lm_prev =
                //         paragraph.get_line_metrics_at(line.saturating_sub(1)).unwrap();
                //     let up_pos = lm_prev.end_excluding_whitespaces;
                //     (up_pos, Some(h_pos))
                // }
            }
        }
        Movement::Vertical(VerticalMovement::LineDown) => {
            let cluster = paragraph.get_glyph_cluster_at(s.active).unwrap();
            let h_pos = s.h_pos.unwrap_or(cluster.bounds.x());
            let line = paragraph.get_line_number_at(s.active).unwrap();
            if line == paragraph.line_number() - 1 {
                (text.len(), Some(h_pos))
            } else {
                let lm = paragraph.get_line_metrics_at(line).unwrap();
                // may not work correctly for point sizes below 1.0
                let y_below = lm.baseline - lm.ascent + lm.height + 1.0;
                let down_pos =
                    paragraph.get_closest_glyph_cluster_at((h_pos, y_below as f32)).unwrap();
                let s = if h_pos < down_pos.bounds.center_x() {
                    down_pos.text_range.start
                } else {
                    down_pos.text_range.end
                };
                (s.min(text.len()), Some(h_pos))
            }
        }
        Movement::Vertical(VerticalMovement::DocumentStart) => (0, None),
        Movement::Vertical(VerticalMovement::DocumentEnd) => (text.len(), None),

        Movement::ParagraphStart => (text.preceding_line_break(s.active), None),
        Movement::ParagraphEnd => (text.next_line_break(s.active), None),

        Movement::Line(d) => {
            // let hit = layout.hit_test_text_position(s.active);
            // let lm = layout.line_metric(hit.line).unwrap();
            // let offset = if d.is_upstream_for_direction(writing_direction) {
            //     lm.start_offset
            // } else {
            //     lm.end_offset - lm.trailing_whitespace
            // };
            // (offset, None)
            todo!()
        }
        Movement::Word(d) if d.is_upstream_for_direction(writing_direction) => {
            let offset = if s.is_caret() || modify {
                text.prev_word_offset(s.active).unwrap_or(0)
            } else {
                s.min()
            };
            (offset, None)
        }
        Movement::Word(_) => {
            let offset = if s.is_caret() || modify {
                text.next_word_offset(s.active).unwrap_or(s.active)
            } else {
                s.max()
            };
            (offset, None)
        }

        // These two are not handled; they require knowledge of the size
        // of the viewport.
        Movement::Vertical(VerticalMovement::PageDown)
        | Movement::Vertical(VerticalMovement::PageUp) => (s.active, s.h_pos),

        Movement::LineStart => {
            let line = paragraph.get_line_number_at(s.active).unwrap();
            let lm = paragraph.get_line_metrics_at(line).unwrap();
            (lm.start_index, None)
        }

        Movement::LineEnd => {
            let line = paragraph.get_line_number_at(s.active).unwrap();
            let lm = paragraph.get_line_metrics_at(line).unwrap();
            (lm.end_index - 1, None)
        }

        other => {
            warn!("unhandled movement {:?}", other);
            (s.anchor, s.h_pos)
        }
    };

    let start = if modify { s.anchor } else { offset };
    Selection::new(start, offset).with_h_pos(h_pos)
}

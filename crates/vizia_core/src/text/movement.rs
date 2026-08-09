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
            let Some(cluster) = shaped.get_glyph_cluster_at(s.active) else {
                return Selection::new(if modify { s.anchor } else { 0 }, 0).with_h_pos(s.h_pos);
            };
            let h_pos = s.h_pos.unwrap_or(cluster.bounds.x());
            let line = shaped.get_line_number_at(s.active).unwrap_or(0);
            if line == 0 {
                (0, Some(h_pos))
            } else {
                let lm = shaped.get_line_metrics_at(line).unwrap();
                let up_y = cluster.bounds.y() - lm.height as f32;
                let up_pos =
                    shaped.get_closest_glyph_cluster_at((h_pos, up_y)).unwrap_or(cluster);
                let s = if h_pos < up_pos.center_x() {
                    up_pos.text_range.start
                } else {
                    up_pos.text_range.end
                };
                (s, Some(h_pos))
            }
        }
        Movement::Vertical(VerticalMovement::LineDown) => {
            let Some(cluster) = shaped.get_glyph_cluster_at(s.active) else {
                return Selection::new(if modify { s.anchor } else { text.len() }, text.len())
                    .with_h_pos(s.h_pos);
            };
            let h_pos = s.h_pos.unwrap_or(cluster.bounds.x());
            let line = shaped.get_line_number_at(s.active).unwrap_or(0);
            if line + 1 >= shaped.line_count() {
                (text.len(), Some(h_pos))
            } else {
                let lm = shaped.get_line_metrics_at(line).unwrap();
                let down_y = lm.baseline as f32 - lm.ascent as f32 + lm.height as f32 + 1.0;
                let down_pos =
                    shaped.get_closest_glyph_cluster_at((h_pos, down_y)).unwrap_or(cluster);
                let s = if h_pos < down_pos.center_x() {
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

        Movement::Line(_) => {
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
        Movement::Vertical(VerticalMovement::PageDown | VerticalMovement::PageUp) => {
            (s.active, s.h_pos)
        }
        Movement::LineStart => {
            let line = shaped.get_line_number_at(s.active).unwrap_or(0);
            let lm = shaped
                .get_line_metrics_at(line)
                .or_else(|| shaped.get_line_metrics().into_iter().next())
                .unwrap_or_else(|| return_default_lm());
            (lm.start_index, None)
        }
        Movement::LineEnd => {
            let line = shaped.get_line_number_at(s.active).unwrap_or(0);
            let lm = shaped
                .get_line_metrics_at(line)
                .or_else(|| shaped.get_line_metrics().into_iter().last())
                .unwrap_or_else(|| return_default_lm());
            (lm.end_index.saturating_sub(1), None)
        }
        other => {
            warn!("unhandled movement {:?}", other);
            (s.anchor, s.h_pos)
        }
    };

    let start = if modify { s.anchor } else { offset };
    Selection::new(start, offset).with_h_pos(h_pos)
}

fn return_default_lm() -> crate::text::shaped_text::LineMetrics {
    crate::text::shaped_text::LineMetrics {
        start_index: 0,
        end_index: 0,
        end_excluding_whitespace: 0,
        end_including_newline: 0,
        hard_break: false,
        ascent: 0.0,
        descent: 0.0,
        unscaled_ascent: 0.0,
        height: 0.0,
        width: 0.0,
        left: 0.0,
        baseline: 0.0,
        line_number: 0,
    }
}

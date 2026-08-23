//! Shaped text representation using Skia text blobs.
//!
//! [`ShapedText`] replaces `skia_safe::textlayout::Paragraph` as the per-entity
//! text representation. It stores:
//!
//! - [`PreShapedText`] — glyph data from Parley shaping (built once in `text_system`).
//! - Laid-out line byte ranges — rebuilt from `PreShapedText` on every `layout()` call
//!   (e.g. when the widget is resized) without re-invoking the shaper.

use std::ops::Range;

use parley::{
    Affinity, BreakReason, Cluster as ParleyCluster, editing::Cursor as ParleyCursor,
    editing::Selection as ParleySelection,
};
use skia_safe::{Font, GlyphId, Paint, Rect};
use vizia_style::TextAlign;

// ─── Public API types ─────────────────────────────────────────────────────────
// These mirror the return types from `skia_safe::textlayout::Paragraph` so that
// existing call-sites in `textbox.rs` and `movement.rs` can be ported with
// minimal churn.

/// A selection rectangle within a text region.
/// Mirrors `skia_safe::textlayout::TextBox`.
#[derive(Debug, Clone)]
pub struct TextBox {
    pub rect: Rect,
    /// `true` if this box belongs to an RTL run.
    pub is_rtl: bool,
}

// ─── Internal pre-shaped types ────────────────────────────────────────────────
// Produced by Parley shaping and stored inside `ShapedText`.
// `layout()` reads these without re-invoking the shaper.

/// One shaped glyph captured from Parley run data.
#[derive(Debug, Clone)]
pub(crate) struct PreGlyph {
    pub glyph_id: GlyphId,
    /// Absolute byte offset of this glyph's cluster in the full text.
    pub cluster_byte: usize,
    /// X position as set by the shaper (relative to the start of the style run).
    pub x: f32,
}

/// Style properties stored per pre-shaped run (needed for rendering).
#[derive(Clone)]
pub(crate) struct RunPaint {
    pub fill: Paint,
    pub underline: bool,
    pub strikethrough: bool,
    pub decoration_paint: Paint,
}

/// A shaped but not yet line-broken run (same font, direction, and style).
#[derive(Clone)]
pub(crate) struct PreShapedRun {
    pub font: Font,
    pub paint: RunPaint,
    /// Absolute byte range in the full text.
    pub byte_range: Range<usize>,
    /// Glyphs in shaper output order (visual left-to-right).
    pub glyphs: Vec<PreGlyph>,
    /// Total X advance of this run (from `RunInfo.advance.x`).
    pub total_advance: f32,
}

/// Full pre-shaped text block — the result of running Parley shaping over an entity's text.
/// This is stored in `TextContext::text_shaped` after `text_system` runs, and consumed by
/// `text_layout_system` (via `ShapedText::layout`).
pub struct PreShapedText {
    /// Styled, shaped runs in logical text order.
    pub(crate) runs: Vec<PreShapedRun>,
    /// The full display text (UTF-8).
    pub(crate) text: String,
    /// Text alignment for line x-offset computation.
    pub(crate) text_align: TextAlign,
    /// Maximum number of lines (from `line-clamp`).
    pub(crate) max_lines: Option<usize>,
    /// Parley layout used to source line ranges at a given wrap width.
    pub(crate) parley_layout: parley::Layout<[u8; 4]>,
}

#[derive(Clone, Copy)]
struct LineByteRange {
    start_index: usize,
    end_including_newline: usize,
}

// ─── ShapedText ───────────────────────────────────────────────────────────────

/// A fully shaped and laid-out block of text.
///
/// Replaces `skia_safe::textlayout::Paragraph` as the per-entity text cache.
/// Call [`ShapedText::layout`] after creating to produce the laid-out lines.
pub struct ShapedText {
    pub(crate) pre_shaped: PreShapedText,
    line_ranges: Vec<LineByteRange>,
    pub height: f32,
    pub max_intrinsic_width: f32,
    pub min_intrinsic_width: f32,
}

impl ShapedText {
    /// Create a new `ShapedText` from pre-shaped data.
    /// Does not perform layout; call [`ShapedText::layout`] before using.
    pub fn new(pre_shaped: PreShapedText) -> Self {
        let widths = pre_shaped.parley_layout.calculate_content_widths();
        let (max_intrinsic_width, min_intrinsic_width) = (widths.max, widths.min);
        ShapedText {
            pre_shaped,
            line_ranges: Vec::new(),
            height: 0.0,
            max_intrinsic_width,
            min_intrinsic_width,
        }
    }

    /// Re-run line breaking at `width` without re-shaping text.
    pub fn layout(&mut self, width: f32) {
        let result = perform_layout(&mut self.pre_shaped, width);
        self.line_ranges = result.line_ranges;
        self.height = result.height;
    }

    // ── Metric API (replaces Paragraph's metric methods) ─────────────────────

    /// Total height of all lines.
    #[inline]
    pub fn height(&self) -> f32 {
        self.height
    }

    /// Maximum intrinsic width (width of the longest single line without wrapping).
    #[inline]
    pub fn max_intrinsic_width(&self) -> f32 {
        self.max_intrinsic_width
    }

    /// Minimum intrinsic width (width of the widest unbreakable word segment).
    #[inline]
    pub fn min_intrinsic_width(&self) -> f32 {
        self.min_intrinsic_width
    }

    /// Number of laid-out lines.
    #[inline]
    pub fn line_count(&self) -> usize {
        self.line_ranges.len()
    }

    /// Which line contains the glyph at byte offset `byte_pos`.
    /// Mirrors `Paragraph::get_line_number_at`.
    pub fn get_line_number_at(&self, byte_pos: usize) -> Option<usize> {
        // Binary search: find the last line whose start_index <= byte_pos.
        let n = self.line_ranges.len();
        if n == 0 {
            return None;
        }
        let mut lo = 0usize;
        let mut hi = n;
        while lo + 1 < hi {
            let mid = (lo + hi) / 2;
            if self.line_ranges[mid].start_index <= byte_pos {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        // `lo` is the last line whose start_index <= byte_pos.
        // Verify that byte_pos is actually within [start, end_including_newline].
        let line = &self.line_ranges[lo];
        if byte_pos <= line.end_including_newline {
            Some(lo)
        } else {
            Some(n - 1) // past all lines -> last line
        }
    }

    /// Bounding rectangles for the text in a byte range.
    /// Mirrors `Paragraph::get_rects_for_range`.
    pub fn get_rects_for_range(&self, range: Range<usize>) -> Vec<TextBox> {
        if range.is_empty() {
            return Vec::new();
        }

        let layout = &self.pre_shaped.parley_layout;
        let text_len = self.pre_shaped.text.len();
        let start = range.start.min(text_len);
        let end = range.end.min(text_len);
        if start >= end {
            return Vec::new();
        }

        let anchor = ParleyCursor::from_byte_index(layout, start, Affinity::Downstream);
        let focus = ParleyCursor::from_byte_index(layout, end, Affinity::Upstream);
        let selection = ParleySelection::new(anchor, focus);

        let mut result = Vec::new();
        selection.geometry_with(layout, |bbox, _line_index| {
            let cx = ((bbox.x0 + bbox.x1) * 0.5) as f32;
            let cy = ((bbox.y0 + bbox.y1) * 0.5) as f32;
            let is_rtl = ParleyCluster::from_point(layout, cx, cy)
                .map(|(cluster, _)| cluster.is_rtl())
                .unwrap_or(false);

            result.push(TextBox {
                rect: Rect::new(bbox.x0 as f32, bbox.y0 as f32, bbox.x1 as f32, bbox.y1 as f32),
                is_rtl,
            });
        });

        result
    }

    pub(crate) fn selectable_text(&self) -> &str {
        self.pre_shaped.text.strip_suffix('\u{200B}').unwrap_or(&self.pre_shaped.text)
    }

    pub(crate) fn selectable_len(&self) -> usize {
        let text_len = self.selectable_text().len();
        self.line_ranges.last().map(|line| line.end_including_newline.min(text_len)).unwrap_or(0)
    }

    pub(crate) fn point_at(&self, x: f32, y: f32) -> (usize, Affinity) {
        let cursor = ParleyCursor::from_point(&self.pre_shaped.parley_layout, x, y);
        (cursor.index().min(self.selectable_len()), cursor.affinity())
    }

    pub(crate) fn word_at(&self, x: f32, y: f32) -> Range<usize> {
        let selection = ParleySelection::word_from_point(&self.pre_shaped.parley_layout, x, y);
        let range = selection.text_range();
        range.start.min(self.selectable_len())..range.end.min(self.selectable_len())
    }

    pub(crate) fn move_visual(
        &self,
        byte: usize,
        affinity: Affinity,
        forward: bool,
        by_word: bool,
    ) -> (usize, Affinity) {
        let layout = &self.pre_shaped.parley_layout;
        let cursor =
            ParleyCursor::from_byte_index(layout, byte.min(self.selectable_len()), affinity);
        let selection = ParleySelection::from(cursor);
        let moved = match (forward, by_word) {
            (true, true) => selection.next_visual_word(layout, false),
            (true, false) => selection.next_visual(layout, false),
            (false, true) => selection.previous_visual_word(layout, false),
            (false, false) => selection.previous_visual(layout, false),
        };
        let focus = moved.focus();
        (focus.index().min(self.selectable_len()), focus.affinity())
    }

    pub(crate) fn move_line_edge(
        &self,
        byte: usize,
        affinity: Affinity,
        end: bool,
    ) -> (usize, Affinity) {
        let layout = &self.pre_shaped.parley_layout;
        let cursor =
            ParleyCursor::from_byte_index(layout, byte.min(self.selectable_len()), affinity);
        let selection = ParleySelection::from(cursor);
        let moved = if end {
            selection.line_end(layout, false)
        } else {
            selection.line_start(layout, false)
        };
        let focus = moved.focus();
        (focus.index().min(self.selectable_len()), focus.affinity())
    }

    pub(crate) fn move_line(
        &self,
        byte: usize,
        affinity: Affinity,
        down: bool,
        preferred_x: Option<f32>,
    ) -> ((usize, Affinity), f32, bool) {
        let layout = &self.pre_shaped.parley_layout;
        let cursor =
            ParleyCursor::from_byte_index(layout, byte.min(self.selectable_len()), affinity);
        let geometry = cursor.geometry(layout, 0.0);
        let preferred_x = preferred_x.unwrap_or(geometry.x0 as f32);
        let current_line = layout
            .lines()
            .position(|line| geometry.y0 as f32 <= line.metrics().block_max_coord)
            .unwrap_or(0)
            .min(self.line_count().saturating_sub(1));
        let target_line = if down {
            current_line.checked_add(1).filter(|index| *index < self.line_count())
        } else {
            current_line.checked_sub(1)
        };
        let Some(target_line) = target_line else {
            return (
                (cursor.index().min(self.selectable_len()), cursor.affinity()),
                preferred_x,
                true,
            );
        };
        let Some(line) = layout.get(target_line) else {
            return (
                (cursor.index().min(self.selectable_len()), cursor.affinity()),
                preferred_x,
                true,
            );
        };
        let y = line.metrics().block_max_coord - line.metrics().ascent * 0.5;
        let moved = ParleyCursor::from_point(layout, preferred_x, y);
        ((moved.index().min(self.selectable_len()), moved.affinity()), preferred_x, false)
    }

    // ── Private helpers ───────────────────────────────────────────────────────
}

// ─── Layout engine ────────────────────────────────────────────────────────────

struct LayoutResult {
    line_ranges: Vec<LineByteRange>,
    height: f32,
}

/// Parley line-breaking and TextBlob construction from pre-shaped data.
fn perform_layout(pre: &mut PreShapedText, constraint_width: f32) -> LayoutResult {
    if pre.text.is_empty() {
        // Only reachable for an editor-driven `PreShapedText` (e.g. an empty `Textbox`) — a
        // label's text always has a trailing ZWS appended (see `build_pre_shaped_text`), so
        // `pre.text` is never truly empty there. The editor's layout still carries one implicit
        // empty line; use its metrics so callers computing caret/selection vertical-centering
        // (which divide by this height) don't treat the content block as 0-height and draw the
        // caret too low.
        let layout = &mut pre.parley_layout;
        layout.break_all_lines(Some(constraint_width.max(0.0)));
        let height =
            layout.lines().next().map(|line| line.metrics().block_max_coord).unwrap_or(0.0);
        return LayoutResult { line_ranges: Vec::new(), height };
    }

    if pre.runs.is_empty() {
        return LayoutResult { line_ranges: Vec::new(), height: 0.0 };
    }

    let layout = &mut pre.parley_layout;
    layout.break_all_lines(Some(constraint_width.max(0.0)));

    let text_len = pre.text.len();
    let mut line_ranges: Vec<LineByteRange> = Vec::new();
    let mut height = 0.0;

    for line in layout.lines() {
        let mut text_range = line.text_range();
        text_range.start = text_range.start.min(text_len);
        text_range.end = text_range.end.min(text_len);

        let break_reason = line.break_reason();
        let explicit_break = matches!(break_reason, BreakReason::Explicit);
        if text_range.start > text_range.end {
            continue;
        }
        if text_range.start == text_range.end && !explicit_break {
            continue;
        }

        let lm = line.metrics();
        // Use `block_max_coord` (baseline + descent + leading-below) rather than
        // `baseline + descent` alone. Parley distributes the line-height leading
        // split above/below the natural ascent/descent; omitting `leading_below`
        // undercounts total content height whenever `line-height` exceeds the
        // font's natural metrics (e.g. 1.5), causing auto-height boxes to be
        // slightly shorter than the rendered content and requiring a tiny scroll.
        height = lm.block_max_coord;

        line_ranges.push(LineByteRange {
            start_index: text_range.start,
            end_including_newline: text_range.end,
        });

        if let Some(max) = pre.max_lines
            && line_ranges.len() >= max
        {
            break;
        }
    }

    LayoutResult { line_ranges, height }
}

//! Shaped text representation using Skia text blobs.
//!
//! [`ShapedText`] replaces `skia_safe::textlayout::Paragraph` as the per-entity
//! text representation. It stores:
//!
//! - [`PreShapedText`] — glyph data from the Skia Shaper (built once in `text_system`).
//! - Laid-out [`ShapedLine`]s — rebuilt from `PreShapedText` on every `layout()` call
//!   (e.g. when the widget is resized) without re-invoking the shaper.

use std::collections::BTreeMap;
use std::ops::Range;

use skia_safe::{Font, GlyphId, Paint, Rect, TextBlob, TextBlobBuilder};
use unicode_linebreak::{BreakOpportunity, linebreaks};
use vizia_style::TextAlign;

// ─── Public API types ─────────────────────────────────────────────────────────
// These mirror the return types from `skia_safe::textlayout::Paragraph` so that
// existing call-sites in `textbox.rs` and `movement.rs` can be ported with
// minimal churn.

/// Information about a glyph cluster, mirroring `skia_safe::textlayout::GlyphClusterInfo`.
#[derive(Debug, Clone)]
pub struct GlyphClusterInfo {
    /// Tight bounding box of the cluster in text-local coordinates
    /// (y is relative to the line baseline; negative = above baseline).
    pub bounds: Rect,
    /// UTF-8 byte range of this cluster in the original full text.
    pub text_range: Range<usize>,
    /// `true` if this cluster belongs to an RTL run.
    pub is_rtl: bool,
}

impl GlyphClusterInfo {
    /// X centre of the cluster bounds, used by movement code to pick affinity.
    #[inline]
    pub fn center_x(&self) -> f32 {
        self.bounds.center_x()
    }
}

/// Return type of [`ShapedText::get_glyph_position_at_coordinate`].
#[derive(Debug, Clone)]
pub struct GlyphPosition {
    /// UTF-8 byte offset of the glyph in the original text.
    pub position: usize,
    /// Whether the position is upstream (before) or downstream (after) the hit glyph.
    pub affinity: Affinity,
}

/// Affinity for a glyph position (cursor placement side).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Affinity {
    Upstream,
    Downstream,
}

/// A selection rectangle within a text region.
/// Mirrors `skia_safe::textlayout::TextBox`.
#[derive(Debug, Clone)]
pub struct TextBox {
    pub rect: Rect,
    /// `true` if this box belongs to an RTL run.
    pub is_rtl: bool,
}

/// Per-line metrics.  Mirrors `skia_safe::textlayout::LineMetrics`.
#[derive(Debug, Clone)]
pub struct LineMetrics {
    /// Byte offset of the first code unit on this line.
    pub start_index: usize,
    /// Byte offset just past the last *non-whitespace* code unit on this line.
    pub end_index: usize,
    /// `end_index` but including trailing whitespace (not the newline).
    pub end_excluding_whitespace: usize,
    /// `end_index` including the trailing hard newline, if present.
    pub end_including_newline: usize,
    /// `true` if this line ends with a hard (`\n`) break.
    pub hard_break: bool,
    /// Ascent above the baseline (positive).
    pub ascent: f64,
    /// Descent below the baseline (positive).
    pub descent: f64,
    /// Unscaled ascent (same as `ascent` here — included for API parity).
    pub unscaled_ascent: f64,
    /// Total line height = `ascent + descent + leading`.
    pub height: f64,
    /// Visual width of this line.
    pub width: f64,
    /// Left edge of the line (accounts for text-align offset).
    pub left: f64,
    /// Y coordinate of the baseline, measured from the top of the text box.
    pub baseline: f64,
    /// Zero-based line index.
    pub line_number: usize,
}

// ─── Internal pre-shaped types ────────────────────────────────────────────────
// Produced by the Skia Shaper and stored inside `ShapedText`.
// `layout()` reads these without re-invoking the shaper.

/// One glyph from the Skia Shaper's run-handler callback.
#[derive(Debug, Clone)]
pub(crate) struct PreGlyph {
    pub glyph_id: GlyphId,
    /// Absolute byte offset of this glyph's cluster in the full text.
    pub cluster_byte: usize,
    /// X position as set by the shaper (relative to the start of the style run).
    pub x: f32,
    /// Y offset from the baseline (usually 0.0 for horizontal text).
    pub y: f32,
}

/// Cluster entry derived from [`PreShapedRun`] — one entry per unique cluster_byte.
#[derive(Debug, Clone)]
pub struct GlyphCluster {
    /// Representative glyph ID (first glyph of this cluster).
    pub glyph_id: GlyphId,
    /// Absolute UTF-8 byte start in the full text.
    pub byte_start: usize,
    /// Absolute UTF-8 byte end (exclusive) in the full text.
    pub byte_end: usize,
    /// Leftmost visual X of this cluster, relative to the start of the style run.
    pub x: f32,
    /// Total horizontal advance of this cluster.
    pub advance: f32,
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
    /// `true` if this run is RTL.
    pub is_rtl: bool,
    /// Absolute byte range in the full text.
    pub byte_range: Range<usize>,
    /// Glyphs in shaper output order (visual left-to-right).
    pub glyphs: Vec<PreGlyph>,
    /// Total X advance of this run (from `RunInfo.advance.x`).
    pub total_advance: f32,
    /// Ascent above the baseline (positive).
    pub ascent: f32,
    /// Descent below the baseline (positive).
    pub descent: f32,
    /// Font leading.
    pub leading: f32,
    /// Cluster list, pre-computed from `glyphs` for efficient layout.
    /// Sorted ascending by `byte_start`.
    pub clusters: Vec<GlyphCluster>,
}

impl PreShapedRun {
    /// Build the cluster list from the raw glyph data.
    /// Must be called once after shaping, before the run is used for layout.
    pub(crate) fn compute_clusters(&mut self) {
        self.clusters = compute_run_clusters(
            &self.glyphs,
            &self.byte_range,
            self.total_advance,
            self.ascent,
            self.descent,
        );
    }
}

/// Compute per-cluster data from a list of pre-shaped glyphs.
/// Returns clusters sorted ascending by `byte_start`.
fn compute_run_clusters(
    glyphs: &[PreGlyph],
    byte_range: &Range<usize>,
    total_advance: f32,
    ascent: f32,
    descent: f32,
) -> Vec<GlyphCluster> {
    if glyphs.is_empty() {
        return Vec::new();
    }

    // Group glyph indices by cluster_byte, recording the leftmost x per cluster.
    // BTreeMap gives us clusters sorted by cluster_byte (= logical byte order).
    let mut cluster_map: BTreeMap<usize, (GlyphId, f32)> = BTreeMap::new();
    for g in glyphs {
        let entry = cluster_map.entry(g.cluster_byte).or_insert((g.glyph_id, g.x));
        if g.x < entry.1 {
            entry.1 = g.x; // keep leftmost x
        }
    }

    // Collect in ascending byte order.
    let by_byte: Vec<(usize, GlyphId, f32)> =
        cluster_map.into_iter().map(|(b, (gid, x))| (b, gid, x)).collect();
    let n = by_byte.len();

    // Compute per-cluster advance using visual x ordering.
    // For both LTR (x increases with byte) and RTL (x decreases with byte),
    // the advance is determined by the x gap to the next cluster in VISUAL order.
    let mut by_x = by_byte.clone();
    by_x.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

    // advances[original_idx] = advance for that cluster
    let mut advances = vec![0.0f32; n];
    for (visual_rank, item) in by_x.iter().enumerate() {
        let orig_idx = by_byte.iter().position(|c| c.0 == item.0).unwrap_or(0);
        let x = item.2;
        let next_x = if visual_rank + 1 < n { by_x[visual_rank + 1].2 } else { total_advance };
        advances[orig_idx] = (next_x - x).max(0.0);
    }

    // Build final cluster list.
    let mut result = Vec::with_capacity(n);
    for (idx, (cluster_byte, glyph_id, x)) in by_byte.iter().enumerate() {
        let byte_end = if idx + 1 < n { by_byte[idx + 1].0 } else { byte_range.end };

        result.push(GlyphCluster {
            glyph_id: *glyph_id,
            byte_start: *cluster_byte,
            byte_end,
            x: *x,
            advance: advances[idx],
        });
    }

    result
}

/// Full pre-shaped text block — the result of running the Skia Shaper over an entity's text.
/// This is stored in `TextContext::text_shaped` after `text_system` runs, and consumed by
/// `text_layout_system` (via `ShapedText::layout`).
pub struct PreShapedText {
    /// Styled, shaped runs in logical text order.
    pub(crate) runs: Vec<PreShapedRun>,
    /// The full display text (UTF-8).
    pub(crate) text: String,
    /// Text alignment for line x-offset computation.
    pub(crate) text_align: TextAlign,
    /// `true` if the base paragraph direction is RTL.
    pub(crate) base_direction_rtl: bool,
    /// Whether to append an ellipsis when the text overflows.
    pub(crate) ellipsis: bool,
    /// Maximum number of lines (from `line-clamp`).
    pub(crate) max_lines: Option<usize>,
    /// Line break opportunities from `unicode_linebreak::linebreaks`.
    /// Each entry is `(byte_pos, is_mandatory)` where `byte_pos` is the index
    /// at which the next line starts (break occurs just before `byte_pos`).
    pub(crate) break_opportunities: Vec<(usize, bool)>,
}

impl PreShapedText {
    /// Compute break opportunities from `self.text` and store them.
    pub(crate) fn compute_break_opportunities(&mut self) {
        self.break_opportunities.clear();
        for (byte_pos, opp) in linebreaks(&self.text) {
            let is_mandatory = matches!(opp, BreakOpportunity::Mandatory);
            self.break_opportunities.push((byte_pos, is_mandatory));
        }
    }

    /// Total visual advance assuming no line breaks (used for `max_intrinsic_width`).
    fn total_advance(&self) -> f32 {
        self.runs.iter().map(|r| r.total_advance).sum()
    }

    /// Minimum advance: the widest single break segment (used for `min_intrinsic_width`).
    fn min_segment_advance(&self) -> f32 {
        let mut max_seg = 0.0f32;
        let mut seg_start = 0usize;
        for &(break_pos, _) in &self.break_opportunities {
            let seg_advance = self.advance_in_range(seg_start, break_pos);
            max_seg = max_seg.max(seg_advance);
            seg_start = break_pos;
        }
        max_seg
    }

    /// Compute total advance for a byte range across all runs.
    pub(crate) fn advance_in_range(&self, start: usize, end: usize) -> f32 {
        let mut total = 0.0f32;
        for run in &self.runs {
            let overlap_start = run.byte_range.start.max(start);
            let overlap_end = run.byte_range.end.min(end);
            if overlap_start >= overlap_end {
                continue;
            }
            for cluster in &run.clusters {
                if cluster.byte_start >= overlap_start && cluster.byte_start < overlap_end {
                    total += cluster.advance;
                }
            }
        }
        total
    }
}

// ─── Final line / run types ───────────────────────────────────────────────────

/// A single laid-out run within a line (constant font, style, direction).
#[derive(Clone)]
pub struct ShapedRun {
    /// Pre-rendered glyph blob.
    pub blob: TextBlob,
    /// X offset of this run's origin within the line (after text-align is applied).
    pub x_offset: f32,
    /// `true` if this is an RTL run.
    pub is_rtl: bool,
    /// Cluster metadata for metric queries (same structure as `PreShapedRun::clusters`,
    /// but trimmed to this line's byte range, and `x` values are run-relative).
    pub clusters: Vec<GlyphCluster>,
    /// Paint used to draw the blob.
    pub paint: Paint,
    /// Decoration flags.
    pub underline: bool,
    pub strikethrough: bool,
    pub decoration_paint: Paint,
    /// Absolute byte range this run covers in the full text.
    pub byte_range: Range<usize>,
    /// Baseline Y relative to the text box top (baked into the blob's `alloc_run_pos_h` y).
    pub baseline_y: f32,
}

/// A fully laid-out line of text.
#[derive(Clone)]
pub struct ShapedLine {
    pub runs: Vec<ShapedRun>,
    pub metrics: LineMetrics,
}

// ─── ShapedText ───────────────────────────────────────────────────────────────

/// A fully shaped and laid-out block of text.
///
/// Replaces `skia_safe::textlayout::Paragraph` as the per-entity text cache.
/// Call [`ShapedText::layout`] after creating to produce the laid-out lines.
pub struct ShapedText {
    pub(crate) pre_shaped: PreShapedText,
    pub lines: Vec<ShapedLine>,
    pub layout_width: f32,
    pub height: f32,
    pub max_intrinsic_width: f32,
    pub min_intrinsic_width: f32,
}

impl ShapedText {
    /// Create a new `ShapedText` from pre-shaped data.
    /// Does not perform layout; call [`ShapedText::layout`] before using.
    pub fn new(pre_shaped: PreShapedText) -> Self {
        let max_intrinsic_width = pre_shaped.total_advance();
        let min_intrinsic_width = pre_shaped.min_segment_advance();
        ShapedText {
            pre_shaped,
            lines: Vec::new(),
            layout_width: f32::INFINITY,
            height: 0.0,
            max_intrinsic_width,
            min_intrinsic_width,
        }
    }

    /// Re-run line breaking at `width`.  Does not re-invoke the Skia Shaper.
    pub fn layout(&mut self, width: f32) {
        self.layout_width = width;
        let result = perform_layout(&self.pre_shaped, width);
        self.lines = result.lines;
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
        self.lines.len()
    }

    /// All line metrics, mirroring `Paragraph::get_line_metrics`.
    pub fn get_line_metrics(&self) -> Vec<LineMetrics> {
        self.lines.iter().map(|l| l.metrics.clone()).collect()
    }

    /// Metrics for line `n` (zero-based), mirroring `Paragraph::get_line_metrics_at`.
    pub fn get_line_metrics_at(&self, n: usize) -> Option<LineMetrics> {
        self.lines.get(n).map(|l| l.metrics.clone())
    }

    /// Which line contains the glyph at byte offset `byte_pos`.
    /// Mirrors `Paragraph::get_line_number_at`.
    pub fn get_line_number_at(&self, byte_pos: usize) -> Option<usize> {
        // Binary search: find the last line whose start_index <= byte_pos.
        let n = self.lines.len();
        if n == 0 {
            return None;
        }
        let mut lo = 0usize;
        let mut hi = n;
        while lo + 1 < hi {
            let mid = (lo + hi) / 2;
            if self.lines[mid].metrics.start_index <= byte_pos {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        // `lo` is the last line whose start_index <= byte_pos.
        // Verify that byte_pos is actually within [start, end_including_newline].
        let lm = &self.lines[lo].metrics;
        if byte_pos <= lm.end_including_newline {
            Some(lo)
        } else {
            Some(n - 1) // past all lines → last line
        }
    }

    /// Cluster info for the cluster at (or containing) `byte_pos`.
    /// Mirrors `Paragraph::get_glyph_cluster_at`.
    pub fn get_glyph_cluster_at(&self, byte_pos: usize) -> Option<GlyphClusterInfo> {
        for line in &self.lines {
            let lm = &line.metrics;
            if byte_pos > lm.end_including_newline {
                continue;
            }
            for run in &line.runs {
                if byte_pos < run.byte_range.start || byte_pos >= run.byte_range.end {
                    continue;
                }
                for cluster in &run.clusters {
                    if byte_pos >= cluster.byte_start && byte_pos < cluster.byte_end {
                        let bounds = Rect::new(
                            run.x_offset + cluster.x,
                            -(lm.ascent as f32),
                            run.x_offset + cluster.x + cluster.advance,
                            lm.descent as f32,
                        );
                        return Some(GlyphClusterInfo {
                            bounds,
                            text_range: cluster.byte_start..cluster.byte_end,
                            is_rtl: run.is_rtl,
                        });
                    }
                }
            }
        }
        // Fallback: last cluster
        self.last_cluster_info()
    }

    /// Find the glyph cluster whose visual position is closest to `(x, y)`.
    /// Mirrors `Paragraph::get_closest_glyph_cluster_at`.
    pub fn get_closest_glyph_cluster_at(&self, (x, y): (f32, f32)) -> Option<GlyphClusterInfo> {
        let line = self.line_at_y(y)?;
        let lm = &line.metrics;

        let mut best_dist = f32::INFINITY;
        let mut best: Option<GlyphClusterInfo> = None;

        for run in &line.runs {
            for cluster in &run.clusters {
                let cx = run.x_offset + cluster.x + cluster.advance * 0.5;
                let dist = (cx - x).abs();
                if dist < best_dist {
                    best_dist = dist;
                    let bounds = Rect::new(
                        run.x_offset + cluster.x,
                        -(lm.ascent as f32),
                        run.x_offset + cluster.x + cluster.advance,
                        lm.descent as f32,
                    );
                    best = Some(GlyphClusterInfo {
                        bounds,
                        text_range: cluster.byte_start..cluster.byte_end,
                        is_rtl: run.is_rtl,
                    });
                }
            }
        }

        best
    }

    /// Convert a screen-local `(x, y)` coordinate to a byte offset.
    /// Mirrors `Paragraph::get_glyph_position_at_coordinate`.
    pub fn get_glyph_position_at_coordinate(&self, (x, y): (f32, f32)) -> GlyphPosition {
        let Some(line) = self.line_at_y(y) else {
            return GlyphPosition { position: 0, affinity: Affinity::Downstream };
        };
        let lm = &line.metrics;

        // Find the cluster at or nearest to x.
        let mut best_pos = lm.start_index;
        let mut best_dist = f32::INFINITY;

        for run in &line.runs {
            for cluster in &run.clusters {
                let cluster_left = run.x_offset + cluster.x;
                let cluster_right = cluster_left + cluster.advance;
                let cx = cluster_left + cluster.advance * 0.5;

                if x >= cluster_left && x < cluster_right {
                    // Hit: interpolate within the cluster for ligature-awareness.
                    let frac = (x - cluster_left) / cluster.advance.max(0.001);
                    let text = &self.pre_shaped.text[cluster.byte_start..cluster.byte_end];
                    let char_count = text.chars().count().max(1);
                    let char_idx = (frac * char_count as f32).round() as usize;
                    let byte_offset = text
                        .char_indices()
                        .nth(char_idx)
                        .map(|(i, _)| cluster.byte_start + i)
                        .unwrap_or(cluster.byte_end);
                    return GlyphPosition {
                        position: byte_offset,
                        affinity: if frac < 0.5 {
                            Affinity::Downstream
                        } else {
                            Affinity::Upstream
                        },
                    };
                }

                let dist = (cx - x).abs();
                if dist < best_dist {
                    best_dist = dist;
                    best_pos = if x <= cx { cluster.byte_start } else { cluster.byte_end };
                }
            }
        }

        GlyphPosition { position: best_pos, affinity: Affinity::Downstream }
    }

    /// Bounding rectangles for the text in a byte range.
    /// Mirrors `Paragraph::get_rects_for_range`.
    pub fn get_rects_for_range(&self, range: Range<usize>) -> Vec<TextBox> {
        let mut result = Vec::new();

        for line in &self.lines {
            let lm = &line.metrics;
            for run in &line.runs {
                // Only process clusters that overlap the query range.
                let mut x_start = f32::INFINITY;
                let mut x_end = f32::NEG_INFINITY;
                let mut any = false;

                for cluster in &run.clusters {
                    if cluster.byte_start >= range.start && cluster.byte_end <= range.end {
                        let cx = run.x_offset + cluster.x;
                        x_start = x_start.min(cx);
                        x_end = x_end.max(cx + cluster.advance);
                        any = true;
                    } else if cluster.byte_start < range.end && cluster.byte_end > range.start {
                        // Partial overlap (ligature spanning the range boundary).
                        // Include the full cluster rect.
                        let cx = run.x_offset + cluster.x;
                        x_start = x_start.min(cx);
                        x_end = x_end.max(cx + cluster.advance);
                        any = true;
                    }
                }

                if any {
                    let top = lm.baseline as f32 - lm.ascent as f32;
                    let bottom = lm.baseline as f32 + lm.descent as f32;
                    result.push(TextBox {
                        rect: Rect::new(x_start, top, x_end, bottom),
                        is_rtl: run.is_rtl,
                    });
                }
            }
        }

        result
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    /// Find the line containing vertical coordinate `y` (text-local, from top).
    fn line_at_y(&self, y: f32) -> Option<&ShapedLine> {
        let mut result = self.lines.first()?;
        for line in &self.lines {
            let top = line.metrics.baseline as f32 - line.metrics.ascent as f32;
            if y >= top {
                result = line;
            }
        }
        Some(result)
    }

    /// Cluster info for the very last glyph (used as a fallback).
    fn last_cluster_info(&self) -> Option<GlyphClusterInfo> {
        let line = self.lines.last()?;
        let run = line.runs.last()?;
        let cluster = run.clusters.last()?;
        let lm = &line.metrics;
        let bounds = Rect::new(
            run.x_offset + cluster.x,
            -(lm.ascent as f32),
            run.x_offset + cluster.x + cluster.advance,
            lm.descent as f32,
        );
        Some(GlyphClusterInfo {
            bounds,
            text_range: cluster.byte_start..cluster.byte_end,
            is_rtl: run.is_rtl,
        })
    }
}

// ─── Layout engine ────────────────────────────────────────────────────────────

struct LayoutResult {
    lines: Vec<ShapedLine>,
    height: f32,
}

/// Greedy line-breaking and TextBlob construction from pre-shaped data.
fn perform_layout(pre: &PreShapedText, constraint_width: f32) -> LayoutResult {
    if pre.runs.is_empty() || pre.text.is_empty() {
        return LayoutResult { lines: Vec::new(), height: 0.0 };
    }

    // ── Step 1: identify break segments ──────────────────────────────────────
    // A "segment" is text between two consecutive break opportunities.
    // We greedily pack segments onto lines.

    struct Segment {
        byte_start: usize,
        byte_end: usize,
        advance: f32,
        mandatory_break: bool,
    }

    let mut segments: Vec<Segment> = Vec::new();
    let mut prev_break = 0usize;
    for &(break_pos, is_mandatory) in &pre.break_opportunities {
        if break_pos <= prev_break {
            continue;
        }
        let advance = pre.advance_in_range(prev_break, break_pos);
        segments.push(Segment {
            byte_start: prev_break,
            byte_end: break_pos,
            advance,
            mandatory_break: is_mandatory,
        });
        prev_break = break_pos;
    }
    // Trailing text after the last break opportunity.
    if prev_break < pre.text.len() {
        let advance = pre.advance_in_range(prev_break, pre.text.len());
        segments.push(Segment {
            byte_start: prev_break,
            byte_end: pre.text.len(),
            advance,
            mandatory_break: false,
        });
    }

    if segments.is_empty() {
        return LayoutResult { lines: Vec::new(), height: 0.0 };
    }

    // ── Step 2: greedy packing ────────────────────────────────────────────────
    struct LineRange {
        byte_start: usize,
        byte_end: usize,
        width: f32,
        hard_break: bool,
    }

    let mut line_ranges: Vec<LineRange> = Vec::new();
    let mut cur_start = segments[0].byte_start;
    let mut cur_end = cur_start;
    let mut cur_width = 0.0f32;

    for seg in &segments {
        if cur_width > 0.0 && cur_width + seg.advance > constraint_width && !seg.mandatory_break {
            // Emit the current line before this segment.
            line_ranges.push(LineRange {
                byte_start: cur_start,
                byte_end: cur_end,
                width: cur_width,
                hard_break: false,
            });
            cur_start = seg.byte_start;
            cur_width = 0.0;
        }

        cur_end = seg.byte_end;
        cur_width += seg.advance;

        if seg.mandatory_break {
            line_ranges.push(LineRange {
                byte_start: cur_start,
                byte_end: cur_end,
                width: cur_width,
                hard_break: true,
            });
            cur_start = cur_end;
            cur_width = 0.0;
        }

        // Apply max_lines.
        if let Some(max) = pre.max_lines {
            if line_ranges.len() >= max {
                break;
            }
        }
    }

    // Emit the last (partial) line.
    if cur_end > cur_start {
        line_ranges.push(LineRange {
            byte_start: cur_start,
            byte_end: cur_end,
            width: cur_width,
            hard_break: false,
        });
    }

    if let Some(max) = pre.max_lines {
        line_ranges.truncate(max);
    }

    // ── Step 3: compute global font metrics (line height) ────────────────────
    // Use the maximum ascent / descent / leading across all runs.
    let (global_ascent, global_descent, global_leading) =
        pre.runs.iter().fold((0.0f32, 0.0f32, 0.0f32), |(a, d, l), r| {
            (a.max(r.ascent), d.max(r.descent), l.max(r.leading))
        });
    let line_height = global_ascent + global_descent + global_leading;

    // ── Step 4: build ShapedLines with TextBlobs ──────────────────────────────
    let mut lines: Vec<ShapedLine> = Vec::with_capacity(line_ranges.len());
    let mut baseline_y = global_ascent;

    for (line_idx, lr) in line_ranges.iter().enumerate() {
        // Compute trailing whitespace / newline boundaries.
        let text_slice = &pre.text[lr.byte_start..lr.byte_end];
        let trimmed_len = text_slice.trim_end().len();
        let end_excluding_ws = lr.byte_start + trimmed_len;

        let has_newline = lr.hard_break;
        let newline_bytes = if has_newline {
            // The last character in this range should be `\n`; find it.
            pre.text[lr.byte_start..lr.byte_end]
                .chars()
                .rev()
                .next()
                .map(|c| c.len_utf8())
                .unwrap_or(0)
        } else {
            0
        };
        let end_including_newline = if has_newline { lr.byte_end } else { lr.byte_end };

        // Line x offset for text-align.
        let line_x_offset = compute_line_x_offset(
            pre.text_align,
            pre.base_direction_rtl,
            lr.width,
            constraint_width,
        );

        // Build runs for this line.
        let line_runs = build_line_runs(pre, lr.byte_start, lr.byte_end, baseline_y, line_x_offset);

        // Compute actual line width from runs (may differ slightly from lr.width due to trimming).
        let actual_width = line_runs
            .iter()
            .map(|r| r.clusters.iter().map(|c| r.x_offset + c.x + c.advance).fold(0.0f32, f32::max))
            .fold(0.0f32, f32::max);

        let metrics = LineMetrics {
            start_index: lr.byte_start,
            end_index: end_excluding_ws,
            end_excluding_whitespace: end_excluding_ws,
            end_including_newline,
            hard_break: lr.hard_break,
            ascent: global_ascent as f64,
            descent: global_descent as f64,
            unscaled_ascent: global_ascent as f64,
            height: line_height as f64,
            width: actual_width as f64,
            left: line_x_offset as f64,
            baseline: baseline_y as f64,
            line_number: line_idx,
        };

        lines.push(ShapedLine { runs: line_runs, metrics });
        baseline_y += line_height;
    }

    // `baseline_y` now sits past the last line (it was incremented after each push).
    // Total height = last_baseline + descent
    //              = (baseline_y - line_height) + global_descent
    //              = baseline_y - global_ascent - global_leading
    let height = if lines.is_empty() { 0.0 } else { baseline_y - global_ascent - global_leading };

    LayoutResult { lines, height }
}

/// Compute the X offset for a line based on text alignment.
fn compute_line_x_offset(
    align: TextAlign,
    base_rtl: bool,
    line_width: f32,
    constraint_width: f32,
) -> f32 {
    let effective_align = if base_rtl {
        match align {
            TextAlign::Left => TextAlign::Right,
            TextAlign::Right => TextAlign::Left,
            other => other,
        }
    } else {
        align
    };

    match effective_align {
        TextAlign::Right => (constraint_width - line_width).max(0.0),
        TextAlign::Center => ((constraint_width - line_width) * 0.5).max(0.0),
        _ => 0.0, // Left / Start / Justify
    }
}

/// Build the `ShapedRun`s for a single line spanning `[line_start, line_end)`.
fn build_line_runs(
    pre: &PreShapedText,
    line_start: usize,
    line_end: usize,
    baseline_y: f32,
    line_x_offset: f32,
) -> Vec<ShapedRun> {
    let mut runs: Vec<ShapedRun> = Vec::new();
    let mut cursor_x = line_x_offset;

    for pre_run in &pre.runs {
        let overlap_start = pre_run.byte_range.start.max(line_start);
        let overlap_end = pre_run.byte_range.end.min(line_end);
        if overlap_start >= overlap_end {
            continue;
        }

        // Collect glyphs whose cluster_byte falls in [overlap_start, overlap_end).
        // Keep them in shaper output order (visual left-to-right).
        let line_glyphs: Vec<&PreGlyph> = pre_run
            .glyphs
            .iter()
            .filter(|g| g.cluster_byte >= overlap_start && g.cluster_byte < overlap_end)
            .collect();

        if line_glyphs.is_empty() {
            continue;
        }

        // Glyphs should be sorted by x (shaper output is visual left-to-right).
        // The leftmost x becomes the origin of this run-slice.
        let x_origin = line_glyphs.iter().map(|g| g.x).fold(f32::INFINITY, f32::min);

        let n = line_glyphs.len();
        let mut builder = TextBlobBuilder::new();
        let (glyph_ids, x_positions) = builder.alloc_run_pos_h(&pre_run.font, n, baseline_y, None);

        for (i, g) in line_glyphs.iter().enumerate() {
            glyph_ids[i] = g.glyph_id;
            x_positions[i] = g.x - x_origin; // run-relative
        }

        let Some(blob) = builder.make() else { continue };

        // Compute run clusters (trimmed to this line's byte range and re-based to run-relative x).
        let run_clusters: Vec<GlyphCluster> = pre_run
            .clusters
            .iter()
            .filter(|c| c.byte_start >= overlap_start && c.byte_start < overlap_end)
            .map(|c| GlyphCluster {
                glyph_id: c.glyph_id,
                byte_start: c.byte_start,
                byte_end: c.byte_end.min(overlap_end),
                x: c.x - x_origin, // run-relative
                advance: c.advance,
            })
            .collect();

        // Compute the advance of this run-slice (sum of cluster advances in the slice).
        let slice_advance: f32 = run_clusters.iter().map(|c| c.advance).sum();

        runs.push(ShapedRun {
            blob,
            x_offset: cursor_x,
            is_rtl: pre_run.is_rtl,
            clusters: run_clusters,
            paint: pre_run.paint.fill.clone(),
            underline: pre_run.paint.underline,
            strikethrough: pre_run.paint.strikethrough,
            decoration_paint: pre_run.paint.decoration_paint.clone(),
            byte_range: overlap_start..overlap_end,
            baseline_y,
        });

        cursor_x += slice_advance;
    }

    runs
}

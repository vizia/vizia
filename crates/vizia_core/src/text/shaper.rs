//! Shaping pipeline: converts an entity's text + style tree into [`PreShapedText`].
//!
//! `build_pre_shaped_text` replaces `build_paragraph` + `add_block` from `systems/text.rs`.
//! It calls the Skia Shaper (HarfBuzz on Linux/Windows, CoreText on macOS) once per styled
//! run and records the shaped glyph data in a [`PreShapedText`] value, which is later
//! consumed by `ShapedText::layout`.

use skia_safe::{
    BlendMode, Font, FontArguments, FontStyle, GlyphId, Paint, Point, Shaper, Typeface,
    font::Edging,
    font_arguments::VariationPosition,
    shaper::{
        RunHandler,
        run_handler::{Buffer, RunInfo},
    },
    textlayout::FontCollection,
};
use vizia_storage::LayoutChildIterator;

use crate::{
    entity::Entity,
    prelude::{FamilyOwned, LineHeight, TextAlign, TextOverflow},
    style::Style,
    text::{
        resolved_text_direction,
        shaped_text::{PreGlyph, PreShapedRun, PreShapedText, RunPaint},
    },
    tree::Tree,
};

// ─── CapturingRunHandler ──────────────────────────────────────────────────────

/// A `RunHandler` for the Skia Shaper that captures glyph IDs, positions, and
/// cluster byte offsets for each shaped run.
///
/// The shaper calls these methods in order:
/// 1. `begin_line()`
/// 2. Per run: `run_info` → `commit_run_info` → `run_buffer` (shaper fills it) → `commit_run_buffer`
/// 3. `commit_line()`
struct CapturingRunHandler {
    // Reused buffers — resized in `run_buffer`, read in `commit_run_buffer`.
    current_glyphs: Vec<GlyphId>,
    current_positions: Vec<Point>,
    current_clusters: Vec<u32>,

    // Metadata stored in `run_info`, used in `commit_run_buffer`.
    current_glyph_count: usize,
    current_total_advance: f32,
    current_utf8_range_start: usize,

    /// Accumulated shaped glyph data (appended in `commit_run_buffer`).
    pub captured_glyphs: Vec<PreGlyph>,
    pub captured_total_advance: f32,
}

impl CapturingRunHandler {
    fn new(utf8_range_start: usize) -> Self {
        Self {
            current_glyphs: Vec::new(),
            current_positions: Vec::new(),
            current_clusters: Vec::new(),
            current_glyph_count: 0,
            current_total_advance: 0.0,
            current_utf8_range_start: utf8_range_start,
            captured_glyphs: Vec::new(),
            captured_total_advance: 0.0,
        }
    }
}

impl RunHandler for CapturingRunHandler {
    fn begin_line(&mut self) {}

    fn run_info(&mut self, info: &RunInfo) {
        let n = info.glyph_count;
        self.current_glyph_count = n;
        self.current_total_advance = info.advance.x;
        self.current_utf8_range_start = info.utf8_range.start;

        // Pre-allocate buffers so `run_buffer` can borrow from them.
        self.current_glyphs.resize(n, 0);
        self.current_positions.resize(n, Point::default());
        self.current_clusters.resize(n, 0);
    }

    fn commit_run_info(&mut self) {}

    fn run_buffer(&mut self, _info: &RunInfo) -> Buffer<'_> {
        // SAFETY: The skia-safe RustRunHandler bridge calls these callbacks
        // sequentially through raw pointers, so there is no actual aliasing.
        Buffer {
            glyphs: &mut self.current_glyphs,
            positions: &mut self.current_positions,
            offsets: None,
            clusters: Some(&mut self.current_clusters),
            point: Point::default(),
        }
    }

    fn commit_run_buffer(&mut self, info: &RunInfo) {
        let n = self.current_glyph_count;
        self.captured_total_advance += info.advance.x;

        // Cluster values from the Shaper are byte offsets into the utf8 slice
        // passed to `Shaper::shape` (relative to the start of that slice).
        // We adjust by `current_utf8_range_start` to make them absolute in the
        // full style-run text (the style run's own byte_start is added later).
        let base = self.current_utf8_range_start;

        for i in 0..n {
            self.captured_glyphs.push(PreGlyph {
                glyph_id: self.current_glyphs[i],
                cluster_byte: base + self.current_clusters[i] as usize,
                x: self.current_positions[i].x,
                y: self.current_positions[i].y,
            });
        }
    }

    fn commit_line(&mut self) {}
}

// ─── build_pre_shaped_text ────────────────────────────────────────────────────

/// Shape the text attached to `entity` (and any `text_span` children) into a
/// [`PreShapedText`], using the entity's CSS-resolved style properties.
///
/// This is the replacement for `build_paragraph` + `add_block`.
pub fn build_pre_shaped_text(
    entity: Entity,
    style: &mut Style,
    tree: &Tree<Entity>,
    font_collection: &mut FontCollection,
) -> Option<PreShapedText> {
    // The Skia Shaper instance.  `None` uses the platform default (HarfBuzz or CoreText).
    let shaper = Shaper::new(None);

    let base_direction_rtl =
        resolved_text_direction(style, entity) == crate::style::Direction::RightToLeft;

    let text_align = resolve_text_align(style, entity);

    let ellipsis = matches!(style.text_overflow.get(entity), Some(&TextOverflow::Ellipsis));

    let max_lines = style.line_clamp.get(entity).map(|c| c.0 as usize);

    let mut pre = PreShapedText {
        runs: Vec::new(),
        text: String::new(),
        text_align,
        base_direction_rtl,
        ellipsis,
        max_lines,
        break_opportunities: Vec::new(),
    };

    // Walk the entity and its text_span children to collect styled runs.
    add_run(&shaper, style, tree, entity, font_collection, &mut pre, base_direction_rtl);

    if pre.text.is_empty() {
        return None;
    }

    // Append zero-width space so the cursor is visible at end-of-text
    // (mirrors the `"\u{200B}"` added by the old paragraph builder).
    pre.text.push('\u{200B}');
    // Extend the last run's byte_range to include the ZWS.
    if let Some(last) = pre.runs.last_mut() {
        last.byte_range.end = pre.text.len();
    }

    // Pre-compute cluster lists and break opportunities.
    for run in &mut pre.runs {
        run.compute_clusters();
    }
    pre.compute_break_opportunities();

    Some(pre)
}

/// Recursively collect styled runs from `entity` and its `text_span` children.
fn add_run(
    shaper: &Shaper,
    style: &mut Style,
    tree: &Tree<Entity>,
    entity: Entity,
    font_collection: &mut FontCollection,
    pre: &mut PreShapedText,
    base_direction_rtl: bool,
) {
    if let Some(text) = style.text.get(entity).cloned() {
        if text.is_empty() {
            return;
        }

        // ── Resolve font ──────────────────────────────────────────────────────
        let font = resolve_font(style, entity, font_collection);

        // ── Build fill paint ──────────────────────────────────────────────────
        let mut fill_paint = Paint::default();
        if let Some(color) = style.font_color.get_resolved(entity, &style.custom_color_props) {
            fill_paint.set_color(color);
        }
        fill_paint.set_anti_alias(true);
        fill_paint.set_blend_mode(BlendMode::SrcOver);

        if let Some(stroke_width) = style.text_stroke_width.get(entity) {
            fill_paint.set_stroke_width(stroke_width.to_px().unwrap_or(0.0));
            if let Some(stroke_style) = style.text_stroke_style.get(entity) {
                fill_paint.set_style((*stroke_style).into());
            }
        }

        // ── Decoration paint ──────────────────────────────────────────────────
        let font_color =
            style.font_color.get_resolved(entity, &style.custom_color_props).unwrap_or_default();
        let dec_color =
            match style.text_decoration_color.get_resolved(entity, &style.custom_color_props) {
                Some(crate::prelude::Color::CurrentColor) | None => font_color,
                Some(c) => c,
            };
        let mut decoration_paint = Paint::default();
        decoration_paint.set_color(dec_color);
        decoration_paint.set_anti_alias(true);

        let underline = style
            .text_decoration_line
            .get(entity)
            .map(|d| d.contains(vizia_style::TextDecorationLine::Underline))
            .unwrap_or(false);
        let strikethrough = style
            .text_decoration_line
            .get(entity)
            .map(|d| d.contains(vizia_style::TextDecorationLine::Strikethrough))
            .unwrap_or(false);

        let run_paint = RunPaint { fill: fill_paint, underline, strikethrough, decoration_paint };

        // ── Get font metrics ──────────────────────────────────────────────────
        let (_, metrics) = font.metrics();
        let ascent = (-metrics.ascent).max(0.0); // ascent is negative in Skia
        let descent = metrics.descent.max(0.0);
        let leading = metrics.leading.max(0.0);

        // ── Resolve line-height ───────────────────────────────────────────────
        // If a fixed line-height is set, override ascent/descent accordingly.
        let font_size = font.size();
        let (ascent, descent, leading) = if let Some(lh) =
            style.line_height.get_resolved(entity, &style.custom_line_height_props)
        {
            match lh {
                LineHeight::Normal => (ascent, descent, leading),
                LineHeight::Number(n) => {
                    let half_extra = (font_size * n - (ascent + descent)) * 0.5;
                    (ascent + half_extra.max(0.0), descent + half_extra.max(0.0), 0.0)
                }
                LineHeight::Percentage(p) => {
                    let half_extra = (font_size * p / 100.0 - (ascent + descent)) * 0.5;
                    (ascent + half_extra.max(0.0), descent + half_extra.max(0.0), 0.0)
                }
                LineHeight::Length(len) => {
                    if let Some(px) = len.to_px() {
                        let half_extra = (px * style.scale_factor() - (ascent + descent)) * 0.5;
                        (ascent + half_extra.max(0.0), descent + half_extra.max(0.0), 0.0)
                    } else {
                        (ascent, descent, leading)
                    }
                }
            }
        } else {
            (ascent, descent, leading)
        };

        // ── Shape the text ────────────────────────────────────────────────────
        let byte_start = pre.text.len();
        let text_str = text.as_str();

        let mut handler = CapturingRunHandler::new(0);
        shaper.shape(text_str, &font, !base_direction_rtl, f32::INFINITY, &mut handler);

        // Adjust cluster bytes to be absolute in the full pre.text buffer.
        let abs_byte_start = byte_start;
        for g in &mut handler.captured_glyphs {
            g.cluster_byte += abs_byte_start;
        }

        pre.text.push_str(text_str);
        let byte_end = pre.text.len();

        // Record text_range for this entity (used by layout_span in text_layout_system).
        style.text_range.insert(entity, byte_start..byte_end);

        pre.runs.push(PreShapedRun {
            font,
            paint: run_paint,
            is_rtl: base_direction_rtl,
            byte_range: byte_start..byte_end,
            glyphs: handler.captured_glyphs,
            total_advance: handler.captured_total_advance,
            ascent,
            descent,
            leading,
            clusters: Vec::new(), // filled by compute_clusters() later
        });
    }

    // Recurse into text_span children.
    let iter = LayoutChildIterator::new(tree, entity);
    for child in iter {
        if style.text_span.get(child).copied().unwrap_or_default() {
            add_run(shaper, style, tree, child, font_collection, pre, base_direction_rtl);
        }
    }
}

// ─── Font resolution ──────────────────────────────────────────────────────────

/// Resolve a `skia_safe::Font` from the style properties of `entity`.
fn resolve_font(style: &Style, entity: Entity, font_collection: &mut FontCollection) -> Font {
    let scale = style.scale_factor();

    let font_size = style
        .font_size
        .get_resolved(entity, &style.custom_font_size_props)
        .and_then(|f| f.0.to_px())
        .unwrap_or(16.0)
        * scale;

    let weight = style.font_weight.get(entity).copied().unwrap_or_default();
    let width = style.font_width.get(entity).copied().unwrap_or_default();
    let slant = style.font_slant.get(entity).copied().unwrap_or_default();
    let font_style = FontStyle::new(weight.into(), width.into(), slant.into());

    let default_families: Vec<FamilyOwned> =
        vec![FamilyOwned::Generic(crate::prelude::GenericFontFamily::SansSerif)];
    let families: &[FamilyOwned] =
        style.font_family.get(entity).map(Vec::as_slice).unwrap_or(default_families.as_slice());

    // Try to find a typeface via FontCollection (includes custom/asset fonts).
    let typefaces = font_collection.find_typefaces(families, font_style);

    let typeface: Option<Typeface> = typefaces.into_iter().next();

    let mut font = if let Some(tf) = typeface {
        Font::new(tf, font_size)
    } else {
        let mut f = Font::default();
        f.set_size(font_size);
        f
    };

    // Apply font variation settings if present.
    if let Some(coords) = style.font_variation_settings.get(entity) {
        let coordinates: Vec<_> = coords.iter().map(|c| c.0).collect();
        let tf = font.typeface();
        let args = FontArguments::new()
            .set_variation_design_position(VariationPosition { coordinates: &coordinates });
        if let Some(new_tf) = tf.clone_with_arguments(&args) {
            font = Font::new(new_tf, font_size);
        }
    }

    // Apply letter-spacing by adjusting the font's text scale (approximate).
    // Skia doesn't have a direct per-font letter-spacing; it's applied via TextStyle
    // in the paragraph API.  For text blobs we bake it into glyph positions post-shaping.
    // (Letter-spacing is left for a follow-up pass; this function just returns the base font.)

    // Enable subpixel antialiasing.  `SubpixelAntiAlias` (LCD) gives the best quality on
    // opaque RGB surfaces (the main window); Skia automatically falls back to greyscale
    // `AntiAlias` on transparent/RGBA surfaces (e.g. compositor layers).
    font.set_edging(Edging::SubpixelAntiAlias);
    // Subpixel glyph *positioning* (fractional pixel offsets) improves spacing accuracy.
    font.set_subpixel(true);

    font
}

// ─── Text-align helpers ───────────────────────────────────────────────────────

fn resolve_text_align(style: &Style, entity: Entity) -> TextAlign {
    use crate::prelude::Alignment;

    let is_rtl = resolved_text_direction(style, entity) == crate::style::Direction::RightToLeft;

    if let Some(align) = style.text_align.get(entity).copied() {
        return flip_for_rtl(align, is_rtl);
    }

    if let Some(alignment) = style.alignment.get(entity).copied() {
        let align = match alignment {
            Alignment::TopLeft | Alignment::Left | Alignment::BottomLeft => TextAlign::Left,
            Alignment::TopCenter | Alignment::Center | Alignment::BottomCenter => TextAlign::Center,
            Alignment::TopRight | Alignment::Right | Alignment::BottomRight => TextAlign::Right,
        };
        return flip_for_rtl(align, is_rtl);
    }

    flip_for_rtl(TextAlign::Left, is_rtl)
}

fn flip_for_rtl(align: TextAlign, is_rtl: bool) -> TextAlign {
    if !is_rtl {
        return align;
    }
    match align {
        TextAlign::Left => TextAlign::Right,
        TextAlign::Right => TextAlign::Left,
        other => other,
    }
}

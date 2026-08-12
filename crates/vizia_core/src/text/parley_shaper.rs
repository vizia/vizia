//! Parley shaping entrypoint.
//!
//! This module is the migration boundary for replacing Skia-driven shaping and
//! line-breaking with Parley while preserving Skia text-blob rendering.

use parley::{
    FontFamily, FontStyle as ParleyFontStyle, FontWeight as ParleyFontWeight,
    FontWidth as ParleyFontWidth, Layout, LineHeight as ParleyLineHeight, PositionedLayoutItem,
    StyleProperty, TextWrapMode,
};
use skia_safe::{
    BlendMode, Font, FontArguments, FontStyle, Paint, Typeface, font::Edging,
    font_arguments::VariationPosition, textlayout::FontCollection,
};
use vizia_storage::LayoutChildIterator;
use vizia_style::TextAlign;

use crate::{
    entity::Entity,
    prelude::{FamilyOwned, LineHeight},
    style::Style,
    text::{
        TextContext, resolved_text_direction,
        shaped_text::{PreGlyph, PreShapedRun, PreShapedText, RunPaint},
    },
    tree::Tree,
};

struct ParleyRunStyle {
    family_css: String,
    font_size: f32,
    font_weight: ParleyFontWeight,
    font_width: ParleyFontWidth,
    font_style: ParleyFontStyle,
    line_height: ParleyLineHeight,
    letter_spacing: f32,
    text_wrap_mode: TextWrapMode,
}

fn generic_family_name(family: crate::prelude::GenericFontFamily) -> &'static str {
    match family {
        crate::prelude::GenericFontFamily::Serif => "serif",
        crate::prelude::GenericFontFamily::SansSerif => "sans-serif",
        crate::prelude::GenericFontFamily::Cursive => "cursive",
        crate::prelude::GenericFontFamily::Fantasy => "fantasy",
        crate::prelude::GenericFontFamily::Monospace => "monospace",
    }
}

fn family_list_to_css(families: &[FamilyOwned]) -> String {
    families
        .iter()
        .map(|family| match family {
            FamilyOwned::Generic(generic) => generic_family_name(*generic).to_string(),
            FamilyOwned::Named(name) => format!("\"{}\"", name.replace('"', "\\\"")),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn parley_font_width(width: vizia_style::FontWidth) -> ParleyFontWidth {
    match width {
        vizia_style::FontWidth::UltraCondensed => ParleyFontWidth::ULTRA_CONDENSED,
        vizia_style::FontWidth::ExtraCondensed => ParleyFontWidth::EXTRA_CONDENSED,
        vizia_style::FontWidth::Condensed => ParleyFontWidth::CONDENSED,
        vizia_style::FontWidth::SemiCondensed => ParleyFontWidth::SEMI_CONDENSED,
        vizia_style::FontWidth::Normal => ParleyFontWidth::NORMAL,
        vizia_style::FontWidth::SemiExpanded => ParleyFontWidth::SEMI_EXPANDED,
        vizia_style::FontWidth::Expanded => ParleyFontWidth::EXPANDED,
        vizia_style::FontWidth::ExtraExpanded => ParleyFontWidth::EXTRA_EXPANDED,
        vizia_style::FontWidth::UltraExpanded => ParleyFontWidth::ULTRA_EXPANDED,
    }
}

fn parley_font_style(slant: vizia_style::FontSlant) -> ParleyFontStyle {
    match slant {
        vizia_style::FontSlant::Normal => ParleyFontStyle::Normal,
        vizia_style::FontSlant::Italic => ParleyFontStyle::Italic,
        vizia_style::FontSlant::Oblique => ParleyFontStyle::Oblique(Some(14.0)),
    }
}

fn parley_line_height(style: &Style, entity: Entity, font_size: f32) -> ParleyLineHeight {
    match style.line_height.get_resolved(entity, &style.custom_line_height_props) {
        Some(LineHeight::Normal) | None => ParleyLineHeight::MetricsRelative(1.0),
        Some(LineHeight::Number(n)) => ParleyLineHeight::FontSizeRelative(n),
        Some(LineHeight::Percentage(p)) => ParleyLineHeight::FontSizeRelative(p / 100.0),
        Some(LineHeight::Length(len)) => {
            let absolute = len.to_px().unwrap_or(font_size) * style.scale_factor();
            ParleyLineHeight::Absolute(absolute)
        }
    }
}

fn parley_letter_spacing(style: &Style, entity: Entity) -> f32 {
    match style.letter_spacing.get_resolved(entity, &style.custom_letter_spacing_props) {
        Some(vizia_style::LetterSpacing::Length(length)) => {
            length.to_px().unwrap_or(0.0) * style.scale_factor()
        }
        _ => 0.0,
    }
}

fn parley_run_style(style: &Style, entity: Entity, font_size: f32) -> ParleyRunStyle {
    let default_families: Vec<FamilyOwned> =
        vec![FamilyOwned::Generic(crate::prelude::GenericFontFamily::SansSerif)];
    let families: &[FamilyOwned] =
        style.font_family.get(entity).map(Vec::as_slice).unwrap_or(default_families.as_slice());

    let family_css = family_list_to_css(families);
    let font_weight =
        ParleyFontWeight::new(style.font_weight.get(entity).copied().unwrap_or_default().0 as f32);
    let font_width = parley_font_width(style.font_width.get(entity).copied().unwrap_or_default());
    let font_style = parley_font_style(style.font_slant.get(entity).copied().unwrap_or_default());
    let line_height = parley_line_height(style, entity, font_size);
    let letter_spacing = parley_letter_spacing(style, entity);
    let text_wrap_mode = if style.text_wrap.get(entity).copied().unwrap_or(true) {
        TextWrapMode::Wrap
    } else {
        TextWrapMode::NoWrap
    };

    ParleyRunStyle {
        family_css,
        font_size,
        font_weight,
        font_width,
        font_style,
        line_height,
        letter_spacing,
        text_wrap_mode,
    }
}

fn build_parley_layout(
    entity: Entity,
    style: &Style,
    text: &str,
    text_context: &mut TextContext,
) -> Layout<[u8; 4]> {
    let font_size = style
        .font_size
        .get_resolved(entity, &style.custom_font_size_props)
        .and_then(|size| size.0.to_px())
        .unwrap_or(16.0);
    let run_style = parley_run_style(style, entity, font_size);

    let mut builder = text_context.parley_layout_context.ranged_builder(
        &mut text_context.parley_font_context,
        text,
        style.scale_factor(),
        true,
    );

    builder.push_default(StyleProperty::FontSize(run_style.font_size));
    builder
        .push_default(StyleProperty::FontFamily(FontFamily::from(run_style.family_css.as_str())));
    builder.push_default(StyleProperty::FontWeight(run_style.font_weight));
    builder.push_default(StyleProperty::FontWidth(run_style.font_width));
    builder.push_default(StyleProperty::FontStyle(run_style.font_style));
    builder.push_default(StyleProperty::LineHeight(run_style.line_height));
    builder.push_default(StyleProperty::LetterSpacing(run_style.letter_spacing));
    builder.push_default(StyleProperty::TextWrapMode(run_style.text_wrap_mode));

    let mut layout: Layout<[u8; 4]> = builder.build(text);
    layout.break_all_lines(None);
    layout
}

/// A group of glyphs that were all shaped using the same physical font (as chosen by
/// Parley's fontique-based script/font fallback), along with the exact Skia [`Font`]
/// built from that same font's raw data.
struct ShapedFontRun {
    font: Font,
    glyphs: Vec<PreGlyph>,
    total_advance: f32,
}

/// Resolves (and caches) a Skia [`Typeface`] built directly from the raw font bytes that
/// Parley/fontique selected for a shaped run. Building the typeface from the exact same
/// data (rather than re-resolving a typeface from the CSS family list via Skia's own font
/// manager) guarantees that the glyph ids produced by shaping refer to glyphs that exist
/// in the typeface used for drawing. This matters most for script fallback (e.g. Arabic,
/// CJK) where Parley may pick a different physical font than Skia's family-name matching
/// would, which previously caused shaped glyph ids to be drawn against the wrong font and
/// render as incorrect/garbled glyphs.
fn typeface_for_font_data(
    text_context: &mut TextContext,
    font_data: &parley::FontData,
) -> Option<Typeface> {
    let key = (font_data.data.id(), font_data.index);
    if let Some(typeface) = text_context.typeface_cache.get(&key) {
        return Some(typeface.clone());
    }

    let typeface = text_context
        .default_font_manager
        .new_from_data(font_data.data.data(), font_data.index as usize)?;
    text_context.typeface_cache.insert(key, typeface.clone());
    Some(typeface)
}

fn shape_run_with_parley(
    text_context: &mut TextContext,
    run_text: &str,
    byte_start: usize,
    run_style: &ParleyRunStyle,
    fallback_font: &Font,
) -> Vec<ShapedFontRun> {
    let mut builder = text_context.parley_layout_context.ranged_builder(
        &mut text_context.parley_font_context,
        run_text,
        1.0,
        false,
    );

    builder
        .push_default(StyleProperty::FontFamily(FontFamily::from(run_style.family_css.as_str())));
    builder.push_default(StyleProperty::FontSize(run_style.font_size));
    builder.push_default(StyleProperty::FontWeight(run_style.font_weight));
    builder.push_default(StyleProperty::FontWidth(run_style.font_width));
    builder.push_default(StyleProperty::FontStyle(run_style.font_style));
    builder.push_default(StyleProperty::LineHeight(run_style.line_height));
    builder.push_default(StyleProperty::LetterSpacing(run_style.letter_spacing));
    builder.push_default(StyleProperty::TextWrapMode(run_style.text_wrap_mode));

    let mut layout: Layout<[u8; 4]> = builder.build(run_text);
    layout.break_all_lines(None);

    let mut groups: Vec<ShapedFontRun> = Vec::new();
    let mut running_x = 0.0f32;

    for line in layout.lines() {
        for item in line.items() {
            if let PositionedLayoutItem::GlyphRun(glyph_run) = item {
                let run = glyph_run.run();
                let font = typeface_for_font_data(text_context, run.font())
                    .map(|typeface| Font::new(typeface, run.font_size()))
                    .unwrap_or_else(|| fallback_font.clone());

                let mut glyphs = Vec::new();
                for cluster in run.visual_clusters() {
                    let cluster_range = cluster.text_range();
                    let cluster_byte = byte_start + cluster_range.start;
                    let cluster_x = running_x;
                    for glyph in cluster.glyphs() {
                        // Skia glyph IDs are u16; skip unrepresentable ids to avoid truncation.
                        if glyph.id > u16::MAX as u32 {
                            continue;
                        }

                        glyphs.push(PreGlyph {
                            glyph_id: glyph.id as u16,
                            cluster_byte,
                            x: cluster_x + glyph.x,
                        });
                    }

                    running_x += cluster.advance();
                }

                if !glyphs.is_empty() {
                    groups.push(ShapedFontRun { font, glyphs, total_advance: running_x });
                }
            }
        }
    }

    groups
}

fn build_run_paint(style: &Style, entity: Entity) -> RunPaint {
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

    RunPaint { fill: fill_paint, underline, strikethrough, decoration_paint }
}

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

    let typefaces = font_collection.find_typefaces(families, font_style);
    let typeface: Option<Typeface> = typefaces.into_iter().next();

    let mut font = if let Some(tf) = typeface {
        Font::new(tf, font_size)
    } else {
        let mut f = Font::default();
        f.set_size(font_size);
        f
    };

    if let Some(coords) = style.font_variation_settings.get(entity) {
        let coordinates: Vec<_> = coords.iter().map(|c| c.0).collect();
        let tf = font.typeface();
        let args = FontArguments::new()
            .set_variation_design_position(VariationPosition { coordinates: &coordinates });
        if let Some(new_tf) = tf.clone_with_arguments(&args) {
            font = Font::new(new_tf, font_size);
        }
    }

    font.set_edging(Edging::SubpixelAntiAlias);
    font.set_subpixel(true);

    font
}

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

/// Resolves the visual text alignment for `entity` (already RTL-flipped, matching
/// [`resolve_text_align`]) into a [`parley::Alignment`] suitable for [`parley::editing::PlainEditor::set_alignment`].
///
/// This is only needed so that Parley's own hit-testing/caret-geometry (which is computed
/// against the editor's internal layout) agrees with the manually-computed visual offsets
/// used by the Skia draw path (see `compute_line_x_offset` in `context/text_draw_helpers.rs`).
pub(crate) fn resolve_parley_alignment(style: &Style, entity: Entity) -> parley::Alignment {
    // Mirrors the (pre-existing) double-flip semantics of `resolve_text_align` +
    // `compute_line_x_offset` so hit-testing agrees with what is actually drawn.
    let is_rtl = resolved_text_direction(style, entity) == crate::style::Direction::RightToLeft;
    let align = resolve_text_align(style, entity);
    let effective = if is_rtl {
        match align {
            TextAlign::Left => TextAlign::Right,
            TextAlign::Right => TextAlign::Left,
            other => other,
        }
    } else {
        align
    };

    match effective {
        TextAlign::Right => parley::Alignment::Right,
        TextAlign::Center => parley::Alignment::Center,
        _ => parley::Alignment::Left,
    }
}

struct PreShapedAccumulator {
    runs: Vec<PreShapedRun>,
    text: String,
}

fn add_run(
    style: &mut Style,
    tree: &Tree<Entity>,
    entity: Entity,
    text_context: &mut TextContext,
    acc: &mut PreShapedAccumulator,
    base_direction_rtl: bool,
) {
    if let Some(text) = style.text.get(entity).cloned() {
        if !text.is_empty() {
            let fallback_font = resolve_font(style, entity, &mut text_context.font_collection);
            let run_paint = build_run_paint(style, entity);

            let byte_start = acc.text.len();
            acc.text.push_str(text.as_str());
            let byte_end = acc.text.len();
            style.text_range.insert(entity, byte_start..byte_end);

            let run_style = parley_run_style(style, entity, fallback_font.size());
            let font_runs = shape_run_with_parley(
                text_context,
                text.as_str(),
                byte_start,
                &run_style,
                &fallback_font,
            );

            if font_runs.is_empty() {
                acc.runs.push(PreShapedRun {
                    font: fallback_font,
                    paint: run_paint,
                    byte_range: byte_start..byte_end,
                    glyphs: Vec::new(),
                    total_advance: 0.0,
                });
            } else {
                for font_run in font_runs {
                    acc.runs.push(PreShapedRun {
                        font: font_run.font,
                        paint: run_paint.clone(),
                        byte_range: byte_start..byte_end,
                        glyphs: font_run.glyphs,
                        total_advance: font_run.total_advance,
                    });
                }
            }
        }
    }

    let iter = LayoutChildIterator::new(tree, entity);
    for child in iter {
        if style.text_span.get(child).copied().unwrap_or_default() {
            add_run(style, tree, child, text_context, acc, base_direction_rtl);
        }
    }
}

/// Build a [`PreShapedText`] directly from a `PlainEditor`'s own up-to-date parley
/// [`Layout`], for glyph drawing only. Selection/caret geometry should be queried
/// directly from the editor (`selection_geometry_with`/`cursor_geometry`) instead of
/// through this bridge, since the editor already owns an authoritative layout.
///
/// This performs a single pass over the editor's already-broken lines instead of
/// re-shaping the text a second time via [`build_pre_shaped_text`].
pub(crate) fn pre_shaped_from_editor_layout(
    entity: Entity,
    style: &Style,
    text: &str,
    layout: &Layout<[u8; 4]>,
    text_context: &mut TextContext,
) -> PreShapedText {
    let base_direction_rtl =
        resolved_text_direction(style, entity) == crate::style::Direction::RightToLeft;
    let text_align = resolve_text_align(style, entity);

    let fallback_font = resolve_font(style, entity, &mut text_context.font_collection);
    let paint = build_run_paint(style, entity);

    let mut runs: Vec<PreShapedRun> = Vec::new();
    let mut running_x = 0.0f32;

    for line in layout.lines() {
        for item in line.items() {
            if let PositionedLayoutItem::GlyphRun(glyph_run) = item {
                let run = glyph_run.run();
                let font = typeface_for_font_data(text_context, run.font())
                    .map(|typeface| Font::new(typeface, run.font_size()))
                    .unwrap_or_else(|| fallback_font.clone());

                let mut glyphs = Vec::new();
                for cluster in run.visual_clusters() {
                    let cluster_range = cluster.text_range();
                    let cluster_byte = cluster_range.start;
                    let cluster_x = running_x;
                    for glyph in cluster.glyphs() {
                        // Skia glyph IDs are u16; skip unrepresentable ids to avoid truncation.
                        if glyph.id > u16::MAX as u32 {
                            continue;
                        }

                        glyphs.push(PreGlyph {
                            glyph_id: glyph.id as u16,
                            cluster_byte,
                            x: cluster_x + glyph.x,
                        });
                    }

                    running_x += cluster.advance();
                }

                if !glyphs.is_empty() {
                    runs.push(PreShapedRun {
                        font,
                        paint: paint.clone(),
                        byte_range: 0..text.len(),
                        glyphs,
                        total_advance: running_x,
                    });
                }
            }
        }
    }

    PreShapedText {
        runs,
        text: text.to_string(),
        text_align,
        base_direction_rtl,
        max_lines: None,
        parley_layout: layout.clone(),
    }
}

/// Populate a `PlainEditor`'s style set from an entity's resolved CSS style.
pub(crate) fn apply_editor_style(
    entity: Entity,
    style: &Style,
    editor: &mut parley::editing::PlainEditor<[u8; 4]>,
) {
    let font_size = style
        .font_size
        .get_resolved(entity, &style.custom_font_size_props)
        .and_then(|size| size.0.to_px())
        .unwrap_or(16.0);
    let run_style = parley_run_style(style, entity, font_size);

    let family: FontFamily<'static> = FontFamily::from(run_style.family_css.as_str()).into_owned();

    let styles = editor.edit_styles();
    styles.insert(StyleProperty::FontSize(run_style.font_size));
    styles.insert(StyleProperty::FontFamily(family));
    styles.insert(StyleProperty::FontWeight(run_style.font_weight));
    styles.insert(StyleProperty::FontWidth(run_style.font_width));
    styles.insert(StyleProperty::FontStyle(run_style.font_style));
    styles.insert(StyleProperty::LineHeight(run_style.line_height));
    styles.insert(StyleProperty::LetterSpacing(run_style.letter_spacing));
    styles.insert(StyleProperty::TextWrapMode(run_style.text_wrap_mode));
}

/// Build pre-shaped text for an entity.
pub fn build_pre_shaped_text(
    entity: Entity,
    style: &mut Style,
    tree: &Tree<Entity>,
    text_context: &mut TextContext,
) -> PreShapedText {
    let base_direction_rtl =
        resolved_text_direction(style, entity) == crate::style::Direction::RightToLeft;

    let text_align = resolve_text_align(style, entity);
    let max_lines = style.line_clamp.get(entity).map(|c| c.0 as usize);

    let mut acc = PreShapedAccumulator { runs: Vec::new(), text: String::new() };

    add_run(style, tree, entity, text_context, &mut acc, base_direction_rtl);

    acc.text.push('\u{200B}');
    if let Some(last) = acc.runs.last_mut() {
        last.byte_range.end = acc.text.len();
    }

    let parley_layout = build_parley_layout(entity, style, &acc.text, text_context);

    PreShapedText {
        runs: acc.runs,
        text: acc.text,
        text_align,
        base_direction_rtl,
        max_lines,
        parley_layout,
    }
}

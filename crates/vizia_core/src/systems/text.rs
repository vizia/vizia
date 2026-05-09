use skia_safe::{
    BlendMode, FontArguments, FontStyle, Paint,
    font_arguments::VariationPosition,
    textlayout::{
        FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, RectHeightStyle,
        RectWidthStyle, TextDirection, TextStyle,
    },
};
use vizia_storage::{LayoutChildIterator, LayoutTreeIterator};

use crate::text::resolved_text_direction;
use crate::{cache::CachedData, prelude::*};

pub(crate) fn text_system(cx: &mut Context) {
    if cx.style.text.is_empty() || cx.style.text_construction.is_empty() {
        return;
    }

    let iterator = LayoutTreeIterator::full(&cx.tree);
    for entity in iterator {
        if !cx.style.text_construction.contains(&entity) {
            continue;
        }

        if cx.style.text.contains(entity)
            && cx.style.display.get(entity).copied().unwrap_or_default() != Display::None
        {
            if let Some(paragraph) =
                build_paragraph(entity, &mut cx.style, &cx.tree, cx.text_context.font_collection())
            {
                cx.text_context.text_paragraphs.insert(entity, paragraph);
                cx.style.needs_relayout();
                cx.style.needs_text_layout(entity);
            }
        }
    }

    cx.style.text_construction.clear();
}

pub(crate) fn text_layout_system(cx: &mut Context) {
    if cx.style.text_layout.is_empty() {
        return;
    }

    let iterator = LayoutTreeIterator::full(&cx.tree);
    let mut redraw_entities = Vec::new();
    for entity in iterator {
        if !cx.style.text_layout.contains(&entity) {
            continue;
        }

        if let Some(paragraph) = cx.text_context.text_paragraphs.get_mut(entity) {
            let bounds = cx.cache.get_bounds(entity);
            let mut padding_left = cx
                .style
                .padding_left
                .get(entity)
                .copied()
                .unwrap_or_default()
                .to_px(bounds.width(), 0.0)
                * cx.style.scale_factor();
            let mut padding_right = cx
                .style
                .padding_right
                .get(entity)
                .copied()
                .unwrap_or_default()
                .to_px(bounds.width(), 0.0)
                * cx.style.scale_factor();
            let padding_top = cx
                .style
                .padding_top
                .get(entity)
                .copied()
                .unwrap_or_default()
                .to_px(bounds.width(), 0.0)
                * cx.style.scale_factor();

            if resolved_text_direction(&cx.style, entity) == Direction::RightToLeft {
                std::mem::swap(&mut padding_left, &mut padding_right);
            }

            let text_bounds = BoundingBox {
                x: padding_left,
                y: 0.0,
                w: bounds.w - padding_left - padding_right,
                h: bounds.h,
            };

            if !cx
                .style
                .width
                .get_resolved(entity, &cx.style.custom_units_props)
                .unwrap_or_default()
                .is_auto()
                && !cx
                    .style
                    .height
                    .get_resolved(entity, &cx.style.custom_units_props)
                    .unwrap_or_default()
                    .is_auto()
            {
                if cx.style.text_overflow.get(entity).copied().unwrap_or_default()
                    == TextOverflow::Clip
                {
                    paragraph.layout(f32::MAX);
                    paragraph
                        .layout(text_bounds.width().max(paragraph.min_intrinsic_width() + 1.0));
                    let mut text_bounds = text_bounds;
                    text_bounds.w = paragraph.max_intrinsic_width();
                    cx.text_context.text_bounds.insert(entity, text_bounds);
                } else {
                    paragraph.layout(text_bounds.width());
                    let mut text_bounds = bounds;
                    text_bounds.w = paragraph.max_intrinsic_width();
                    cx.text_context.text_bounds.insert(entity, text_bounds);
                }
            } else {
                // For auto-sized text views, re-layout at the final constrained width
                // so constraints like min-width affect line breaking and text bounds.
                let final_text_width = text_bounds.width();
                paragraph.layout(final_text_width);

                if let Some(stored_text_bounds) = cx.text_context.text_bounds.get_mut(entity) {
                    stored_text_bounds.x = bounds.x + padding_left;
                    stored_text_bounds.y = bounds.y + padding_top;
                    stored_text_bounds.w = final_text_width;
                    stored_text_bounds.h = paragraph.height();
                }
            }

            layout_span(&cx.style, &mut cx.cache, &cx.tree, entity, paragraph, bounds);

            redraw_entities.push(entity);
        }
    }
    for entity in redraw_entities {
        cx.needs_redraw(entity);
    }
    cx.style.text_layout.clear();
}

pub fn layout_span(
    style: &Style,
    cache: &mut CachedData,
    tree: &Tree<Entity>,
    entity: Entity,
    paragraph: &Paragraph,
    paragraph_bounds: BoundingBox,
) -> BoundingBox {
    let mut bounds = BoundingBox::default();
    if style.text_span.get(entity).copied().unwrap_or_default() {
        if let Some(range) = style.text_range.get(entity) {
            let rects = paragraph.get_rects_for_range(
                range.clone(),
                RectHeightStyle::Tight,
                RectWidthStyle::Tight,
            );

            let min_x = rects.iter().fold(1000000.0f32, |min, item| min.min(item.rect.x()));
            let min_y = rects.iter().fold(1000000.0f32, |min, item| min.min(item.rect.y()));
            let max_x = rects.iter().fold(0.0f32, |max, item| max.max(item.rect.right()));
            let max_y = rects.iter().fold(0.0f32, |max, item| max.max(item.rect.bottom()));

            bounds = BoundingBox::from_min_max(min_x, min_y, max_x, max_y);
        }
    }

    let iter = LayoutChildIterator::new(tree, entity);
    for child in iter {
        if bounds.width() == 0.0 && bounds.height() == 0.0 {
            bounds = layout_span(style, cache, tree, child, paragraph, paragraph_bounds);
        } else {
            bounds =
                bounds.union(&layout_span(style, cache, tree, child, paragraph, paragraph_bounds));
        }
    }

    if style.text_span.get(entity).copied().unwrap_or_default() {
        cache.bounds.insert(
            entity,
            BoundingBox::from_min_max(
                paragraph_bounds.x + bounds.x,
                paragraph_bounds.y + bounds.y,
                paragraph_bounds.x + bounds.right(),
                paragraph_bounds.y + bounds.bottom(),
            ),
        );
    }

    bounds
}

pub fn build_paragraph(
    entity: Entity,
    style: &mut Style,
    tree: &Tree<Entity>,
    font_collection: &FontCollection,
) -> Option<Paragraph> {
    let mut paragraph_style = ParagraphStyle::default();
    // paragraph_style.turn_hinting_off();

    // For fixed line-height lengths, use paragraph struts to enforce absolute line height.
    if let Some(LineHeight::Length(length)) =
        style.line_height.get_resolved(entity, &style.custom_line_height_props)
    {
        if let Some(line_height_px) = length.to_px() {
            if line_height_px > 0.0 {
                let mut strut_style = skia_safe::textlayout::StrutStyle::new();
                strut_style
                    .set_strut_enabled(true)
                    .set_force_strut_height(true)
                    .set_height_override(true)
                    .set_height(1.0)
                    .set_font_size(line_height_px * style.scale_factor());
                paragraph_style.set_strut_style(strut_style);
            }
        }
    }

    // Overflow
    match style.text_overflow.get(entity) {
        Some(&TextOverflow::Ellipsis) => {
            paragraph_style.set_ellipsis("…");
        }

        Some(&TextOverflow::Clip) => {
            paragraph_style.set_ellipsis("");
        }

        _ => {
            paragraph_style.set_ellipsis("");
        }
    }

    // Line Clamp
    if let Some(line_clamp) = style.line_clamp.get(entity) {
        paragraph_style.set_max_lines(line_clamp.0 as usize);
    }

    // Text Align
    paragraph_style.set_text_align(resolve_text_align(style, entity).into());

    // Text Direction
    paragraph_style.set_text_direction(
        if resolved_text_direction(style, entity) == Direction::RightToLeft {
            TextDirection::RTL
        } else {
            TextDirection::LTR
        },
    );

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

    add_block(style, tree, entity, &mut paragraph_builder, &mut 0);

    paragraph_builder.add_text("\u{200B}");
    paragraph_builder.build().into()
}

fn resolve_text_align(style: &Style, entity: Entity) -> TextAlign {
    let is_rtl = resolved_text_direction(style, entity) == Direction::RightToLeft;

    if let Some(text_align) = style.text_align.get(entity).copied() {
        return flip_text_align_for_rtl(text_align, is_rtl);
    }

    if let Some(alignment) = style.alignment.get(entity).copied() {
        let alignment = flip_alignment_for_rtl(alignment, is_rtl);
        return match alignment {
            Alignment::TopLeft | Alignment::Left | Alignment::BottomLeft => TextAlign::Left,
            Alignment::TopCenter | Alignment::Center | Alignment::BottomCenter => TextAlign::Center,
            Alignment::TopRight | Alignment::Right | Alignment::BottomRight => TextAlign::Right,
        };
    }

    flip_text_align_for_rtl(TextAlign::Left, is_rtl)
}

fn flip_text_align_for_rtl(text_align: TextAlign, is_rtl: bool) -> TextAlign {
    if !is_rtl {
        return text_align;
    }

    match text_align {
        TextAlign::Left => TextAlign::Right,
        TextAlign::Right => TextAlign::Left,
        _ => text_align,
    }
}

fn flip_alignment_for_rtl(alignment: Alignment, is_rtl: bool) -> Alignment {
    if !is_rtl {
        return alignment;
    }

    match alignment {
        Alignment::TopLeft => Alignment::TopRight,
        Alignment::Left => Alignment::Right,
        Alignment::BottomLeft => Alignment::BottomRight,
        Alignment::TopRight => Alignment::TopLeft,
        Alignment::Right => Alignment::Left,
        Alignment::BottomRight => Alignment::BottomLeft,
        _ => alignment,
    }
}

fn add_block(
    style: &mut Style,
    tree: &Tree<Entity>,
    entity: Entity,
    paragraph_builder: &mut ParagraphBuilder,
    current: &mut usize,
) {
    // let mut new_current = current;

    if let Some(text) = style.text.get(entity) {
        if !text.is_empty() {
            // Text Style

            let mut text_style = TextStyle::new();

            let font_color = style
                .font_color
                .get_resolved(entity, &style.custom_color_props)
                .unwrap_or_default();

            if let Some(text_decoration_line) = style.text_decoration_line.get(entity).copied() {
                text_style.set_decoration_type(text_decoration_line.into());
                text_style.set_decoration_color(font_color);
            }

            // Font Families
            text_style.set_font_families(
                style
                    .font_family
                    .get(entity)
                    .map(Vec::as_slice)
                    .unwrap_or(&[FamilyOwned::Generic(GenericFontFamily::SansSerif)]),
            );

            let mut paint = Paint::default();
            // Font Color
            if let Some(font_color) =
                style.font_color.get_resolved(entity, &style.custom_color_props)
            {
                paint.set_color(font_color);
                paint.set_anti_alias(false);
                paint.set_blend_mode(BlendMode::SrcOver);
            }

            if let Some(text_stroke) = style.text_stroke_width.get(entity) {
                paint.set_stroke_width(text_stroke.to_px().unwrap_or(0.0));
                paint.set_style(
                    (*style.text_stroke_style.get(entity).unwrap_or(&TextStrokeStyle::default()))
                        .into(),
                );
            }

            text_style.set_foreground_paint(&paint);

            if let Some(background_color) = style.background_color.get(entity) {
                if style.text_span.get(entity).is_some() {
                    let mut paint = Paint::default();
                    paint.set_color(*background_color);
                    paint.set_anti_alias(false);
                    paint.set_blend_mode(BlendMode::SrcOver);
                    text_style.set_background_paint(&paint);
                }
            }

            // Font Size
            let font_size = style
                .font_size
                .get_resolved(entity, &style.custom_font_size_props)
                .and_then(|f| f.0.to_px())
                .unwrap_or(16.0);
            text_style.set_font_size(font_size * style.scale_factor());

            if let Some(line_height) =
                style.line_height.get_resolved(entity, &style.custom_line_height_props)
            {
                let height = match line_height {
                    LineHeight::Normal => None,
                    LineHeight::Number(number) => Some(number),
                    LineHeight::Percentage(percentage) => Some(percentage / 100.0),
                    LineHeight::Length(length) => length
                        .to_px()
                        .and_then(|pixels| (font_size > 0.0).then_some(pixels / font_size)),
                };

                if let Some(height) = height {
                    text_style.set_height_override(true);
                    text_style.set_height(height);
                }
            }

            // Font Style
            match (
                style.font_weight.get(entity),
                style.font_width.get(entity),
                style.font_slant.get(entity),
            ) {
                (None, None, None) => {}
                (weight, width, slant) => {
                    text_style.set_font_style(FontStyle::new(
                        weight.copied().unwrap_or_default().into(),
                        width.copied().unwrap_or_default().into(),
                        slant.copied().unwrap_or_default().into(),
                    ));
                }
            }

            // Font Variations
            if let Some(coordinates) = style.font_variation_settings.get(entity) {
                let coordinates = coordinates.iter().map(|c| c.0).collect::<Vec<_>>();
                text_style.set_font_arguments(&FontArguments::new().set_variation_design_position(
                    VariationPosition { coordinates: &coordinates },
                ));
            }

            paragraph_builder.push_style(&text_style);
            style.text_range.insert(entity, *current..*current + text.len());
            paragraph_builder.add_text(text.as_str());
            *current += text.len();
        }
    }

    let iter = LayoutChildIterator::new(tree, entity);
    for child in iter {
        if style.text_span.get(child).copied().unwrap_or_default() {
            add_block(style, tree, child, paragraph_builder, current);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rtl_flips_explicit_left_right_text_align() {
        assert_eq!(flip_text_align_for_rtl(TextAlign::Left, true), TextAlign::Right);
        assert_eq!(flip_text_align_for_rtl(TextAlign::Right, true), TextAlign::Left);
        assert_eq!(flip_text_align_for_rtl(TextAlign::Center, true), TextAlign::Center);
    }

    #[test]
    fn rtl_flips_horizontal_alignment_variants() {
        assert_eq!(flip_alignment_for_rtl(Alignment::TopLeft, true), Alignment::TopRight);
        assert_eq!(flip_alignment_for_rtl(Alignment::Left, true), Alignment::Right);
        assert_eq!(flip_alignment_for_rtl(Alignment::BottomLeft, true), Alignment::BottomRight);
        assert_eq!(flip_alignment_for_rtl(Alignment::TopRight, true), Alignment::TopLeft);
        assert_eq!(flip_alignment_for_rtl(Alignment::Right, true), Alignment::Left);
        assert_eq!(flip_alignment_for_rtl(Alignment::BottomRight, true), Alignment::BottomLeft);
        assert_eq!(flip_alignment_for_rtl(Alignment::Center, true), Alignment::Center);
    }
}

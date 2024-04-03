use skia_safe::{
    font_arguments::VariationPosition,
    textlayout::{FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextStyle},
    FontArguments, FontStyle, Paint,
};
use vizia_storage::LayoutTreeIterator;

use crate::prelude::*;

pub(crate) fn text_system(cx: &mut Context) {
    let iterator = LayoutTreeIterator::full(&cx.tree);
    for entity in iterator {
        if !cx.style.text_construction.contains(entity) {
            continue;
        }

        if let Some(paragraph) =
            build_paragraph(entity, &cx.style, cx.text_context.font_collection())
        {
            cx.text_context.text_paragraphs.insert(entity, paragraph);
            cx.style.needs_text_layout(entity);
        }
    }

    cx.style.text_construction.clear();
}

pub(crate) fn text_layout_system(cx: &mut Context) {
    let iterator = LayoutTreeIterator::full(&cx.tree);
    for entity in iterator {
        if !cx.style.text_layout.contains(entity) {
            continue;
        }

        if let Some(paragraph) = cx.text_context.text_paragraphs.get_mut(entity) {
            let bounds = cx.cache.get_bounds(entity);
            let padding_left = cx
                .style
                .child_left
                .get(entity)
                .copied()
                .unwrap_or_default()
                .to_px(bounds.width(), 0.0)
                * cx.style.scale_factor();
            let padding_right = cx
                .style
                .child_right
                .get(entity)
                .copied()
                .unwrap_or_default()
                .to_px(bounds.width(), 0.0)
                * cx.style.scale_factor();
            let text_bounds = cx
                .text_context
                .text_bounds
                .get(entity)
                .copied()
                .unwrap_or(bounds.shrink_sides(padding_left, 0.0, padding_right, 0.0));

            if !cx.style.width.get(entity).copied().unwrap_or_default().is_auto() {
                // paragraph.layout(text_bounds.width());
                paragraph.layout(bounds.width() - padding_left - padding_right);
            }

            cx.style.needs_redraw(entity);
        }
    }

    cx.style.text_layout.clear();
}

pub fn build_paragraph(
    entity: Entity,
    style: &Style,
    font_collection: &FontCollection,
) -> Option<Paragraph> {
    let text = style.text.get(entity)?;

    let mut paragraph_style = ParagraphStyle::default();
    // paragraph_style.turn_hinting_off();

    // Overflow
    if style.text_overflow.get(entity) == Some(&TextOverflow::Ellipsis) {
        paragraph_style.set_ellipsis("...");
    }

    // Line Clamp
    if let Some(line_clamp) = style.line_clamp.get(entity) {
        paragraph_style.set_max_lines(line_clamp.0 as usize);
    }

    // Text Align
    paragraph_style.set_text_align(
        if let Some(text_align) = style.text_align.get(entity) {
            *text_align
        } else if let Some(Units::Stretch(_)) = style.child_left.get(entity) {
            if let Some(Units::Stretch(_)) = style.child_right.get(entity) {
                TextAlign::Center
            } else {
                TextAlign::Right
            }
        } else {
            TextAlign::Left
        }
        .into(),
    );

    // Text Style
    paragraph_style.set_text_style(&{
        let mut text_style = TextStyle::new();

        // Font Families
        text_style.set_font_families(
            style
                .font_family
                .get(entity)
                .map(Vec::as_slice)
                .unwrap_or(&[FamilyOwned::Generic(GenericFontFamily::SansSerif)]),
        );

        // Font Color
        if let Some(font_color) = style.font_color.get(entity) {
            let mut paint = Paint::default();
            paint.set_color(*font_color);
            paint.set_anti_alias(true);
            text_style.set_foreground_paint(&paint);
        }

        // Font Size
        let font_size = style.font_size.get(entity).map_or(16.0, |f| f.0);
        text_style.set_font_size(font_size.round() * style.scale_factor());

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
            text_style.set_font_arguments(
                &FontArguments::new()
                    .set_variation_design_position(VariationPosition { coordinates }),
            );
        }

        text_style
    });

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
    paragraph_builder.add_text(text.as_str());
    paragraph_builder.add_text("\u{200B}");
    paragraph_builder.build().into()
}

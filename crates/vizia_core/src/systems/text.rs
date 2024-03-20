use skia_safe::font_arguments::variation_position::Coordinate;
use skia_safe::font_arguments::VariationPosition;
use skia_safe::textlayout::{
    FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextStyle,
};
use skia_safe::{FontArguments, FontStyle};
use vizia_storage::LayoutTreeIterator;

use crate::prelude::*;

pub(crate) fn text_system(cx: &mut Context) {
    let iterator = LayoutTreeIterator::full(&cx.tree);
    for entity in iterator {
        if !cx.style.text_construction.contains(entity) {
            continue;
        }

        if let Some(paragraph) =
            build_paragraph(entity, &cx.style, cx.text_context.font_collection(), 1.0)
        {
            // println!("build text: {}", entity);
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

        // println!("layout text: {} {}", entity, cx.cache.get_bounds(entity),);

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
                println!("{} {}", entity, bounds.width() - padding_left - padding_right);
            } else {
            }
        }
    }

    cx.style.text_layout.clear();
}

pub fn build_paragraph(
    entity: Entity,
    style: &Style,
    font_collection: &FontCollection,
    opacity: f32,
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

    let mut text_style = TextStyle::new();

    // Font Families
    let font_families = style
        .font_family
        .get(entity)
        .map(|families| {
            families
                .iter()
                .map(|family| match family {
                    FamilyOwned::Generic(_) => "Roboto Flex",
                    FamilyOwned::Named(name) => name.as_str(),
                })
                .collect()
        })
        .unwrap_or_else(|| vec!["Roboto Flex"]);

    text_style.set_font_families(&font_families);

    // Background Color
    let mut paint = skia_safe::Paint::default();
    paint.set_color(style.background_color.get(entity).copied().unwrap_or_default());
    paint.set_anti_alias(true);
    text_style.set_background_paint(&paint);

    // Foreground Color
    let mut paint = skia_safe::Paint::default();
    paint.set_color(style.font_color.get(entity).copied().unwrap_or_default());
    paint.set_anti_alias(true);
    text_style.set_foreground_paint(&paint);

    // Font Size
    let font_size = style.font_size.get(entity).map_or(16.0, |f| f.0);
    text_style.set_font_size(font_size * style.scale_factor());

    // Font Style
    // text_style.set_font_style(FontStyle::new(
    //     style.font_weight.get(entity).copied().unwrap_or_default().into(),
    //     style.font_width.get(entity).copied().unwrap_or_default().into(),
    //     style.font_slant.get(entity).copied().unwrap_or_default().into(),
    // ));

    // Font Variations
    {
        let mut coordinates = vec![];

        if let Some(value) = style.font_weight.get(entity).map(|w| w.0 as f32) {
            coordinates.push(Coordinate { axis: ('w', 'g', 'h', 't').into(), value });
        }

        if let Some(width) = style.font_width.get(entity) {
            coordinates.push(Coordinate {
                axis: ('w', 'd', 't', 'h').into(),
                value: match width {
                    FontWidth::UltraCondensed => 50.0,
                    FontWidth::ExtraCondensed => 62.5,
                    FontWidth::Condensed => 75.0,
                    FontWidth::SemiCondensed => 87.5,
                    FontWidth::Normal => 100.0,
                    FontWidth::SemiExpanded => 112.5,
                    FontWidth::Expanded => 125.0,
                    FontWidth::ExtraExpanded => 137.5,
                    FontWidth::UltraExpanded => 150.0,
                },
            });
        }

        if let Some(slant) = style.font_slant.get(entity) {
            coordinates.push(Coordinate {
                axis: ('s', 'l', 'n', 't').into(),
                value: match slant {
                    FontSlant::Normal => 0.0,
                    FontSlant::Oblique => -5.0,
                    FontSlant::Italic => -10.0,
                },
            });
        }

        if !coordinates.is_empty() {
            text_style
                .set_font_arguments(&FontArguments::new().set_variation_design_position(
                    VariationPosition { coordinates: &coordinates },
                ));
        }
    }
    paragraph_style.set_text_style(&text_style);

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
    paragraph_builder.push_style(&text_style);
    paragraph_builder.add_text(text.as_str());
    paragraph_builder.add_text("\u{200B}");
    paragraph_builder.build().into()
}

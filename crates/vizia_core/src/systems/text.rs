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

            if cx.style.width.get(entity).copied().unwrap_or_default().is_auto() {
                paragraph.layout(text_bounds.width());
            } else {
                paragraph.layout(bounds.width() - padding_left - padding_right);
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
    style.text.get(entity).map(|text| {
        let mut paragraph_style = ParagraphStyle::default();
        if style.text_overflow.get(entity).copied().unwrap_or_default() == TextOverflow::Ellipsis {
            paragraph_style.set_ellipsis("...");
        }

        paragraph_style.set_max_lines(style.line_clamp.get(entity).map(|lc| lc.0 as usize));
        let text_align = if let Some(text_align) = style.text_align.get(entity).copied() {
            text_align
        } else {
            let child_left = style.child_left.get(entity).copied().unwrap_or_default();
            let child_right = style.child_right.get(entity).copied().unwrap_or_default();
            if matches!(child_left, Units::Stretch(_)) {
                if matches!(child_right, Units::Stretch(_)) {
                    TextAlign::Center
                } else {
                    TextAlign::Right
                }
            } else {
                TextAlign::Left
            }
        };

        paragraph_style.set_text_align(text_align.into());

        let mut text_style = TextStyle::new();
        let font_weight = style.font_weight.get(entity).copied().unwrap_or_default();
        let font_width = style.font_width.get(entity).copied().unwrap_or_default();
        let font_slant = style.font_slant.get(entity).copied().unwrap_or_default();
        let font_style = FontStyle::new(font_weight.into(), font_width.into(), font_slant.into());
        text_style.set_font_style(font_style);
        let font_families = style
            .font_family
            .get(entity)
            .map(|families| {
                families
                    .iter()
                    .map(|family| match family {
                        FamilyOwned::Generic(generic) => match generic {
                            _ => "Roboto Flex",
                        },

                        FamilyOwned::Named(name) => name.as_str(),
                    })
                    .collect()
            })
            .unwrap_or(vec!["Roboto Flex"]);

        text_style.set_font_families(&font_families);

        text_style.set_color(
            style
                .font_color
                .get(entity)
                .copied()
                .map(|col| Color::rgba(col.r(), col.g(), col.b(), (opacity * col.a() as f32) as u8))
                .unwrap_or_default(),
        );
        let font_size = style.font_size.get(entity).copied().map(|f| f.0).unwrap_or(16.0)
            * style.scale_factor();
        text_style.set_font_size(font_size);
        // text_style.add_font_feature("opsz", 1);
        let coordinates = Box::new([
            Coordinate { axis: ('w', 'g', 'h', 't').into(), value: font_weight.0 as f32 },
            Coordinate { axis: ('o', 'p', 's', 'z').into(), value: font_size },
        ]);
        let args = FontArguments::new();
        let pos = VariationPosition { coordinates: coordinates.as_ref() };
        let args = args.set_variation_design_position(pos);
        text_style.set_font_arguments(&Some(args));

        paragraph_style.turn_hinting_off();
        paragraph_style.set_text_style(&text_style);
        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

        paragraph_builder.push_style(&text_style);
        paragraph_builder.add_text(text.as_str());
        paragraph_builder.add_text("\u{200B}");

        paragraph_builder.build()
    })
}

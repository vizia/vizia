use vizia_storage::LayoutChildIterator;

use crate::text::resolved_text_direction;
use crate::text::{build_pre_shaped_text, shaped_text::ShapedText};
use crate::{cache::CachedData, prelude::*};

pub(crate) fn text_system(cx: &mut Context) {
    if cx.style.text.is_empty() || cx.style.text_construction.is_empty() {
        return;
    }

    let dirty_entities = std::mem::take(&mut cx.style.text_construction);
    for entity in dirty_entities {
        if cx.style.text.contains(entity)
            && cx.style.display.get(entity).copied().unwrap_or_default() != Display::None
        {
            let pre_shaped =
                build_pre_shaped_text(entity, &mut cx.style, &cx.tree, &mut cx.text_context);
            let shaped = ShapedText::new(pre_shaped);
            cx.text_context.text_shaped.insert(entity, shaped);

            cx.style.needs_relayout(entity);
            cx.style.needs_text_layout(entity);
        }
    }
}

pub(crate) fn text_layout_system(cx: &mut Context) {
    if cx.style.text_layout.is_empty() {
        return;
    }

    let dirty_entities = std::mem::take(&mut cx.style.text_layout);
    let mut redraw_entities = Vec::new();
    for entity in dirty_entities {
        if let Some(shaped) = cx.text_context.text_shaped.get_mut(entity) {
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

            let avail_w = bounds.w - padding_left - padding_right;

            let width_is_auto = cx
                .style
                .width
                .get_resolved(entity, &cx.style.custom_units_props)
                .unwrap_or_default()
                .is_auto();
            let height_is_auto = cx
                .style
                .height
                .get_resolved(entity, &cx.style.custom_units_props)
                .unwrap_or_default()
                .is_auto();

            let text_width = if !width_is_auto && !height_is_auto {
                if cx.style.text_overflow.get(entity).copied().unwrap_or_default()
                    == TextOverflow::Clip
                {
                    let w = avail_w.max(shaped.min_intrinsic_width() + 1.0);
                    shaped.layout(w);
                    let mut tb = BoundingBox { x: padding_left, y: 0.0, w, h: bounds.h };
                    tb.w = shaped.max_intrinsic_width();
                    cx.text_context.text_bounds.insert(entity, tb);
                    w
                } else {
                    shaped.layout(avail_w);
                    let mut tb = bounds;
                    tb.w = shaped.max_intrinsic_width();
                    cx.text_context.text_bounds.insert(entity, tb);
                    avail_w
                }
            } else {
                shaped.layout(avail_w);
                if let Some(stored) = cx.text_context.text_bounds.get_mut(entity) {
                    stored.x = bounds.x + padding_left;
                    stored.y = bounds.y + padding_top;
                    stored.w = avail_w;
                    stored.h = shaped.height();
                }
                avail_w
            };

            layout_span(&cx.style, &mut cx.cache, &cx.tree, entity, shaped, bounds);
            let _ = text_width;

            redraw_entities.push(entity);
        }
    }
    for entity in redraw_entities {
        cx.needs_redraw(entity);
    }
}

pub fn layout_span(
    style: &Style,
    cache: &mut CachedData,
    tree: &Tree<Entity>,
    entity: Entity,
    shaped: &ShapedText,
    paragraph_bounds: BoundingBox,
) -> BoundingBox {
    let mut bounds = BoundingBox::default();
    if style.text_span.get(entity).copied().unwrap_or_default() {
        if let Some(range) = style.text_range.get(entity) {
            let rects = shaped.get_rects_for_range(range.clone());

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
            bounds = layout_span(style, cache, tree, child, shaped, paragraph_bounds);
        } else {
            bounds =
                bounds.union(&layout_span(style, cache, tree, child, shaped, paragraph_bounds));
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

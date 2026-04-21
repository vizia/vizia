use crate::{cache::CachedData, prelude::*};
use morphorm::Node;
use skia_safe::{ClipOp, ImageFilter, Paint, Rect, SamplingOptions, Surface, canvas::SaveLayerRec};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use vizia_storage::{DrawChildIterator, LayoutTreeIterator};
use vizia_style::BlendMode;

pub(crate) fn draw_system(
    cx: &mut Context,
    window_entity: Entity,
    surface: &mut Surface,
    dirty_surface: &mut Surface,
) -> bool {
    if cx.windows.is_empty() {
        return false;
    }

    if !cx.entity_manager.is_alive(window_entity) {
        return false;
    }

    let window = cx.windows.get_mut(&window_entity).unwrap();

    let mut dirty_rect = std::mem::take(&mut window.dirty_rect);
    let redraw_list = std::mem::take(&mut window.redraw_list);

    // Note: dirty_rect can be populated by entity removal independently of
    // redraw_list, and backdrop filter processing generates dirty areas outside
    // the redraw_list. Un-commenting would cause rendering artifacts. Maybe we
    // can revisit this in the future.
    // if redraw_list.is_empty() {
    //     return false;
    // }

    for &entity in &redraw_list {
        // Skip binding views
        if cx.tree.is_ignored(entity) {
            continue;
        }

        if cx.tree.is_window(entity) && entity != window_entity {
            continue;
        }

        if entity.visible(&cx.style) {
            let draw_bounds = draw_bounds(&cx.style, &cx.cache, &cx.tree, entity);
            let dirty_bounds = cx
                .cache
                .draw_bounds
                .get(entity)
                .map_or(draw_bounds, |prev_bounds| draw_bounds.union(prev_bounds));
            union_dirty_rect(&mut dirty_rect, dirty_bounds);
            cx.cache.draw_bounds.insert(entity, draw_bounds);
        } else {
            // If the entity was previously visible but is no longer, we must dirty
            // its previous area so that it is correctly cleared from the screen.
            if let Some(previous_draw_bounds) = cx.cache.draw_bounds.remove(entity) {
                union_dirty_rect(&mut dirty_rect, previous_draw_bounds);
            }
        }
    }

    let iter = LayoutTreeIterator::full(&cx.tree);
    for entity in iter {
        if cx.tree.is_ignored(entity) {
            continue;
        }

        if cx.tree.is_window(entity) && entity != window_entity {
            continue;
        }

        // Check if the entity has a filter style.
        if cx.style.filter.get(entity).is_some() || cx.style.backdrop_filter.get(entity).is_some() {
            if entity.visible(&cx.style) {
                // Entity is VISIBLE and has a filter.
                // Skip recomputation if already processed in redraw_list.
                if !redraw_list.contains(&entity) {
                    let filter_current_bounds = draw_bounds(&cx.style, &cx.cache, &cx.tree, entity);

                    // Update cache for visible entity
                    cx.cache.draw_bounds.insert(entity, filter_current_bounds);

                    if filter_current_bounds.w > 0.0 && filter_current_bounds.h > 0.0 {
                        // Ensure bounds are valid
                        // Condition to update dirty_rect:
                        // 1. dirty_rect is None (then set it to filter_current_bounds).
                        // 2. dirty_rect is Some, and filter_current_bounds intersects with it (then union).
                        if dirty_rect.is_none_or(|current_dr_val| {
                            filter_current_bounds.intersects(&current_dr_val)
                        }) {
                            dirty_rect =
                                Some(dirty_rect.map_or(filter_current_bounds, |current_dr_val| {
                                    current_dr_val.union(&filter_current_bounds)
                                }));
                        }
                    }
                }
            } else {
                // Entity is INVISIBLE but has (or had) a filter style.
                // Its *previous* bounds need to be added to dirty_rect.
                if let Some(previous_draw_bounds) = cx.cache.draw_bounds.get(entity).copied() {
                    union_dirty_rect(&mut dirty_rect, previous_draw_bounds);
                }

                // Remove from cache as it's no longer visible with these bounds.
                cx.cache.draw_bounds.remove(entity);
            }
        }
    }

    // if dirty_rect.is_none() {
    //     return true;
    // }

    let canvas = dirty_surface.canvas();

    canvas.save();

    if let Some(rect) = dirty_rect.map(Rect::from) {
        canvas.clip_rect(rect, ClipOp::Intersect, false);
        canvas.clear(Color::transparent());
    }

    cx.resource_manager.mark_images_unused();

    let mut queue = BinaryHeap::new();
    queue.push(ZEntity { index: 0, entity: window_entity, visible: true });

    while let Some(zentity) = queue.pop() {
        canvas.save();
        draw_entity(
            &mut DrawContext {
                current: zentity.entity,
                style: &cx.style,
                cache: &mut cx.cache,
                tree: &cx.tree,
                models: &cx.models,
                views: &mut cx.views,
                resource_manager: &cx.resource_manager,
                text_context: &mut cx.text_context,
                modifiers: &cx.modifiers,
                mouse: &cx.mouse,
                windows: &mut cx.windows,
            },
            &dirty_rect,
            canvas,
            zentity.index,
            &mut queue,
            zentity.visible,
        );
        canvas.restore();
    }

    canvas.restore();

    // surface.canvas().clear(Color::transparent());
    dirty_surface.draw(surface.canvas(), (0, 0), SamplingOptions::default(), None);

    // Debug draw dirty rect
    // if let Some(rect) = dirty_rect.map(Rect::from) {
    //     let mut paint = Paint::default();
    //     paint.set_style(skia_safe::PaintStyle::Stroke);
    //     paint.set_color(Color::red());
    //     paint.set_stroke_width(1.0);
    //     surface.canvas().draw_rect(rect, &paint);
    // }

    true
}

fn union_dirty_rect(dirty_rect: &mut Option<BoundingBox>, bounds: BoundingBox) {
    if bounds.w <= 0.0 || bounds.h <= 0.0 {
        return;
    }

    if let Some(current) = dirty_rect.as_mut() {
        *current = current.union(&bounds);
    } else {
        *dirty_rect = Some(bounds);
    }
}

fn draw_entity(
    cx: &mut DrawContext,
    dirty_rect: &Option<BoundingBox>,
    canvas: &Canvas,
    current_z: i32,
    queue: &mut BinaryHeap<ZEntity>,
    visible: bool,
) {
    let current = cx.current;

    // Skip views with display: none.
    if cx.display() == Display::None {
        return;
    }

    let z_index = cx.z_index();

    if z_index > current_z {
        queue.push(ZEntity { index: z_index, entity: current, visible });
        return;
    }

    let filter = cx.filter();
    let backdrop_filter = cx.backdrop_filter();
    let blend_mode = cx.style.blend_mode.get(current).copied().unwrap_or_default();

    let layer_count = if cx.opacity() != 1.0
        || filter.is_some()
        || backdrop_filter.is_some()
        || blend_mode != BlendMode::Normal
    {
        let mut paint = Paint::default();
        paint.set_alpha_f(cx.opacity());
        paint.set_blend_mode(blend_mode.into());

        let rect: Rect = cx.bounds().into();
        let mut backdrop_image_filter = ImageFilter::crop(rect, None, None).unwrap();

        if let Some(filter) = filter {
            match filter {
                Filter::Blur(radius) => {
                    let sigma = radius.to_px().unwrap() * cx.scale_factor() / 2.0;
                    let image_filter = ImageFilter::crop(rect, None, None)
                        .unwrap()
                        .blur(None, (sigma, sigma), None)
                        .unwrap();
                    paint.set_image_filter(image_filter);
                }
            }
        }

        let slr = if let Some(backdrop_filter) = backdrop_filter {
            match backdrop_filter {
                Filter::Blur(radius) => {
                    let sigma = radius.to_px().unwrap() * cx.scale_factor() / 2.0;
                    backdrop_image_filter =
                        backdrop_image_filter.blur(None, (sigma, sigma), None).unwrap();
                    SaveLayerRec::default().paint(&paint).backdrop(&backdrop_image_filter)
                }
            }
        } else {
            SaveLayerRec::default().paint(&paint)
        };

        Some(canvas.save_layer(&slr))
    } else {
        None
    };

    canvas.save();
    if let Some(Some(clip_path)) = cx.cache.clip_path.get(current) {
        canvas.clip_path(clip_path, ClipOp::Intersect, true);
    }

    if let Some(transform) = cx.cache.transform.get(current) {
        canvas.set_matrix(&(transform.into()));
    }

    let is_visible = match (visible, cx.visibility()) {
        (v, None) => v,
        (_, Some(Visibility::Hidden)) => false,
        (_, Some(Visibility::Visible)) => true,
    };

    // Draw the view
    if is_visible {
        let should_draw = if let Some(dirty_rect) = dirty_rect {
            let bounds = cached_draw_bounds(cx, current);
            bounds.intersects(dirty_rect)
        } else {
            true
        };

        if should_draw {
            if let Some(view) = cx.views.remove(&current) {
                view.draw(cx, canvas);
                cx.views.insert(current, view);
            }
        }
    }
    canvas.restore();
    let child_iter = DrawChildIterator::new(cx.tree, cx.current);

    // Draw its children
    for child in child_iter {
        cx.current = child;
        // TODO: Skip views with zero-sized bounding boxes here? Or let user decide if they want to skip?
        draw_entity(cx, dirty_rect, canvas, current_z, queue, is_visible);
    }

    if let Some(count) = layer_count {
        canvas.restore_to_count(count);
    }

    cx.current = current;
}

fn cached_draw_bounds(cx: &mut DrawContext, entity: Entity) -> BoundingBox {
    if let Some(bounds) = cx.cache.draw_bounds.get(entity).copied() {
        return bounds;
    }

    let bounds = draw_bounds(cx.style, cx.cache, cx.tree, entity);
    cx.cache.draw_bounds.insert(entity, bounds);
    bounds
}

// Must be called after transform and clipping systems to be valid.
pub(crate) fn draw_bounds(
    style: &Style,
    cache: &CachedData,
    tree: &Tree<Entity>,
    entity: Entity,
) -> BoundingBox {
    let mut layout_bounds = cache.bounds.get(entity).copied().unwrap();

    if let Some(shadows) = style.shadow.get_resolved(entity, &style.custom_shadow_props) {
        let original_bounds = layout_bounds;

        for shadow in shadows.iter().filter(|shadow| !shadow.inset) {
            let mut shadow_bounds = original_bounds;

            let scale_factor = style.scale_factor();

            let x = shadow.x_offset.to_px().unwrap() * scale_factor;
            let y = shadow.y_offset.to_px().unwrap() * scale_factor;

            shadow_bounds = shadow_bounds.offset(x, y);

            if let Some(blur_radius) = shadow.blur_radius.as_ref().and_then(Length::to_px) {
                shadow_bounds = shadow_bounds.expand(blur_radius * scale_factor);
            }

            if let Some(spread_radius) = shadow.spread_radius.as_ref().and_then(Length::to_px) {
                shadow_bounds = shadow_bounds.expand(spread_radius * scale_factor);
            }

            layout_bounds = layout_bounds.union(&shadow_bounds);
        }
    }

    let mut outline_bounds = layout_bounds;

    if let Some(outline_width) =
        style.outline_width.get_resolved(entity, &style.custom_length_props)
    {
        outline_bounds = outline_bounds
            .expand(outline_width.to_pixels(layout_bounds.diagonal(), style.scale_factor()));
    }

    if let Some(outline_offset) =
        style.outline_offset.get_resolved(entity, &style.custom_length_props)
    {
        outline_bounds = outline_bounds
            .expand(outline_offset.to_pixels(layout_bounds.diagonal(), style.scale_factor()));
    }

    layout_bounds = layout_bounds.union(&outline_bounds);

    let matrix = cache.transform.get(entity).copied().unwrap_or_default();

    let (rect, _) = matrix.map_rect(Rect::from(layout_bounds));

    let mut dirty_bounds = BoundingBox::from_min_max(
        rect.left().floor(),
        rect.top().floor(),
        rect.right().ceil(),
        rect.bottom().ceil(),
    );

    // If overflow is visible we have to include all children since they may draw outside of our own bounds.
    if !matches!(style.overflowx.get(entity), Some(Overflow::Hidden))
        || !matches!(style.overflowy.get(entity), Some(Overflow::Hidden))
    {
        let child_iter = DrawChildIterator::new(tree, entity);
        for child in child_iter {
            dirty_bounds = dirty_bounds.union(&draw_bounds(style, cache, tree, child));
        }
    }

    if let Some(filter) = style.filter.get(entity) {
        match filter {
            Filter::Blur(radius) => {
                dirty_bounds = dirty_bounds.expand(radius.to_px().unwrap() * style.scale_factor());
            }
        }
    }

    // If z-index is not zero we can't be sure of the stacking context so we have to assume so and not clip.
    if matches!(style.z_index.get(entity), Some(z_index) if *z_index != 0) {
        return dirty_bounds;
    }

    if tree.is_window(entity) {
        return dirty_bounds;
    }

    let parent = tree
        .get_layout_parent(entity)
        .unwrap_or_else(|| tree.get_parent_window(entity).unwrap_or(Entity::root()));

    let Some(Some(clip_path)) = cache.clip_path.get(parent) else {
        return dirty_bounds;
    };

    let clip_bounds = clip_path.bounds();

    let clip_bounds = BoundingBox::from_min_max(
        clip_bounds.left().floor(),
        clip_bounds.top().floor(),
        clip_bounds.right().ceil(),
        clip_bounds.bottom().ceil(),
    );

    dirty_bounds.intersection(&clip_bounds)
}

struct ZEntity {
    pub index: i32,
    pub entity: Entity,
    pub visible: bool,
}

impl Ord for ZEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        other.index.cmp(&self.index)
    }
}
impl PartialOrd for ZEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for ZEntity {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for ZEntity {}

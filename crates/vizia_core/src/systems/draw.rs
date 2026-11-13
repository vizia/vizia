use crate::{
    cache::{CachedData, DrawBounds, is_identity, is_scale_translate},
    prelude::*,
    systems::clipping::own_clip_bounds,
};
use hashbrown::HashSet;
use morphorm::Node;
use skia_safe::{
    ClipOp, ImageFilter, Matrix, Paint, Rect, SamplingOptions, Surface, canvas::SaveLayerRec,
};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use vizia_storage::{DrawChildIterator, LayoutParentIterator};
use vizia_style::BlendMode;

fn should_apply_entity_clip(style: &Style, entity: Entity) -> bool {
    !style.ignore_clipping.get(entity).copied().unwrap_or(false)
}

pub(crate) fn draw_system(
    cx: &mut Context,
    window_entity: Entity,
    surface: &mut Surface,
    dirty_surface: &mut Surface,
) -> bool {
    let _scope = crate::profiling::scope("draw_system");
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

    let redraw_entities: Vec<Entity> = redraw_list
        .iter()
        .copied()
        .filter(|&entity| {
            !cx.tree.is_ignored(entity) && (!cx.tree.is_window(entity) || entity == window_entity)
        })
        .collect();

    // The cached draw bounds of an entity cover all of its descendants, so a redrawn entity with a
    // visible redrawn ancestor which has cached bounds does not need to dirty its own areas. The
    // previous areas are computed before any cached bounds are invalidated.
    let redraw_set: HashSet<Entity> = redraw_entities.iter().copied().collect();
    let previous_bounds: Vec<(Entity, Option<BoundingBox>)> = redraw_entities
        .iter()
        .copied()
        .filter(|&entity| {
            !LayoutParentIterator::new(&cx.tree, entity).skip(1).any(|ancestor| {
                redraw_set.contains(&ancestor)
                    && cx.cache.draw_bounds.contains(ancestor)
                    && ancestor.visible(&cx.style)
            })
        })
        .map(|entity| (entity, cached_screen_bounds(&cx.cache, &cx.tree, entity)))
        .collect();

    for &entity in &redraw_entities {
        cx.cache.invalidate_draw_bounds(&cx.tree, entity);
    }

    for (entity, previous) in previous_bounds {
        // If the entity was previously visible but is no longer, its previous area must still be
        // dirtied so that it is correctly cleared from the screen.
        if let Some(previous) = previous {
            union_dirty_rect(&mut dirty_rect, previous);
        }

        if entity.visible(&cx.style) {
            let bounds = screen_bounds(&cx.style, &mut cx.cache, &cx.tree, entity);
            union_dirty_rect(&mut dirty_rect, bounds);
        }
    }

    for entity in cx.style.filter_entities(&cx.tree) {
        if cx.tree.is_ignored(entity) {
            continue;
        }

        if cx.tree.is_window(entity) && entity != window_entity {
            continue;
        }

        if entity.visible(&cx.style) {
            // Entity is VISIBLE and has a filter.
            // Skip recomputation if already processed in redraw_list.
            if !redraw_list.contains(&entity) {
                let filter_current_bounds =
                    screen_bounds(&cx.style, &mut cx.cache, &cx.tree, entity);

                if is_non_empty(&filter_current_bounds) {
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
            if let Some(previous_draw_bounds) = cached_screen_bounds(&cx.cache, &cx.tree, entity) {
                union_dirty_rect(&mut dirty_rect, previous_draw_bounds);
                // Remove from cache as it's no longer visible with these bounds.
                cx.cache.invalidate_draw_bounds(&cx.tree, entity);
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

    let mut raised = BinaryHeap::new();
    let mut clipping_escapes = BinaryHeap::new();
    let mut next_order = 0;
    let mut queue = DrawQueue {
        raised: &mut raised,
        clipping_escapes: &mut clipping_escapes,
        next_order: &mut next_order,
        escape_opacity: 1.0,
    };
    // With no dirty area nothing visible has changed, so the retained dirty surface is still
    // valid and only needs compositing. Surface recreation (e.g. on resize) always dirties the
    // whole window via `needs_refresh` or `needs_resize_redraw`.
    if dirty_rect.is_some() {
        queue.defer_raised(
            0,
            window_entity,
            true,
            1.0,
            ParentSpace::new(Matrix::new_identity(), None, &dirty_rect),
        );
    }

    while let Some(zentity) = {
        queue.raised.append(queue.clipping_escapes);
        queue.raised.pop()
    } {
        let count = save_ancestor_opacity(canvas, zentity.ancestor_opacity);
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
            false,
            zentity.ancestor_opacity,
            &zentity.parent,
        );
        canvas.restore_to_count(count);
    }

    canvas.restore();

    // Composite dirty surface with Src blend so transparent pixels in dirty
    // regions replace destination pixels instead of preserving stale content.
    let mut composite_paint = Paint::default();
    composite_paint.set_blend_mode(skia_safe::BlendMode::Src);
    dirty_surface.draw(
        surface.canvas(),
        (0, 0),
        SamplingOptions::default(),
        Some(&composite_paint),
    );

    // Debug overlay: outline the views which underwent layout in the most recent layout pass by
    // stroking their transformed bounds with an orange rectangle. The overlay persists across
    // frames (it is redrawn in full on every draw) until the layout system runs again, at which
    // point the layout system repopulates `laid_out` and schedules the previous outlines to be
    // erased.
    if cx.style.debug_layout && !cx.style.laid_out.is_empty() {
        let canvas = surface.canvas();
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::PaintStyle::Stroke);
        paint.set_stroke_width(1.0);
        // Orange (RGB 255, 165, 0).
        paint.set_color(skia_safe::Color::from_argb(255, 255, 165, 0));

        for entity in cx.style.laid_out.iter().copied() {
            let Some(bounds) = cx.cache.bounds.get(entity).copied() else {
                continue;
            };
            if bounds.w <= 0.0 || bounds.h <= 0.0 {
                continue;
            }
            let matrix = cx.cache.absolute_transform(&cx.tree, entity);
            let (rect, _) = matrix.map_rect(Rect::from(bounds));
            canvas.draw_rect(rect, &paint);
        }
    }

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

/// Saves the canvas and, for a queued view drawn outside some of its ancestors' opacity layers,
/// re-enters the product of their opacities, as CSS applies an ancestor's opacity to every
/// descendant. Returns the count to restore to once the view is drawn.
fn save_ancestor_opacity(canvas: &Canvas, ancestor_opacity: f32) -> usize {
    let count = canvas.save();
    if ancestor_opacity != 1.0 {
        let mut paint = Paint::default();
        paint.set_alpha_f(ancestor_opacity);
        canvas.save_layer(&SaveLayerRec::default().paint(&paint));
    }
    count
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

#[allow(clippy::too_many_arguments)]
fn draw_entity(
    cx: &mut DrawContext,
    dirty_rect: &Option<BoundingBox>,
    canvas: &Canvas,
    current_z: i32,
    queue: &mut DrawQueue<'_>,
    visible: bool,
    ancestor_backdrop_clip: bool,
    ancestor_opacity: f32,
    parent: &ParentSpace,
) {
    let current = cx.current;

    // Skip views with display: none.
    if cx.display() == Display::None {
        return;
    }

    // Skip the whole subtree if nothing within it can intersect the dirty rect. Subtree bounds
    // include z-indexed and clip-escaping descendants, so it is safe to cull before they would be
    // deferred to the z-order queue.
    let draw_bounds = dirty_rect.map(|_| cached_bounds(cx.style, cx.cache, cx.tree, current));
    if let (Some(dirty_rect), Some(bounds)) = (dirty_rect, &draw_bounds) {
        if !parent.intersects_dirty(bounds, dirty_rect) {
            return;
        }
    }

    let z_index = cx.z_index();

    // A Skia clip cannot be expanded until its saved canvas state is restored. Defer views which
    // explicitly escape ancestor clipping so a backdrop clip higher in the recursive draw cannot
    // accidentally constrain popovers and other overlays.
    if ancestor_backdrop_clip && !should_apply_entity_clip(cx.style, current) {
        // Clip escapes drain with their owner, outside the layers opened below it.
        queue.defer_clipping_escape(
            current_z.max(z_index),
            current,
            visible,
            queue.escape_opacity * ancestor_opacity,
            *parent,
        );
        return;
    }

    if z_index > current_z {
        queue.defer_raised(z_index, current, visible, ancestor_opacity, *parent);
        return;
    }

    let transform = match cx.cache.local_transform.get(current) {
        Some(local) if !is_identity(local) => parent.transform * *local,
        _ => parent.transform,
    };

    let filter = cx.filter();
    let backdrop_filter = cx.backdrop_filter();
    let blend_mode = cx.style.blend_mode.get(current).copied().unwrap_or_default();
    let owns_opacity_queue = cx.opacity() != 1.0
        && filter.is_none()
        && backdrop_filter.is_none()
        && blend_mode == BlendMode::Normal;

    // Backdrop layers must be created inside the entity clip. Creating the
    // layer first allows image-filter output to bleed past rounded clips.
    let backdrop_clip_count = backdrop_filter.map(|_| {
        let count = canvas.save();
        if let Some(Some(clip_path)) = cx.cache.clip_path.get(current) {
            canvas.clip_path(clip_path, ClipOp::Intersect, true);
        }
        count
    });

    let layer_count = if cx.opacity() != 1.0
        || filter.is_some()
        || backdrop_filter.is_some()
        || blend_mode != BlendMode::Normal
    {
        let mut paint = Paint::default();
        paint.set_alpha_f(cx.opacity());
        paint.set_blend_mode(blend_mode.into());

        let rect: Rect = cx.bounds().into();
        let transformed_rect = transform.map_rect(rect).0;
        let mut backdrop_image_filter = ImageFilter::crop(transformed_rect, None, None).unwrap();

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
                    SaveLayerRec::default()
                        .bounds(&transformed_rect)
                        .paint(&paint)
                        .backdrop(&backdrop_image_filter)
                }
            }
        } else {
            SaveLayerRec::default().paint(&paint)
        };

        Some(canvas.save_layer(&slr))
    } else {
        None
    };

    let is_visible = match (visible, cx.visibility()) {
        (v, None) => v,
        (_, Some(Visibility::Hidden)) => false,
        (_, Some(Visibility::Visible)) => true,
    };

    let should_draw = if is_visible {
        if let (Some(dirty_rect), Some(bounds)) = (dirty_rect, &draw_bounds) {
            intersects_non_empty(&own_to_screen(bounds, &parent.transform, parent.clip), dirty_rect)
        } else {
            true
        }
    } else {
        false
    };

    // Draw the view. Children set up their own clip and absolute transform, so the canvas state
    // only needs to be configured when this view actually paints.
    if should_draw {
        canvas.save();
        if backdrop_clip_count.is_none() {
            if let Some(Some(clip_path)) = cx.cache.clip_path.get(current) {
                canvas.clip_path(clip_path, ClipOp::Intersect, true);
            }
        }
        canvas.set_matrix(&(transform.into()));

        if let Some(view) = cx.views.remove(&current) {
            view.draw(cx, canvas);
            cx.views.insert(current, view);
        }
        canvas.restore();
    }
    let child_iter = DrawChildIterator::new(cx.tree, cx.current);
    let descendants_are_backdrop_clipped = ancestor_backdrop_clip || backdrop_clip_count.is_some();
    let child_space = ParentSpace::new(transform, clip_bounds(cx.cache, current), dirty_rect);
    let opacity = ancestor_opacity * cx.opacity();

    // An opacity group composites its complete image once, including raised descendants. A Skia
    // clip cannot be widened while it is active, so only a group outside every backdrop clip also
    // owns the clip escapes below it; a group inside one leaves them to an outer owner.
    let owns_escapes = owns_opacity_queue && !ancestor_backdrop_clip;
    let mut group_raised = BinaryHeap::new();
    let mut group_escapes = BinaryHeap::new();
    let (raised, descendants_ancestor_opacity, escape_opacity) = if owns_opacity_queue {
        let escape_opacity = if owns_escapes { 1.0 } else { queue.escape_opacity * opacity };
        (&mut group_raised, 1.0, escape_opacity)
    } else {
        (&mut *queue.raised, opacity, queue.escape_opacity)
    };
    let mut child_queue = DrawQueue {
        raised,
        clipping_escapes: if owns_escapes {
            &mut group_escapes
        } else {
            &mut *queue.clipping_escapes
        },
        next_order: queue.next_order,
        escape_opacity,
    };

    for child in child_iter {
        cx.current = child;
        // TODO: Skip views with zero-sized bounding boxes here? Or let user decide if they want to skip?
        draw_entity(
            cx,
            dirty_rect,
            canvas,
            current_z,
            &mut child_queue,
            is_visible,
            descendants_are_backdrop_clipped,
            descendants_ancestor_opacity,
            &child_space,
        );
    }

    if owns_opacity_queue {
        while let Some(zentity) = {
            if owns_escapes {
                child_queue.raised.append(child_queue.clipping_escapes);
            }
            child_queue.raised.pop()
        } {
            cx.current = zentity.entity;
            // This group's layer is still open, but layers between it and the view have closed.
            let count = save_ancestor_opacity(canvas, zentity.ancestor_opacity);
            draw_entity(
                cx,
                dirty_rect,
                canvas,
                zentity.index,
                &mut child_queue,
                zentity.visible,
                descendants_are_backdrop_clipped,
                zentity.ancestor_opacity,
                &zentity.parent,
            );
            canvas.restore_to_count(count);
        }
    }

    cx.current = current;

    // Draw outline after children so it appears above descendant content.
    if should_draw {
        canvas.save();
        if let Some(Some(clip_path)) = cx.cache.clip_path.get(current) {
            canvas.clip_path(clip_path, ClipOp::Intersect, true);
        }

        canvas.set_matrix(&(transform.into()));

        cx.draw_outline(canvas);
        canvas.restore();
    }

    if let Some(count) = layer_count {
        canvas.restore_to_count(count);
    }
    if let Some(count) = backdrop_clip_count {
        canvas.restore_to_count(count);
    }
}

fn is_non_empty(bounds: &BoundingBox) -> bool {
    bounds.w > 0.0 && bounds.h > 0.0
}

fn intersects_non_empty(a: &BoundingBox, b: &BoundingBox) -> bool {
    is_non_empty(a) && is_non_empty(b) && a.intersects(b)
}

fn union_non_empty(a: BoundingBox, b: BoundingBox) -> BoundingBox {
    match (is_non_empty(&a), is_non_empty(&b)) {
        (true, true) => a.union(&b),
        (true, false) => a,
        _ => b,
    }
}

fn snap(rect: Rect) -> BoundingBox {
    BoundingBox::from_min_max(
        rect.left().floor(),
        rect.top().floor(),
        rect.right().ceil(),
        rect.bottom().ceil(),
    )
}

/// Maps bounds through a transform, expanding the result to whole pixels.
fn map_bounds(transform: &Matrix, bounds: BoundingBox) -> BoundingBox {
    if !is_non_empty(&bounds) {
        return BoundingBox::default();
    }

    // Fast path for scale and translation, which avoids a call into Skia for every culled entity.
    if is_scale_translate(transform) {
        let (sx, sy) = (transform.scale_x(), transform.scale_y());
        let (tx, ty) = (transform.translate_x(), transform.translate_y());
        let (x0, x1) = (bounds.left() * sx + tx, bounds.right() * sx + tx);
        let (y0, y1) = (bounds.top() * sy + ty, bounds.bottom() * sy + ty);
        return snap(Rect::new(x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1)));
    }

    snap(transform.map_rect(Rect::from(bounds)).0)
}

/// Returns the bounds of the clip path of an entity in window coordinates.
pub(crate) fn clip_bounds(cache: &CachedData, entity: Entity) -> Option<BoundingBox> {
    cache.clip_bounds.get(entity).copied()
}

/// Converts the cached draw bounds of an entity and its descendants into window coordinates, given
/// the transform and clip bounds of the parent of the entity in window coordinates.
pub(crate) fn subtree_to_screen(
    bounds: &DrawBounds,
    parent_transform: &Matrix,
    parent_clip: Option<BoundingBox>,
) -> BoundingBox {
    let clipped = map_bounds(parent_transform, bounds.clipped);
    let clipped = match parent_clip {
        Some(clip) if is_non_empty(&clipped) => clipped.intersection(&clip),
        _ => clipped,
    };
    if !is_non_empty(&bounds.escaped) {
        return clipped;
    }
    union_non_empty(clipped, map_bounds(parent_transform, bounds.escaped))
}

/// Converts the cached bounds drawn by an entity itself into window coordinates.
fn own_to_screen(
    bounds: &DrawBounds,
    parent_transform: &Matrix,
    parent_clip: Option<BoundingBox>,
) -> BoundingBox {
    let own = map_bounds(parent_transform, bounds.own);
    match parent_clip {
        Some(clip) if !bounds.escapes_clip && is_non_empty(&own) => own.intersection(&clip),
        _ => own,
    }
}

/// Returns the transform and clip bounds, in window coordinates, which apply to the cached draw
/// bounds of an entity.
fn parent_space(
    cache: &CachedData,
    tree: &Tree<Entity>,
    entity: Entity,
) -> (Matrix, Option<BoundingBox>) {
    if tree.is_window(entity) {
        return (Matrix::new_identity(), None);
    }

    let parent = tree
        .get_layout_parent(entity)
        .unwrap_or_else(|| tree.get_parent_window(entity).unwrap_or(Entity::root()));

    (cache.absolute_transform(tree, parent), clip_bounds(cache, parent))
}

/// Returns the previously computed draw bounds of an entity and its descendants in window
/// coordinates, or `None` if they are not cached.
pub(crate) fn cached_screen_bounds(
    cache: &CachedData,
    tree: &Tree<Entity>,
    entity: Entity,
) -> Option<BoundingBox> {
    let bounds = cache.draw_bounds.get(entity)?;
    let (parent_transform, parent_clip) = parent_space(cache, tree, entity);
    Some(subtree_to_screen(bounds, &parent_transform, parent_clip))
}

pub(crate) fn cached_own_screen_bounds(
    cache: &CachedData,
    tree: &Tree<Entity>,
    entity: Entity,
) -> Option<BoundingBox> {
    let bounds = cache.draw_bounds.get(entity)?;
    let (parent_transform, parent_clip) = parent_space(cache, tree, entity);
    Some(own_to_screen(bounds, &parent_transform, parent_clip))
}

/// Returns the draw bounds of an entity and its descendants in window coordinates, computing and
/// caching them if necessary.
fn screen_bounds(
    style: &Style,
    cache: &mut CachedData,
    tree: &Tree<Entity>,
    entity: Entity,
) -> BoundingBox {
    let bounds = cached_bounds(style, cache, tree, entity);
    let (parent_transform, parent_clip) = parent_space(cache, tree, entity);
    subtree_to_screen(&bounds, &parent_transform, parent_clip)
}

/// Returns the cached draw bounds of an entity, computing and caching them (and those of its
/// descendants) if necessary.
fn cached_bounds(
    style: &Style,
    cache: &mut CachedData,
    tree: &Tree<Entity>,
    entity: Entity,
) -> DrawBounds {
    if let Some(bounds) = cache.draw_bounds.get(entity).copied() {
        return bounds;
    }

    let bounds = draw_bounds(style, cache, tree, entity);
    cache.draw_bounds.insert(entity, bounds);
    bounds
}

/// Computes the draw bounds of an entity in the coordinate space of its parent.
// Must be called after the layout and transform systems to be valid.
fn draw_bounds(
    style: &Style,
    cache: &mut CachedData,
    tree: &Tree<Entity>,
    entity: Entity,
) -> DrawBounds {
    let mut own = decorated_bounds(style, cache, entity);

    let mut clipped = BoundingBox::default();
    let mut escaped = BoundingBox::default();
    for child in DrawChildIterator::new(tree, entity) {
        let child_bounds = cached_bounds(style, cache, tree, child);
        clipped = union_non_empty(clipped, child_bounds.clipped);
        escaped = union_non_empty(escaped, child_bounds.escaped);
    }

    if is_non_empty(&clipped) {
        if let Some(clip) = own_clip_bounds(style, cache, tree, entity) {
            clipped = clipped.intersection(&clip);
        }
    }
    let mut clipped = union_non_empty(own, clipped);

    if let Some(Filter::Blur(radius)) = style.filter.get(entity) {
        let radius = radius.to_px().unwrap() * style.scale_factor();
        own = own.expand(radius);
        clipped = clipped.expand(radius);
        if is_non_empty(&escaped) {
            escaped = escaped.expand(radius);
        }
    }

    let transform = cache.local_transform.get(entity).copied().unwrap_or_default();
    let own = map_bounds(&transform, own);
    let mut clipped = map_bounds(&transform, clipped);
    let mut escaped = map_bounds(&transform, escaped);

    // If z-index is not zero we can't be sure of the stacking context so we have to assume so and
    // not clip.
    let escapes_clip = tree.is_window(entity)
        || !should_apply_entity_clip(style, entity)
        || matches!(style.z_index.get(entity), Some(z_index) if *z_index != 0);

    if escapes_clip {
        escaped = union_non_empty(escaped, clipped);
        clipped = BoundingBox::default();
    }

    DrawBounds { own, clipped, escaped, escapes_clip }
}

/// Returns the layout bounds of an entity expanded by its shadows and outline.
fn decorated_bounds(style: &Style, cache: &CachedData, entity: Entity) -> BoundingBox {
    let mut layout_bounds = cache.bounds.get(entity).copied().unwrap_or_default();
    if !is_non_empty(&layout_bounds) {
        return BoundingBox::default();
    }

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

    layout_bounds.union(&outline_bounds)
}

/// The coordinate space of the parent of an entity being drawn.
#[derive(Clone, Copy)]
struct ParentSpace {
    /// The transform of the parent in window coordinates.
    transform: Matrix,
    /// The clip bounds of the parent in window coordinates.
    clip: Option<BoundingBox>,
    /// The dirty rect in the coordinate space of the parent, if the parent transform only scales
    /// and translates. Allows children to be culled using their cached draw bounds directly.
    local_dirty: Option<LocalDirty>,
}

#[derive(Clone, Copy)]
struct LocalDirty {
    /// The dirty rect intersected with the parent clip, for comparison with clipped bounds.
    clipped: BoundingBox,
    /// The dirty rect, for comparison with bounds which escape the parent clip.
    escaped: BoundingBox,
}

impl ParentSpace {
    fn new(transform: Matrix, clip: Option<BoundingBox>, dirty_rect: &Option<BoundingBox>) -> Self {
        let local_dirty = dirty_rect.and_then(|dirty| {
            let (sx, sy) = (transform.scale_x(), transform.scale_y());
            if !is_scale_translate(&transform) || sx == 0.0 || sy == 0.0 {
                return None;
            }
            let (tx, ty) = (transform.translate_x(), transform.translate_y());
            let to_local = |bounds: BoundingBox| {
                if !is_non_empty(&bounds) {
                    return BoundingBox::default();
                }
                // Window space bounds are expanded to whole pixels, so expand the dirty rect to
                // keep culling conservative.
                let bounds = bounds.expand(1.0);
                let (x0, x1) = ((bounds.left() - tx) / sx, (bounds.right() - tx) / sx);
                let (y0, y1) = ((bounds.top() - ty) / sy, (bounds.bottom() - ty) / sy);
                BoundingBox::from_min_max(x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1))
            };
            let clipped = clip.map_or(dirty, |clip| dirty.intersection(&clip));
            Some(LocalDirty { clipped: to_local(clipped), escaped: to_local(dirty) })
        });
        Self { transform, clip, local_dirty }
    }

    /// Returns true if the drawn bounds of an entity and its descendants may intersect the dirty
    /// rect.
    fn intersects_dirty(&self, bounds: &DrawBounds, dirty_rect: &BoundingBox) -> bool {
        match &self.local_dirty {
            Some(local) => {
                intersects_non_empty(&bounds.clipped, &local.clipped)
                    || intersects_non_empty(&bounds.escaped, &local.escaped)
            }
            None => intersects_non_empty(
                &subtree_to_screen(bounds, &self.transform, self.clip),
                dirty_rect,
            ),
        }
    }
}

struct ZEntity {
    pub index: i32,
    pub entity: Entity,
    pub visible: bool,
    parent: ParentSpace,
    pub order: usize,
    /// Ancestor alpha not supplied by the layers still open when the queue drains this view.
    pub ancestor_opacity: f32,
}

struct DrawQueue<'a> {
    raised: &'a mut BinaryHeap<ZEntity>,
    clipping_escapes: &'a mut BinaryHeap<ZEntity>,
    next_order: &'a mut usize,
    /// Alpha of the layers open between the owners of `clipping_escapes` and `raised`, which
    /// escapes drain outside of.
    escape_opacity: f32,
}

impl DrawQueue<'_> {
    fn entry(
        &mut self,
        index: i32,
        entity: Entity,
        visible: bool,
        ancestor_opacity: f32,
        parent: ParentSpace,
    ) -> ZEntity {
        let order = *self.next_order;
        *self.next_order += 1;
        ZEntity { index, entity, visible, parent, order, ancestor_opacity }
    }

    fn defer_raised(
        &mut self,
        index: i32,
        entity: Entity,
        visible: bool,
        ancestor_opacity: f32,
        parent: ParentSpace,
    ) {
        let entry = self.entry(index, entity, visible, ancestor_opacity, parent);
        self.raised.push(entry);
    }

    fn defer_clipping_escape(
        &mut self,
        index: i32,
        entity: Entity,
        visible: bool,
        ancestor_opacity: f32,
        parent: ParentSpace,
    ) {
        let entry = self.entry(index, entity, visible, ancestor_opacity, parent);
        self.clipping_escapes.push(entry);
    }
}

impl Ord for ZEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        other.index.cmp(&self.index).then_with(|| other.order.cmp(&self.order))
    }
}
impl PartialOrd for ZEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for ZEntity {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.order == other.order
    }
}

impl Eq for ZEntity {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::{is_identity, is_scale_translate};

    #[test]
    fn matrix_predicates_match_skia() {
        let matrices = [
            Matrix::new_identity(),
            Matrix::translate((3.0, -4.5)),
            Matrix::scale((-2.0, 0.5)),
            Matrix::rotate_deg(30.0),
        ];
        for matrix in matrices {
            assert_eq!(is_identity(&matrix), matrix.is_identity());
            assert_eq!(is_scale_translate(&matrix), matrix.is_scale_translate());
        }
    }

    #[test]
    fn scale_translate_fast_path_matches_skia() {
        let bounds = BoundingBox { x: 10.25, y: -3.5, w: 40.0, h: 20.75 };
        let mut matrix = Matrix::scale((-1.5, 2.0));
        matrix.post_translate((7.3, -2.1));
        assert_eq!(map_bounds(&matrix, bounds), snap(matrix.map_rect(Rect::from(bounds)).0));
    }

    #[test]
    fn parent_space_culling_is_conservative() {
        let mut transform = Matrix::scale((1.5, -0.75));
        transform.post_translate((13.3, 200.6));
        let clip = Some(BoundingBox { x: 20.0, y: 30.0, w: 120.0, h: 90.0 });

        for i in 0..400 {
            let f = i as f32;
            let dirty = BoundingBox {
                x: (f * 7.3) % 180.0,
                y: (f * 3.1) % 220.0,
                w: 1.0 + (f % 9.0),
                h: 1.0 + (f % 5.0),
            };
            let child = BoundingBox {
                x: (f * 5.7) % 140.0,
                y: (f * 11.9) % 260.0,
                w: 0.5 + (f % 13.0),
                h: 0.5 + (f % 7.0),
            };
            for escapes in [false, true] {
                let bounds = DrawBounds {
                    own: child,
                    clipped: if escapes { BoundingBox::default() } else { child },
                    escaped: if escapes { child } else { BoundingBox::default() },
                    escapes_clip: escapes,
                };
                let space = ParentSpace::new(transform, clip, &Some(dirty));
                assert!(space.local_dirty.is_some());
                let screen =
                    intersects_non_empty(&subtree_to_screen(&bounds, &transform, clip), &dirty);
                if screen {
                    assert!(space.intersects_dirty(&bounds, &dirty), "{i} {escapes}");
                }
            }
        }
    }

    #[test]
    fn removing_a_subtree_dirties_the_area_of_its_root() {
        let mut cx = Context::default();
        cx.windows.entry(Entity::root()).or_default();
        let parent = cx.with_current(Entity::root(), |cx| Element::new(cx).entity());
        let child = cx.with_current(parent, |cx| Element::new(cx).entity());
        cx.cache.set_bounds(parent, BoundingBox { x: 10.0, y: 10.0, w: 100.0, h: 100.0 });
        cx.cache.set_bounds(child, BoundingBox { x: 20.0, y: 20.0, w: 10.0, h: 10.0 });
        screen_bounds(&cx.style, &mut cx.cache, &cx.tree, Entity::root());
        cx.windows.get_mut(&Entity::root()).unwrap().dirty_rect = None;

        cx.remove(parent);

        let dirty = cx.windows[&Entity::root()].dirty_rect.unwrap();
        assert!(dirty.contains(&BoundingBox { x: 10.5, y: 10.5, w: 99.0, h: 99.0 }), "{dirty:?}");
    }

    #[test]
    fn removing_a_child_preserves_the_painted_area_of_its_ancestors() {
        let mut cx = Context::default();
        cx.windows.entry(Entity::root()).or_default();
        let parent = cx.with_current(Entity::root(), |cx| Element::new(cx).entity());
        let child = cx.with_current(parent, |cx| Element::new(cx).entity());
        cx.cache.set_bounds(parent, BoundingBox { x: 0.0, y: 0.0, w: 100.0, h: 140.0 });
        cx.cache.set_bounds(child, BoundingBox { x: 0.0, y: 20.0, w: 100.0, h: 100.0 });
        screen_bounds(&cx.style, &mut cx.cache, &cx.tree, Entity::root());
        cx.windows.get_mut(&Entity::root()).unwrap().dirty_rect = None;

        cx.remove(child);
        cx.cache.set_bounds(parent, BoundingBox { x: 0.0, y: 0.0, w: 100.0, h: 40.0 });

        let dirty = cx.windows[&Entity::root()].dirty_rect.unwrap();
        assert!(dirty.contains_point(50.0, 139.0), "{dirty:?}");
    }

    #[test]
    fn resizing_a_window_invalidates_window_dependent_draw_bounds() {
        use crate::backend::BackendContext;

        let mut backend = BackendContext::new(Context::default());
        backend.add_main_window(Entity::root(), &Default::default(), 1.0);
        let cx = &mut backend.0;
        cx.windows.entry(Entity::root()).or_default();
        cx.cache.set_bounds(Entity::root(), BoundingBox { x: 0.0, y: 0.0, w: 100.0, h: 100.0 });
        let clipper = Element::new(cx).overflowx(Overflow::Hidden).entity();
        let child = cx.with_current(clipper, |cx| Element::new(cx).entity());
        cx.cache.set_bounds(clipper, BoundingBox { x: 0.0, y: 0.0, w: 50.0, h: 50.0 });
        cx.cache.set_bounds(child, BoundingBox { x: 0.0, y: 120.0, w: 20.0, h: 20.0 });
        let before = screen_bounds(&cx.style, &mut cx.cache, &cx.tree, clipper);
        assert!(!before.contains_point(10.0, 130.0));

        backend.set_window_size(Entity::root(), 100.0, 200.0);

        let cx = &mut backend.0;
        let after = screen_bounds(&cx.style, &mut cx.cache, &cx.tree, clipper);
        assert!(after.contains_point(10.0, 130.0), "{after:?}");
    }

    const RED: [u8; 4] = [255, 0, 0, 255];

    /// Styles a `.probe` view, a child of a `.parent` view, with `css`, paints the probe's shadows
    /// and background with its bounds at (10, 10, 20, 20) on a 60x40 surface, and returns a lookup
    /// of the painted pixel at (x, y).
    fn paint_probe(css: &'static str) -> impl Fn(usize, usize) -> [u8; 4] {
        use skia_safe::{AlphaType, ColorType, ImageInfo};

        let mut cx = Context::default();
        cx.add_stylesheet(css).unwrap();
        let parent = Element::new(&mut cx).class("parent").entity();
        let probe = cx.with_current(parent, |cx| Element::new(cx).class("probe").entity());
        crate::systems::style_system(&mut cx);
        cx.cache.set_bounds(probe, BoundingBox { x: 10.0, y: 10.0, w: 20.0, h: 20.0 });

        let info = ImageInfo::new((60, 40), ColorType::RGBA8888, AlphaType::Premul, None);
        let mut surface = skia_safe::surfaces::raster(&info, None, None).unwrap();
        let mut draw_cx = DrawContext {
            current: probe,
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
        };
        draw_cx.draw_shadows(surface.canvas());
        draw_cx.draw_background(surface.canvas());

        let mut pixels = vec![0u8; 60 * 40 * 4];
        assert!(surface.read_pixels(&info, &mut pixels, 60 * 4, (0, 0)));
        move |x, y| {
            let i = (y * 60 + x) * 4;
            [pixels[i], pixels[i + 1], pixels[i + 2], pixels[i + 3]]
        }
    }

    #[test]
    fn currentcolor_background_follows_the_font_color() {
        let pixel = paint_probe(".probe { color: red; background-color: currentcolor; }");
        assert_eq!(pixel(20, 20), RED);
    }

    #[test]
    fn currentcolor_shadow_follows_the_font_color() {
        let pixel = paint_probe(".probe { color: red; shadow: 20px 0px currentcolor; }");
        assert_eq!(pixel(40, 20), RED);
    }

    #[test]
    fn shadow_without_a_color_follows_the_font_color() {
        let pixel = paint_probe(".probe { color: red; shadow: 20px 0px; }");
        assert_eq!(pixel(40, 20), RED);
    }

    #[test]
    fn currentcolor_follows_an_inherited_font_color() {
        let pixel = paint_probe(
            ".parent { color: red; }
            .probe { background-color: currentcolor; shadow: 20px 0px currentcolor; }",
        );
        assert_eq!(pixel(20, 20), RED);
        assert_eq!(pixel(40, 20), RED);
    }

    #[test]
    fn each_shadow_in_a_list_resolves_its_own_color() {
        // A 2px spread ring without a color around the box, over an explicit offset shadow.
        let pixel =
            paint_probe(".probe { color: red; shadow: 0px 0px 0px 2px, 20px 0px rgb(0, 255, 0); }");
        // Inside the ring on the left, where the offset shadow does not reach.
        assert_eq!(pixel(9, 20), RED);
        // Inside the offset shadow, beyond the ring on the right.
        assert_eq!(pixel(40, 20), [0, 255, 0, 255]);
    }

    #[test]
    fn explicit_background_and_shadow_colors_ignore_the_font_color() {
        let pixel = paint_probe(
            ".probe { color: red; background-color: blue; shadow: 20px 0px rgb(0, 255, 0); }",
        );
        assert_eq!(pixel(20, 20), [0, 0, 255, 255]);
        assert_eq!(pixel(40, 20), [0, 255, 0, 255]);

        let pixel = paint_probe(".probe { color: red; shadow: 20px 0px transparent; }");
        assert_eq!(pixel(40, 20), [0, 0, 0, 0]);
    }

    use crate::backend::BackendContext;

    fn renderer_rect(cx: &mut Context, x: f32, width: f32, color: Color) -> Entity {
        Element::new(cx)
            .position_type(PositionType::Absolute)
            .left(Pixels(x))
            .top(Pixels(0.0))
            .width(Pixels(width))
            .height(Pixels(16.0))
            .background_color(color)
            .entity()
    }

    fn renderer_backend(cx: Context) -> BackendContext {
        let mut backend = BackendContext::new(cx);
        backend.0.windows.insert(Entity::root(), WindowState::default());
        backend.set_window_size(Entity::root(), 32.0, 16.0);
        backend.process_style_updates();
        backend.process_visual_updates();
        backend
    }

    fn renderer_surfaces() -> (skia_safe::Surface, skia_safe::Surface) {
        (
            skia_safe::surfaces::raster_n32_premul((32, 16)).unwrap(),
            skia_safe::surfaces::raster_n32_premul((32, 16)).unwrap(),
        )
    }

    // Raised overlapping descendants composite once inside each nested opacity group.
    #[test]
    fn renderer_group_opacity_composites_raised_overlap_once() {
        for (outer_opacity, inner_opacity, alpha) in [(0.5, 1.0, 128), (0.55, 0.6, 84)] {
            let mut cx = Context::new();
            let outer = renderer_rect(&mut cx, 0.0, 24.0, Color::transparent());
            cx.style.opacity.insert(outer, outer_opacity.into());
            let inner =
                cx.with_current(outer, |cx| renderer_rect(cx, 0.0, 24.0, Color::transparent()));
            cx.style.opacity.insert(inner, inner_opacity.into());
            cx.with_current(inner, |cx| {
                renderer_rect(cx, 0.0, 16.0, Color::red());
                let raised = renderer_rect(cx, 8.0, 16.0, Color::blue());
                cx.style.z_index.insert(raised, 5);
            });
            let mut backend = renderer_backend(cx);
            let (mut surface, mut dirty) = renderer_surfaces();
            assert!(backend.draw(Entity::root(), &mut surface, &mut dirty));
            let pixels = surface.peek_pixels().unwrap();
            assert_eq!(pixels.get_color((4, 8)), skia_safe::Color::from_argb(alpha, 255, 0, 0));
            assert_eq!(pixels.get_color((12, 8)), skia_safe::Color::from_argb(alpha, 0, 0, 255));
            assert_eq!(pixels.get_color((20, 8)), skia_safe::Color::from_argb(alpha, 0, 0, 255));
            assert_eq!(pixels.get_color((28, 8)), skia_safe::Color::TRANSPARENT);
        }
    }

    // A raised opacity group retains sibling order and equal-z descendants retain tree order.
    #[test]
    fn renderer_group_opacity_preserves_sibling_and_equal_z_order() {
        let mut cx = Context::new();
        let group = renderer_rect(&mut cx, 0.0, 24.0, Color::transparent());
        cx.style.opacity.insert(group, 0.5.into());
        cx.style.z_index.insert(group, 2);
        cx.with_current(group, |cx| {
            for color in [Color::red(), Color::green(), Color::blue(), Color::white()] {
                let raised = renderer_rect(cx, 0.0, 24.0, color);
                cx.style.z_index.insert(raised, 5);
            }
        });
        let behind = renderer_rect(&mut cx, 0.0, 8.0, Color::red());
        cx.style.z_index.insert(behind, 1);
        let ahead = renderer_rect(&mut cx, 16.0, 8.0, Color::green());
        cx.style.z_index.insert(ahead, 3);
        let mut backend = renderer_backend(cx);
        let (mut surface, mut dirty) = renderer_surfaces();
        assert!(backend.draw(Entity::root(), &mut surface, &mut dirty));
        let pixels = surface.peek_pixels().unwrap();
        assert_eq!(pixels.get_color((4, 8)), skia_safe::Color::from_rgb(255, 128, 128));
        assert_eq!(pixels.get_color((12, 8)), skia_safe::Color::from_argb(128, 255, 255, 255));
        assert_eq!(pixels.get_color((20, 8)), skia_safe::Color::from_rgb(0, 128, 0));
    }

    // Dirty redraws apply changed group opacity and preserve hidden inheritance with a visible override.
    #[test]
    fn renderer_group_opacity_redraws_and_preserves_visibility() {
        let mut cx = Context::new();
        let group = renderer_rect(&mut cx, 0.0, 24.0, Color::transparent());
        cx.style.opacity.insert(group, 0.5.into());
        let raised = cx.with_current(group, |cx| renderer_rect(cx, 0.0, 24.0, Color::blue()));
        cx.style.z_index.insert(raised, 5);
        let visible = cx.with_current(group, |cx| renderer_rect(cx, 16.0, 8.0, Color::red()));
        cx.style.z_index.insert(visible, 6);
        cx.style.visibility.insert(visible, Visibility::Visible);
        let mut backend = renderer_backend(cx);
        let (mut surface, mut dirty) = renderer_surfaces();
        for (opacity, hidden, expected) in
            [(0.5, false, 128), (0.25, false, 64), (0.25, true, 0), (0.5, false, 128)]
        {
            backend.0.style.opacity.insert(group, opacity.into());
            backend
                .0
                .style
                .visibility
                .insert(group, if hidden { Visibility::Hidden } else { Visibility::Visible });
            backend.0.needs_redraw(group);
            backend.process_style_updates();
            backend.process_visual_updates();
            assert!(backend.draw(Entity::root(), &mut surface, &mut dirty));
            let pixels = surface.peek_pixels().unwrap();
            assert_eq!(pixels.get_color((4, 8)).a(), expected);
            assert_eq!(
                pixels.get_color((20, 8)),
                skia_safe::Color::from_argb(if opacity == 0.5 { 128 } else { 64 }, 255, 0, 0,)
            );
        }
    }

    // Backdrop clipping bounds ordinary raised content while a disjoint escaping overlay inherits each ancestor and its own opacity exactly once.
    #[test]
    fn renderer_group_opacity_keeps_backdrop_clipping_escape_separate() {
        for (outer, inner, own, blue_alpha, red_alpha) in
            [(1.0, 0.5, 1.0, 128, 128), (0.55, 0.6, 0.5, 84, 42)]
        {
            let mut cx = Context::new();
            let backdrop = renderer_rect(&mut cx, 0.0, 16.0, Color::transparent());
            cx.style.opacity.insert(backdrop, outer.into());
            cx.style.backdrop_filter.insert(backdrop, Filter::Blur(Length::px(0.0)));
            cx.style.overflowx.insert(backdrop, Overflow::Hidden);
            cx.style.overflowy.insert(backdrop, Overflow::Hidden);
            let group =
                cx.with_current(backdrop, |cx| renderer_rect(cx, 0.0, 24.0, Color::transparent()));
            cx.style.opacity.insert(group, inner.into());
            cx.with_current(group, |cx| {
                let raised = renderer_rect(cx, 0.0, 24.0, Color::blue());
                cx.style.z_index.insert(raised, 5);
                Element::new(cx)
                    .position_type(PositionType::Absolute)
                    .left(Pixels(24.0))
                    .top(Pixels(0.0))
                    .width(Pixels(8.0))
                    .height(Pixels(16.0))
                    .background_color(Color::red())
                    .opacity(own)
                    .ignore_clipping(true);
            });
            let mut backend = renderer_backend(cx);
            let (mut surface, mut dirty) = renderer_surfaces();
            assert!(backend.draw(Entity::root(), &mut surface, &mut dirty));
            let pixels = surface.peek_pixels().unwrap();
            assert_eq!(
                pixels.get_color((8, 8)),
                skia_safe::Color::from_argb(blue_alpha, 0, 0, 255)
            );
            assert_eq!(pixels.get_color((20, 8)), skia_safe::Color::TRANSPARENT);
            assert_eq!(
                pixels.get_color((28, 8)),
                skia_safe::Color::from_argb(red_alpha, 255, 0, 0)
            );
        }
    }

    // A group outside a backdrop clip composites an escape from it once and keeps it beneath raised content.
    #[test]
    fn renderer_group_opacity_owns_clipping_escape_from_inner_backdrop() {
        let mut cx = Context::new();
        let group = renderer_rect(&mut cx, 0.0, 32.0, Color::transparent());
        cx.style.opacity.insert(group, 0.5.into());
        let backdrop =
            cx.with_current(group, |cx| renderer_rect(cx, 0.0, 16.0, Color::transparent()));
        cx.style.backdrop_filter.insert(backdrop, Filter::Blur(Length::px(0.0)));
        cx.style.overflowx.insert(backdrop, Overflow::Hidden);
        cx.style.overflowy.insert(backdrop, Overflow::Hidden);
        cx.with_current(backdrop, |cx| {
            let raised = renderer_rect(cx, 0.0, 24.0, Color::blue());
            cx.style.z_index.insert(raised, 5);
            let escape = renderer_rect(cx, 8.0, 16.0, Color::red());
            cx.style.ignore_clipping.insert(escape, true);
        });
        let mut backend = renderer_backend(cx);
        let (mut surface, mut dirty) = renderer_surfaces();
        assert!(backend.draw(Entity::root(), &mut surface, &mut dirty));
        let pixels = surface.peek_pixels().unwrap();
        assert_eq!(pixels.get_color((4, 8)), skia_safe::Color::from_argb(128, 0, 0, 255));
        assert_eq!(pixels.get_color((12, 8)), skia_safe::Color::from_argb(128, 0, 0, 255));
        assert_eq!(pixels.get_color((20, 8)), skia_safe::Color::from_argb(128, 255, 0, 0));
        assert_eq!(pixels.get_color((28, 8)), skia_safe::Color::TRANSPARENT);
    }

    // A raised child inherits the opacity of every ancestor between it and the group draining it.
    #[test]
    fn renderer_group_opacity_restores_closed_intermediate_opacity() {
        for intermediate in ["filter", "backdrop", "blend"] {
            let mut cx = Context::new();
            let outer = renderer_rect(&mut cx, 0.0, 24.0, Color::transparent());
            cx.style.opacity.insert(outer, 0.5.into());
            let inner =
                cx.with_current(outer, |cx| renderer_rect(cx, 0.0, 24.0, Color::transparent()));
            cx.style.opacity.insert(inner, 0.5.into());
            match intermediate {
                "filter" => cx.style.filter.insert(inner, Filter::Blur(Length::px(0.0))),
                "backdrop" => cx.style.backdrop_filter.insert(inner, Filter::Blur(Length::px(0.0))),
                _ => cx.style.blend_mode.insert(inner, BlendMode::Multiply),
            }
            cx.with_current(inner, |cx| {
                renderer_rect(cx, 0.0, 8.0, Color::red());
                let raised = renderer_rect(cx, 8.0, 16.0, Color::blue());
                cx.style.z_index.insert(raised, 5);
            });
            let mut backend = renderer_backend(cx);
            let (mut surface, mut dirty) = renderer_surfaces();
            assert!(backend.draw(Entity::root(), &mut surface, &mut dirty));
            let pixels = surface.peek_pixels().unwrap();
            assert_eq!(
                pixels.get_color((4, 8)),
                skia_safe::Color::from_argb(64, 255, 0, 0),
                "{intermediate}"
            );
            assert_eq!(
                pixels.get_color((12, 8)),
                skia_safe::Color::from_argb(64, 0, 0, 255),
                "{intermediate}"
            );
        }
    }

    // A clip escape below nested opacity groups inherits the opacity of each group once.
    #[test]
    fn renderer_group_opacity_applies_nested_group_opacity_to_clipping_escape() {
        let mut cx = Context::new();
        let outer = renderer_rect(&mut cx, 0.0, 32.0, Color::transparent());
        cx.style.opacity.insert(outer, 0.5.into());
        let backdrop =
            cx.with_current(outer, |cx| renderer_rect(cx, 0.0, 16.0, Color::transparent()));
        cx.style.backdrop_filter.insert(backdrop, Filter::Blur(Length::px(0.0)));
        cx.style.overflowx.insert(backdrop, Overflow::Hidden);
        cx.style.overflowy.insert(backdrop, Overflow::Hidden);
        let inner =
            cx.with_current(backdrop, |cx| renderer_rect(cx, 0.0, 32.0, Color::transparent()));
        cx.style.opacity.insert(inner, 0.5.into());
        let innermost =
            cx.with_current(inner, |cx| renderer_rect(cx, 0.0, 32.0, Color::transparent()));
        cx.style.opacity.insert(innermost, 0.5.into());
        cx.with_current(innermost, |cx| {
            renderer_rect(cx, 0.0, 8.0, Color::blue());
            let escape = renderer_rect(cx, 24.0, 8.0, Color::red());
            cx.style.ignore_clipping.insert(escape, true);
        });
        let mut backend = renderer_backend(cx);
        let (mut surface, mut dirty) = renderer_surfaces();
        assert!(backend.draw(Entity::root(), &mut surface, &mut dirty));
        let pixels = surface.peek_pixels().unwrap();
        assert_eq!(pixels.get_color((4, 8)), skia_safe::Color::from_argb(32, 0, 0, 255));
        assert_eq!(pixels.get_color((28, 8)), skia_safe::Color::from_argb(32, 255, 0, 0));
    }

    // A z-lifted child under a half-opaque parent dims with it, as an un-lifted sibling does.
    #[test]
    fn z_lifted_child_dims_with_translucent_parent() {
        let mut cx = Context::new();
        cx.windows.insert(Entity::root(), WindowState::default());
        let (mut plain, mut lifted) = (Entity::null(), Entity::null());
        let parent = HStack::new(&mut cx, |cx| {
            plain = Element::new(cx).background_color(Color::white()).entity();
            lifted = Element::new(cx).background_color(Color::white()).z_index(5).entity();
        })
        .opacity(0.5)
        .entity();
        let mut backend = BackendContext::new(cx);
        backend.process_style_updates();
        backend.0.cache.set_bounds(Entity::root(), BoundingBox::from_min_max(0.0, 0.0, 16.0, 16.0));
        backend.0.cache.set_bounds(parent, BoundingBox::from_min_max(0.0, 0.0, 16.0, 16.0));
        backend.0.cache.set_bounds(plain, BoundingBox::from_min_max(0.0, 0.0, 8.0, 16.0));
        backend.0.cache.set_bounds(lifted, BoundingBox::from_min_max(8.0, 0.0, 16.0, 16.0));
        let mut surface = skia_safe::surfaces::raster_n32_premul((16, 16)).unwrap();
        let mut dirty = skia_safe::surfaces::raster_n32_premul((16, 16)).unwrap();
        backend.draw(Entity::root(), &mut surface, &mut dirty);
        let pixels = surface.peek_pixels().unwrap();
        let (plain_a, lifted_a) = (pixels.get_color((4, 8)).a(), pixels.get_color((12, 8)).a());
        assert!((126..=129).contains(&plain_a), "un-lifted sibling alpha {plain_a}");
        assert!((126..=129).contains(&lifted_a), "z-lifted child alpha {lifted_a}");
    }
}

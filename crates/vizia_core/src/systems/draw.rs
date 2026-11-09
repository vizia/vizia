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

    let mut queue = BinaryHeap::new();
    // With no dirty area nothing visible has changed, so the retained dirty surface is still
    // valid and only needs compositing. Surface recreation (e.g. on resize) always dirties the
    // whole window via `needs_refresh` or `needs_resize_redraw`.
    if dirty_rect.is_some() {
        queue.push(ZEntity {
            index: 0,
            entity: window_entity,
            visible: true,
            parent: ParentSpace::new(Matrix::new_identity(), None, &dirty_rect),
        });
    }

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
            false,
            &zentity.parent,
        );
        canvas.restore();
    }

    canvas.restore();

    surface.canvas().clear(Color::transparent());
    dirty_surface.draw(surface.canvas(), (0, 0), SamplingOptions::default(), None);

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
    queue: &mut BinaryHeap<ZEntity>,
    visible: bool,
    ancestor_backdrop_clip: bool,
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

    let defer = |queue: &mut BinaryHeap<ZEntity>, index: i32| {
        queue.push(ZEntity { index, entity: current, visible, parent: *parent });
    };

    if z_index > current_z {
        defer(queue, z_index);
        return;
    }

    // A Skia clip cannot be expanded until its saved canvas state is restored. Defer views which
    // explicitly escape ancestor clipping so a backdrop clip higher in the recursive draw cannot
    // accidentally constrain popovers and other overlays.
    if ancestor_backdrop_clip && !should_apply_entity_clip(cx.style, current) {
        defer(queue, current_z.max(z_index));
        return;
    }

    let transform = match cx.cache.local_transform.get(current) {
        Some(local) if !is_identity(local) => parent.transform * *local,
        _ => parent.transform,
    };

    let filter = cx.filter();
    let backdrop_filter = cx.backdrop_filter();
    let blend_mode = cx.style.blend_mode.get(current).copied().unwrap_or_default();

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

    // Draw its children
    for child in child_iter {
        cx.current = child;
        // TODO: Skip views with zero-sized bounding boxes here? Or let user decide if they want to skip?
        draw_entity(
            cx,
            dirty_rect,
            canvas,
            current_z,
            queue,
            is_visible,
            descendants_are_backdrop_clipped,
            &child_space,
        );
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
}

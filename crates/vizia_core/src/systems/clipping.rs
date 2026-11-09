use skia_safe::{Path, PathBuilder, Point, RRect, Rect};
use vizia_storage::LayoutChildIterator;

use crate::{prelude::*, tree::minimal_layout_dirty_roots};

/// Computes the clipping path for each entity in the layout tree.
///
/// Callers only mark the entities whose own clip inputs changed. Starting from the topmost marked
/// entities, the system recomputes the clip path of each marked entity, and of every descendant
/// whose parent clip path changed.
pub(crate) fn clipping_system(cx: &mut Context) {
    let _scope = crate::profiling::scope("clipping_system");
    if cx.style.reclip.is_empty() {
        return;
    }

    let reclip = std::mem::take(&mut cx.style.reclip);

    // Each entry holds an entity, whether the clip path of its parent changed, and whether its
    // clip path must be recomputed regardless.
    let mut stack: Vec<(Entity, bool, bool)> = minimal_layout_dirty_roots(&cx.tree, &reclip)
        .into_iter()
        .map(|root| (root, false, false))
        .collect();

    while let Some((entity, parent_changed, forced)) = stack.pop() {
        let marked = reclip.contains(&entity);
        let changed = (forced || parent_changed || marked) && update_clip_path(cx, entity);

        // Clip paths with a single hidden axis extend to the window bounds, so a change to a window
        // recomputes everything within it.
        let forced = forced || (marked && cx.tree.is_window(entity));

        stack.extend(
            LayoutChildIterator::new(&cx.tree, entity).map(|child| (child, changed, forced)),
        );
    }
}

/// Returns true if the entity clips its content, rather than only inheriting the clip path of its
/// parent.
pub(crate) fn has_own_clip(style: &Style, entity: Entity) -> bool {
    matches!(style.overflowx.get(entity), Some(Overflow::Hidden))
        || matches!(style.overflowy.get(entity), Some(Overflow::Hidden))
        || matches!(style.clip_path.get(entity), Some(ClipPath::Shape(_)))
}

/// Recomputes the clip path of an entity. Returns true if the clip path changed.
fn update_clip_path(cx: &mut Context, entity: Entity) -> bool {
    let Some(bounds) = cx.cache.bounds.get(entity).copied() else { return false };
    let clip_path = compute_clip_path(cx, entity, bounds);

    let previous = cx.cache.clip_path.get(entity);
    if previous == Some(&clip_path) {
        return false;
    }

    let previous_bounds = previous.and_then(Option::as_ref).map(|path| *path.bounds());
    let new_bounds = clip_path.as_ref().map(|path| *path.bounds());
    cx.cache.clip_path.insert(entity, clip_path);

    // The cached draw bounds of children are limited by the clip path of their parent.
    if previous_bounds != new_bounds {
        for child in LayoutChildIterator::new(&cx.tree, entity) {
            cx.cache.invalidate_draw_bounds(&cx.tree, child);
        }
    }

    true
}

/// Computes the clip path of an entity, intersecting its own clip with the clip of its parent.
fn compute_clip_path(cx: &Context, entity: Entity, bounds: BoundingBox) -> Option<Path> {
    if entity == Entity::root() {
        return Some(build_clip_path(cx, entity, bounds));
    }

    let parent = cx.tree.get_layout_parent(entity).unwrap_or(Entity::root());
    let ignore_clipping = cx.style.ignore_clipping.get(entity).copied().unwrap_or(false);
    let parent_clip_path = if ignore_clipping || cx.tree.is_window(entity) {
        None
    } else {
        cx.cache.clip_path.get(parent).cloned().flatten()
    };

    if !has_own_clip(&cx.style, entity) {
        return parent_clip_path;
    }

    let overflowx = cx.style.overflowx.get(entity).copied().unwrap_or_default();
    let overflowy = cx.style.overflowy.get(entity).copied().unwrap_or_default();

    let scale = cx.style.scale_factor();

    let transform =
        cx.cache.transform.get(entity).copied().unwrap_or(skia_safe::Matrix::new_identity());

    let shape_clip_path = cx.style.clip_path.get(entity).and_then(|clip| match clip {
        ClipPath::Auto => None,
        ClipPath::Shape(rect) => {
            let clip_bounds = bounds.shrink_sides(
                rect.3.to_pixels(bounds.w, scale),
                rect.0.to_pixels(bounds.h, scale),
                rect.1.to_pixels(bounds.w, scale),
                rect.2.to_pixels(bounds.h, scale),
            );

            Some(build_clip_path(cx, entity, clip_bounds))
        }
    });

    let window_entity = if cx.tree.is_window(entity) {
        entity
    } else {
        cx.tree.get_parent_window(entity).unwrap_or(Entity::root())
    };
    let window_bounds = cx.cache.bounds.get(window_entity).copied().unwrap_or(bounds);

    let overflow_clip_path = match (overflowx, overflowy) {
        (Overflow::Visible, Overflow::Visible) => None,
        (Overflow::Hidden, Overflow::Visible) => {
            let left = bounds.left();
            let right = bounds.right();
            let top = window_bounds.top();
            let bottom = window_bounds.bottom();
            let clip_bounds = BoundingBox::from_min_max(left, top, right, bottom);
            Some(build_clip_path(cx, entity, clip_bounds))
        }
        (Overflow::Visible, Overflow::Hidden) => {
            let left = window_bounds.left();
            let right = window_bounds.right();
            let top = bounds.top();
            let bottom = bounds.bottom();
            let clip_bounds = BoundingBox::from_min_max(left, top, right, bottom);
            Some(build_clip_path(cx, entity, clip_bounds))
        }
        (Overflow::Hidden, Overflow::Hidden) => Some(build_clip_path(cx, entity, bounds)),
    };

    let clip_path = match (overflow_clip_path, shape_clip_path) {
        (Some(overflow_path), Some(shape_path)) => {
            overflow_path.op(&shape_path, skia_safe::PathOp::Intersect)
        }
        (Some(overflow_path), None) => Some(overflow_path),
        (None, Some(shape_path)) => Some(shape_path),
        (None, None) => None,
    };

    let clip_path = clip_path.map(|clip_path| clip_path.make_transform(&transform));

    match (clip_path, parent_clip_path) {
        (Some(clip_path), Some(parent_clip_path)) => {
            clip_path.op(&parent_clip_path, skia_safe::PathOp::Intersect)
        }
        (Some(clip_path), None) => Some(clip_path),
        (None, Some(parent_clip_path)) => Some(parent_clip_path),
        (None, None) => None,
    }
}
fn build_clip_path(cx: &Context, entity: Entity, clip_bounds: BoundingBox) -> skia_safe::Path {
    let bounds = cx.cache.bounds.get(entity).copied().unwrap_or_default();
    let scale = cx.style.scale_factor();
    let radius = |value: Option<&LengthOrPercentage>| {
        value.map(|length| length.to_pixels(bounds.w.min(bounds.h), scale)).unwrap_or_default()
    };
    let radii = [
        radius(cx.style.corner_top_left_radius.get(entity)),
        radius(cx.style.corner_top_right_radius.get(entity)),
        radius(cx.style.corner_bottom_right_radius.get(entity)),
        radius(cx.style.corner_bottom_left_radius.get(entity)),
    ];
    let rrect = RRect::new_rect_radii(
        Rect::from(clip_bounds),
        &[
            Point::new(radii[0], radii[0]),
            Point::new(radii[1], radii[1]),
            Point::new(radii[2], radii[2]),
            Point::new(radii[3], radii[3]),
        ],
    );
    let mut path = PathBuilder::new();
    path.add_rrect(rrect, None, None);
    path.detach()
}

#[cfg(test)]
mod tests {
    use super::clipping_system;
    use crate::prelude::*;
    use skia_safe::Rect;

    fn element(cx: &mut Context, parent: Entity, bounds: BoundingBox) -> Entity {
        let entity = cx.with_current(parent, |cx| Element::new(cx).entity());
        cx.cache.set_bounds(entity, bounds);
        entity
    }

    fn clip_bounds(cx: &Context, entity: Entity) -> Option<Rect> {
        cx.cache.clip_path.get(entity).cloned().flatten().map(|path| *path.bounds())
    }

    /// Builds root > a > b > c, where only `a` clips its content, and computes the clip paths.
    fn setup() -> (Context, Entity, Entity, Entity) {
        let mut cx = Context::default();
        cx.cache.set_bounds(Entity::root(), BoundingBox { x: 0.0, y: 0.0, w: 1000.0, h: 1000.0 });
        let a =
            element(&mut cx, Entity::root(), BoundingBox { x: 10.0, y: 10.0, w: 100.0, h: 100.0 });
        cx.style.overflowx.insert(a, Overflow::Hidden);
        cx.style.overflowy.insert(a, Overflow::Hidden);
        let b = element(&mut cx, a, BoundingBox { x: 10.0, y: 10.0, w: 500.0, h: 500.0 });
        let c = element(&mut cx, b, BoundingBox { x: 10.0, y: 10.0, w: 500.0, h: 500.0 });
        cx.style.needs_reclip(Entity::root());
        clipping_system(&mut cx);
        (cx, a, b, c)
    }

    fn cache_draw_bounds(cx: &mut Context, entities: &[Entity]) {
        for &entity in entities {
            cx.cache.draw_bounds.insert(entity, BoundingBox { x: 0.0, y: 0.0, w: 1.0, h: 1.0 });
        }
    }

    #[test]
    fn descendants_inherit_parent_clip() {
        let (cx, a, b, c) = setup();
        let expected = Some(Rect::new(10.0, 10.0, 110.0, 110.0));
        assert_eq!(clip_bounds(&cx, a), expected);
        assert_eq!(clip_bounds(&cx, b), expected);
        assert_eq!(clip_bounds(&cx, c), expected);
    }

    #[test]
    fn clip_change_propagates_to_unmarked_descendants() {
        let (mut cx, a, b, c) = setup();
        cache_draw_bounds(&mut cx, &[b, c]);
        cx.cache.set_bounds(a, BoundingBox { x: 10.0, y: 10.0, w: 50.0, h: 50.0 });
        cx.style.needs_reclip(a);
        clipping_system(&mut cx);

        assert_eq!(clip_bounds(&cx, c), Some(Rect::new(10.0, 10.0, 60.0, 60.0)));
        assert!(cx.style.reclip.is_empty());
        assert!(!cx.cache.draw_bounds.contains(b));
        assert!(!cx.cache.draw_bounds.contains(c));
    }

    #[test]
    fn unchanged_clip_keeps_descendant_draw_bounds() {
        let (mut cx, a, b, c) = setup();
        cache_draw_bounds(&mut cx, &[b, c]);
        cx.style.needs_reclip(a);
        clipping_system(&mut cx);

        assert!(cx.cache.draw_bounds.contains(b));
        assert!(cx.cache.draw_bounds.contains(c));
    }
}

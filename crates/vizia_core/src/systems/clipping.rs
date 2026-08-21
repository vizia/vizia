use std::collections::HashSet;

use skia_safe::{PathBuilder, Point, RRect, Rect};
use vizia_storage::LayoutTreeIterator;

use crate::{prelude::*, tree::minimal_layout_dirty_roots};

/// Computes the clipping path for each entity in the layout tree.
pub(crate) fn clipping_system(cx: &mut Context) {
    if cx.style.reclip.is_empty() {
        return;
    }

    let dirty_subtree_roots = minimal_layout_dirty_roots(&cx.tree, &cx.style.reclip);
    let mut invalidated_subtrees = HashSet::new();

    for &root in &dirty_subtree_roots {
        let iter = LayoutTreeIterator::subtree(&cx.tree, root);
        for entity in iter {
            if !cx.style.reclip.contains(&entity) {
                continue;
            }

            let parent = cx.tree.get_layout_parent(entity).unwrap_or(Entity::root());

            let Some(bounds) = cx.cache.bounds.get(entity).copied() else { continue };
            let previous_clip_bounds = cx
                .cache
                .clip_path
                .get(entity)
                .and_then(|clip_path| clip_path.as_ref())
                .map(|clip_path| *clip_path.bounds());

            if entity == Entity::root() {
                let clip_path = build_clip_path(cx, Entity::root(), bounds);
                cx.cache.clip_path.insert(entity, Some(clip_path));
                let new_clip_bounds = cx
                    .cache
                    .clip_path
                    .get(entity)
                    .and_then(|clip_path| clip_path.as_ref())
                    .map(|clip_path| *clip_path.bounds());

                let should_invalidate_subtree = should_invalidate_subtree(
                    cx,
                    entity,
                    &dirty_subtree_roots,
                    &invalidated_subtrees,
                );

                if previous_clip_bounds != new_clip_bounds && should_invalidate_subtree {
                    for descendant in LayoutTreeIterator::subtree(&cx.tree, entity).skip(1) {
                        cx.cache.draw_bounds.remove(descendant);
                    }
                    invalidated_subtrees.insert(entity);
                }
                continue;
            }

            let overflowx = cx.style.overflowx.get(entity).copied().unwrap_or_default();
            let overflowy = cx.style.overflowy.get(entity).copied().unwrap_or_default();

            let scale = cx.style.scale_factor();

            let transform = cx
                .cache
                .transform
                .get(entity)
                .copied()
                .unwrap_or(skia_safe::Matrix::new_identity());

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

            let ignore_clipping = cx.style.ignore_clipping.get(entity).copied().unwrap_or(false);

            let parent_clip_path = if ignore_clipping || cx.tree.is_window(entity) {
                None
            } else {
                cx.cache.clip_path.get(parent).cloned().flatten()
            };

            let effective_clip_path = match (clip_path, parent_clip_path) {
                (Some(clip_path), Some(parent_clip_path)) => {
                    clip_path.op(&parent_clip_path, skia_safe::PathOp::Intersect)
                }
                (Some(clip_path), None) => Some(clip_path),
                (None, Some(parent_clip_path)) => Some(parent_clip_path),
                (None, None) => None,
            };

            let new_clip_bounds = effective_clip_path.as_ref().map(|clip_path| *clip_path.bounds());
            cx.cache.clip_path.insert(entity, effective_clip_path);
            let should_invalidate_subtree =
                should_invalidate_subtree(cx, entity, &dirty_subtree_roots, &invalidated_subtrees);

            if previous_clip_bounds != new_clip_bounds && should_invalidate_subtree {
                for descendant in LayoutTreeIterator::subtree(&cx.tree, entity).skip(1) {
                    cx.cache.draw_bounds.remove(descendant);
                }
                invalidated_subtrees.insert(entity);
            }
        }
    }

    cx.style.reclip.clear();
}

fn should_invalidate_subtree(
    cx: &Context,
    entity: Entity,
    dirty_subtree_roots: &HashSet<Entity>,
    invalidated_subtrees: &HashSet<Entity>,
) -> bool {
    dirty_subtree_roots.contains(&entity)
        || !has_invalidated_layout_ancestor(cx, entity, invalidated_subtrees)
}

fn has_invalidated_layout_ancestor(
    cx: &Context,
    entity: Entity,
    invalidated_subtrees: &HashSet<Entity>,
) -> bool {
    let mut ancestor = cx.tree.get_layout_parent(entity);

    while let Some(current) = ancestor {
        if invalidated_subtrees.contains(&current) {
            return true;
        }

        ancestor = cx.tree.get_layout_parent(current);
    }

    false
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

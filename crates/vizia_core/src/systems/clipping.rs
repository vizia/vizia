use hashbrown::HashSet;
use skia_safe::{Path, PathBuilder, Point, RRect, Rect};
use vizia_storage::{LayoutChildIterator, LayoutParentIterator};

use crate::cache::CachedData;

use crate::{prelude::*, tree::minimal_layout_dirty_roots};

/// Computes the clipping path for each entity in the layout tree.
///
/// Callers only mark the entities whose own clip inputs changed. Starting from the topmost marked
/// entities, the system recomputes the clip path of each marked entity, and of every descendant
/// whose parent clip path changed.
pub(crate) fn clipping_system(cx: &mut Context) -> bool {
    let _scope = crate::profiling::scope("clipping_system");
    if cx.style.reclip.is_empty() {
        return false;
    }

    let reclip = std::mem::take(&mut cx.style.reclip);
    let hovered_ancestors: HashSet<Entity> =
        LayoutParentIterator::new(&cx.tree, cx.hovered).collect();
    let mut hover_affected = false;

    // Each entry holds an entity, whether the clip path of its parent changed, and whether its
    // clip path must be recomputed regardless.
    let mut stack: Vec<(Entity, bool, bool)> = minimal_layout_dirty_roots(&cx.tree, &reclip)
        .into_iter()
        .map(|root| (root, false, false))
        .collect();

    while let Some((entity, parent_changed, forced)) = stack.pop() {
        let marked = reclip.contains(&entity);
        let changed = (forced || parent_changed || marked) && update_clip_path(cx, entity);
        hover_affected |= changed && hovered_ancestors.contains(&entity);

        // Clip paths with a single hidden axis extend to the window bounds, so a change to a window
        // recomputes everything within it.
        let forced = forced || (marked && cx.tree.is_window(entity));

        stack.extend(
            LayoutChildIterator::new(&cx.tree, entity).map(|child| (child, changed, forced)),
        );
    }

    hover_affected
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

    if has_own_clip(&cx.style, entity) {
        cx.cache.own_clips.insert(entity);
    } else {
        cx.cache.own_clips.remove(&entity);
    }

    let clip_path = compute_clip_path(cx, entity, bounds);

    // Cached draw bounds are independent of the clip paths of ancestors, so they remain valid.
    if cx.cache.clip_path.get(entity) == Some(&clip_path) {
        return false;
    }

    match clip_path.as_ref() {
        Some(path) => {
            let bounds = path.bounds();
            cx.cache.clip_bounds.insert(
                entity,
                BoundingBox::from_min_max(
                    bounds.left().floor(),
                    bounds.top().floor(),
                    bounds.right().ceil(),
                    bounds.bottom().ceil(),
                ),
            );
        }
        None => {
            cx.cache.clip_bounds.remove(entity);
        }
    }
    cx.cache.clip_path.insert(entity, clip_path);
    true
}

/// Returns the bounds of the clip of an entity in its own coordinate space (i.e. excluding its
/// transform), ignoring the clips of its ancestors, or `None` if it does not clip its content.
pub(crate) fn own_clip_bounds(
    style: &Style,
    cache: &CachedData,
    tree: &Tree<Entity>,
    entity: Entity,
) -> Option<BoundingBox> {
    let bounds = cache.bounds.get(entity).copied()?;
    match (
        overflow_clip_bounds(style, cache, tree, entity, bounds),
        shape_clip_bounds(style, entity, bounds),
    ) {
        (Some(overflow), Some(shape)) => Some(overflow.intersection(&shape)),
        (overflow, shape) => overflow.or(shape),
    }
}

fn shape_clip_bounds(style: &Style, entity: Entity, bounds: BoundingBox) -> Option<BoundingBox> {
    let scale = style.scale_factor();
    match style.clip_path.get(entity) {
        Some(ClipPath::Shape(rect)) => Some(bounds.shrink_sides(
            rect.3.to_pixels(bounds.w, scale),
            rect.0.to_pixels(bounds.h, scale),
            rect.1.to_pixels(bounds.w, scale),
            rect.2.to_pixels(bounds.h, scale),
        )),
        _ => None,
    }
}

fn overflow_clip_bounds(
    style: &Style,
    cache: &CachedData,
    tree: &Tree<Entity>,
    entity: Entity,
    bounds: BoundingBox,
) -> Option<BoundingBox> {
    let overflowx = style.overflowx.get(entity).copied().unwrap_or_default();
    let overflowy = style.overflowy.get(entity).copied().unwrap_or_default();

    // A single hidden axis clips to the window bounds along the other axis.
    let window_bounds = || {
        let window_entity = if tree.is_window(entity) {
            entity
        } else {
            tree.get_parent_window(entity).unwrap_or(Entity::root())
        };
        cache.bounds.get(window_entity).copied().unwrap_or(bounds)
    };

    match (overflowx, overflowy) {
        (Overflow::Visible, Overflow::Visible) => None,
        (Overflow::Hidden, Overflow::Visible) => {
            let window_bounds = window_bounds();
            Some(BoundingBox::from_min_max(
                bounds.left(),
                window_bounds.top(),
                bounds.right(),
                window_bounds.bottom(),
            ))
        }
        (Overflow::Visible, Overflow::Hidden) => {
            let window_bounds = window_bounds();
            Some(BoundingBox::from_min_max(
                window_bounds.left(),
                bounds.top(),
                window_bounds.right(),
                bounds.bottom(),
            ))
        }
        (Overflow::Hidden, Overflow::Hidden) => Some(bounds),
    }
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

    let shape_clip_path = shape_clip_bounds(&cx.style, entity, bounds)
        .map(|clip_bounds| build_clip_path(cx, entity, clip_bounds));
    let overflow_clip_path = overflow_clip_bounds(&cx.style, &cx.cache, &cx.tree, entity, bounds)
        .map(|clip_bounds| build_clip_path(cx, entity, clip_bounds));

    let clip_path = match (overflow_clip_path, shape_clip_path) {
        (Some(overflow_path), Some(shape_path)) => {
            intersect_clip_paths(&overflow_path, &shape_path)
        }
        (Some(overflow_path), None) => Some(overflow_path),
        (None, Some(shape_path)) => Some(shape_path),
        (None, None) => None,
    };

    let transform = cx.cache.absolute_transform(&cx.tree, entity);
    let clip_path = clip_path.map(|clip_path| clip_path.make_transform(&transform));

    match (clip_path, parent_clip_path) {
        (Some(clip_path), Some(parent_clip_path)) => {
            intersect_clip_paths(&clip_path, &parent_clip_path)
        }
        (Some(clip_path), None) => Some(clip_path),
        (None, Some(parent_clip_path)) => Some(parent_clip_path),
        (None, None) => None,
    }
}

/// Intersects two clip paths.
///
/// Skia clips against a rect or a rounded rect analytically, but against any other path through a
/// mask. A path op always returns a general path, even when the intersection of two rounded rects
/// is itself a rounded rect, so the shape is composed directly when it can be represented
/// exactly, and the exact path op is used for everything else.
fn intersect_clip_paths(a: &Path, b: &Path) -> Option<Path> {
    if let (Some(a), Some(b)) = (circular_rrect(a), circular_rrect(b))
        && let Some(rrect) = intersect_rrects(&a, &b)
    {
        let mut path = PathBuilder::new();
        path.add_rrect(rrect, None, None);
        return Some(path.detach());
    }

    a.op(b, skia_safe::PathOp::Intersect)
}

/// Returns a clip path as a rect or rounded rect whose corners are circular. Corners follow Skia's
/// order: top left, top right, bottom right, bottom left.
fn circular_rrect(path: &Path) -> Option<RRect> {
    if path.is_inverse_fill_type() {
        return None;
    }

    let rrect = match path.is_rect() {
        Some((rect, _, _)) => RRect::new_rect(rect),
        None => path.is_rrect()?,
    };

    rrect.radii_ref().iter().all(|radius| radius.x == radius.y).then_some(rrect)
}

/// Returns the intersection of two rounded rects, if it is a rounded rect itself.
///
/// A rounded rect is its bounds without the area that each rounded corner cuts off. The
/// intersection is therefore the intersection of both bounds without what either shape cuts off,
/// and it is a rounded rect when each corner of those bounds is either square or cut by the
/// corner of a shape that sits right on it. Any other cut, such as the arc of one shape running
/// across the other, has no such representation.
fn intersect_rrects(a: &RRect, b: &RRect) -> Option<RRect> {
    let bounds = Rect::new(
        a.rect().left.max(b.rect().left),
        a.rect().top.max(b.rect().top),
        a.rect().right.min(b.rect().right),
        a.rect().bottom.min(b.rect().bottom),
    );
    if bounds.is_empty() {
        return None;
    }

    let mut radii = [Point::new(0.0, 0.0); 4];
    for (corner, radius) in radii.iter_mut().enumerate() {
        let point = corner_point(&bounds, corner);
        let a_radius = a.radii_ref()[corner].x;
        let b_radius = b.radii_ref()[corner].x;
        let a_on_corner = corner_point(a.rect(), corner) == point;
        let b_on_corner = corner_point(b.rect(), corner) == point;
        let radius_x = match (a_on_corner, b_on_corner) {
            // Both shapes have their corner here: the larger cut contains the smaller one.
            (true, true) => a_radius.max(b_radius),
            // Only one shape has its corner here: the other must not cut into what it keeps.
            (true, false) if corner_arc_inside(a, b, corner) => a_radius,
            (false, true) if corner_arc_inside(b, a, corner) => b_radius,
            // Neither does: the corner is square, provided both shapes contain it.
            (false, false) if rrect_contains(a, point) && rrect_contains(b, point) => 0.0,
            _ => return None,
        };
        *radius = Point::new(radius_x, radius_x);
    }

    let rrect = RRect::new_rect_radii(bounds, &radii);
    // Skia scales radii that overlap along an edge, which changes the region.
    (rrect.radii_ref() == &radii).then_some(rrect)
}

/// Returns the corner of a rect with the given index.
fn corner_point(rect: &Rect, corner: usize) -> Point {
    Point::new(
        if matches!(corner, 0 | 3) { rect.left } else { rect.right },
        if corner < 2 { rect.top } else { rect.bottom },
    )
}

/// Returns the direction in which a corner's edges lead into its rect, along each axis.
fn inward(corner: usize) -> Point {
    Point::new(
        if matches!(corner, 0 | 3) { 1.0 } else { -1.0 },
        if corner < 2 { 1.0 } else { -1.0 },
    )
}

/// Returns the square that a rounded corner of the given radius is cut from.
fn corner_square(rect: &Rect, radius: f32, corner: usize) -> Rect {
    let point = corner_point(rect, corner);
    let inward = inward(corner);
    Rect::new(point.x, point.y, point.x + inward.x * radius, point.y + inward.y * radius).sorted()
}

/// Returns the centre of the arc of a rounded corner of the given radius.
fn arc_centre(rect: &Rect, radius: f32, corner: usize) -> (f64, f64) {
    let point = corner_point(rect, corner);
    let inward = inward(corner);
    (
        point.x as f64 + inward.x as f64 * radius as f64,
        point.y as f64 + inward.y as f64 * radius as f64,
    )
}

/// Returns true if the rounded rect contains the point, boundary included.
fn rrect_contains(rrect: &RRect, point: Point) -> bool {
    let bounds = rrect.rect();
    if point.x < bounds.left
        || point.x > bounds.right
        || point.y < bounds.top
        || point.y > bounds.bottom
    {
        return false;
    }

    (0..4).all(|corner| {
        let radius = rrect.radii_ref()[corner].x;
        let square = corner_square(bounds, radius, corner);
        if radius == 0.0
            || point.x < square.left
            || point.x > square.right
            || point.y < square.top
            || point.y > square.bottom
        {
            return true;
        }

        let (x, y) = arc_centre(bounds, radius, corner);
        let (dx, dy) = (point.x as f64 - x, point.y as f64 - y);
        dx * dx + dy * dy <= radius as f64 * radius as f64
    })
}

/// Returns true if `outer` does not cut into what the given corner of `inner` keeps, where that
/// corner sits on a corner of the intersection. That holds if the corner's square meets no rounded
/// corner of `outer`, or meets only the same corner of `outer` and its arc lies inside that
/// corner's arc.
fn corner_arc_inside(inner: &RRect, outer: &RRect, corner: usize) -> bool {
    let radius = inner.radii_ref()[corner].x;
    let square = corner_square(inner.rect(), radius, corner);

    (0..4).all(|other| {
        let other_radius = outer.radii_ref()[other].x;
        if other_radius == 0.0 {
            return true;
        }

        let other_square = corner_square(outer.rect(), other_radius, other);
        let disjoint = square.right < other_square.left
            || square.left > other_square.right
            || square.bottom < other_square.top
            || square.top > other_square.bottom;
        if disjoint {
            return true;
        }
        if other != corner {
            return false;
        }

        let (x, y) = arc_centre(inner.rect(), radius, corner);
        let (other_x, other_y) = arc_centre(outer.rect(), other_radius, corner);
        let nested = (x - other_x).hypot(y - other_y) + radius as f64 <= other_radius as f64;

        // Arcs of equal radius are nested as well when the inner one is moved into the outer shape.
        let point = corner_point(inner.rect(), corner);
        let other_point = corner_point(outer.rect(), corner);
        let inward = inward(corner);
        let moved_inward = (point.x - other_point.x) * inward.x >= 0.0
            && (point.y - other_point.y) * inward.y >= 0.0;

        nested || (radius == other_radius && moved_inward)
    })
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
    use super::{clipping_system, intersect_clip_paths};
    use crate::prelude::*;
    use skia_safe::{Matrix, Path, PathBuilder, PathFillType, PathOp, Point, RRect, Rect};

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
        let (mut cx, a, _, c) = setup();
        cx.cache.set_bounds(a, BoundingBox { x: 10.0, y: 10.0, w: 50.0, h: 50.0 });
        cx.style.needs_reclip(a);
        clipping_system(&mut cx);

        assert_eq!(clip_bounds(&cx, c), Some(Rect::new(10.0, 10.0, 60.0, 60.0)));
        assert!(cx.style.reclip.is_empty());
    }

    #[test]
    fn entities_with_own_clips_are_tracked() {
        let (mut cx, a, b, _) = setup();
        assert!(cx.cache.own_clips.contains(&a));
        assert!(!cx.cache.own_clips.contains(&b));

        cx.style.overflowx.insert(a, Overflow::Visible);
        cx.style.overflowy.insert(a, Overflow::Visible);
        cx.style.needs_reclip(a);
        clipping_system(&mut cx);
        assert!(!cx.cache.own_clips.contains(&a));
    }

    #[test]
    fn clip_paths_include_ancestor_transforms() {
        let (mut cx, a, b, _) = setup();
        cx.style.overflowx.insert(b, Overflow::Hidden);
        cx.style.overflowy.insert(b, Overflow::Hidden);
        cx.cache.set_bounds(b, BoundingBox { x: 20.0, y: 20.0, w: 20.0, h: 20.0 });
        cx.cache.local_transform.insert(a, skia_safe::Matrix::translate((5.0, 0.0)));
        cx.style.needs_reclip(b);
        clipping_system(&mut cx);

        assert_eq!(clip_bounds(&cx, b), Some(Rect::new(25.0, 20.0, 45.0, 40.0)));
    }

    #[test]
    fn hovered_entity_ancestor_clip_requests_refresh() {
        let (mut cx, ancestor, _, hovered) = setup();
        cx.hovered = hovered;
        cx.cache.set_bounds(ancestor, BoundingBox { x: 10.0, y: 10.0, w: 50.0, h: 50.0 });
        cx.style.needs_reclip(ancestor);

        assert!(clipping_system(&mut cx));
    }

    #[test]
    fn unrelated_descendant_clip_does_not_request_refresh() {
        let (mut cx, hovered, descendant, _) = setup();
        cx.hovered = hovered;
        cx.style.overflowx.insert(descendant, Overflow::Hidden);
        cx.style.overflowy.insert(descendant, Overflow::Hidden);
        cx.style.needs_reclip(descendant);

        assert!(!clipping_system(&mut cx));
    }

    /// Adds an element that clips its content to a rounded rectangle.
    fn rounded(cx: &mut Context, parent: Entity, bounds: BoundingBox, radius: f32) -> Entity {
        let entity = element(cx, parent, bounds);
        cx.style.overflowx.insert(entity, Overflow::Hidden);
        cx.style.overflowy.insert(entity, Overflow::Hidden);
        for corner in [
            &mut cx.style.corner_top_left_radius,
            &mut cx.style.corner_top_right_radius,
            &mut cx.style.corner_bottom_right_radius,
            &mut cx.style.corner_bottom_left_radius,
        ] {
            corner.insert(entity, LengthOrPercentage::px(radius));
        }
        entity
    }

    fn reclip(cx: &mut Context, entities: &[Entity]) {
        for entity in entities {
            cx.style.needs_reclip(*entity);
        }
        clipping_system(cx);
    }

    fn clip(cx: &Context, entity: Entity) -> Path {
        cx.cache.clip_path.get(entity).cloned().flatten().expect("effective clip exists")
    }

    fn rect_path(rect: Rect) -> Path {
        let mut path = PathBuilder::new();
        path.add_rect(rect, None, None);
        path.detach()
    }

    fn rrect(rect: Rect, radii: [f32; 4]) -> RRect {
        RRect::new_rect_radii(rect, &radii.map(|radius| Point::new(radius, radius)))
    }

    fn rrect_path(rect: Rect, radii: [f32; 4]) -> Path {
        let mut path = PathBuilder::new();
        path.add_rrect(rrect(rect, radii), None, None);
        path.detach()
    }

    struct Xorshift(u64);

    impl Xorshift {
        fn below(&mut self, count: u64) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            (self.0 >> 11) % count
        }
    }

    fn is_shape(path: &Path) -> bool {
        path.is_rect().is_some() || path.is_rrect().is_some()
    }

    #[test]
    fn rounded_clips_stay_rounded_rects() {
        let mut cx = Context::default();
        cx.cache.set_bounds(Entity::root(), BoundingBox { x: 0.0, y: 0.0, w: 1000.0, h: 1000.0 });
        let panel = rounded(
            &mut cx,
            Entity::root(),
            BoundingBox { x: 20.0, y: 20.0, w: 120.0, h: 120.0 },
            24.0,
        );
        let well =
            rounded(&mut cx, panel, BoundingBox { x: 30.0, y: 30.0, w: 100.0, h: 100.0 }, 24.0);
        cx.style.needs_reclip(Entity::root());
        clipping_system(&mut cx);

        assert_eq!(
            clip(&cx, panel).is_rrect(),
            Some(RRect::new_rect_xy(Rect::new(20.0, 20.0, 140.0, 140.0), 24.0, 24.0))
        );
        assert_eq!(
            clip(&cx, well).is_rrect(),
            Some(RRect::new_rect_xy(Rect::new(30.0, 30.0, 130.0, 130.0), 24.0, 24.0))
        );
    }

    #[test]
    fn rounded_clips_keep_their_shape_through_scale_and_translation() {
        for scale in [1.0_f32, 2.0] {
            for transformed in [false, true] {
                let mut cx = Context::default();
                cx.style.dpi_factor = scale as f64;
                cx.cache.set_bounds(
                    Entity::root(),
                    BoundingBox { x: 0.0, y: 0.0, w: 4000.0, h: 4000.0 },
                );
                let panel = rounded(
                    &mut cx,
                    Entity::root(),
                    BoundingBox { x: 0.0, y: 0.0, w: 1240.0 * scale, h: 700.0 * scale },
                    8.0,
                );
                // A clip path shape with no inset still reaches the panel's own bounds.
                cx.style.clip_path.insert(panel, LengthOrPercentage::px(0.0).into());
                let content = element(
                    &mut cx,
                    panel,
                    BoundingBox { x: 0.0, y: 0.0, w: 100.0 * scale, h: 100.0 * scale },
                );
                let well = rounded(
                    &mut cx,
                    content,
                    BoundingBox {
                        x: 12.0 * scale,
                        y: 12.0 * scale,
                        w: 60.0 * scale,
                        h: 60.0 * scale,
                    },
                    4.0,
                );
                let (offset, factor) = if transformed { (10.0 * scale, 0.5) } else { (0.0, 1.0) };
                cx.cache
                    .local_transform
                    .insert(panel, Matrix::scale_translate((factor, factor), (offset, offset)));
                reclip(&mut cx, &[Entity::root(), panel, content, well]);

                let panel_clip = rrect(
                    Rect::from_xywh(
                        offset,
                        offset,
                        1240.0 * scale * factor,
                        700.0 * scale * factor,
                    ),
                    [8.0 * scale * factor; 4],
                );
                assert_eq!(clip(&cx, panel).is_rrect(), Some(panel_clip));
                assert_eq!(clip(&cx, content).is_rrect(), Some(panel_clip));
                assert_eq!(
                    clip(&cx, well).is_rrect(),
                    Some(rrect(
                        Rect::from_xywh(
                            offset + 12.0 * scale * factor,
                            offset + 12.0 * scale * factor,
                            60.0 * scale * factor,
                            60.0 * scale * factor,
                        ),
                        [4.0 * scale * factor; 4],
                    ))
                );
            }
        }
    }

    #[test]
    fn rounded_well_inside_rounded_panel_stays_rounded() {
        let mut cx = Context::default();
        cx.cache.set_bounds(Entity::root(), BoundingBox { x: 0.0, y: 0.0, w: 1000.0, h: 1000.0 });
        let panel = rounded(
            &mut cx,
            Entity::root(),
            BoundingBox { x: 20.0, y: 20.0, w: 100.0, h: 80.0 },
            6.0,
        );
        // Wells inset by one pixel, with a radius smaller than and equal to the panel's.
        let wells = [4.0, 6.0].map(|radius| {
            rounded(&mut cx, panel, BoundingBox { x: 21.0, y: 21.0, w: 98.0, h: 78.0 }, radius)
        });
        // A child larger than the panel inherits the panel's shape.
        let larger =
            rounded(&mut cx, panel, BoundingBox { x: 10.0, y: 10.0, w: 120.0, h: 100.0 }, 4.0);
        cx.style.needs_reclip(Entity::root());
        clipping_system(&mut cx);

        for (well, radius) in wells.into_iter().zip([4.0, 6.0]) {
            assert_eq!(
                clip(&cx, well).is_rrect(),
                Some(rrect(Rect::new(21.0, 21.0, 119.0, 99.0), [radius; 4]))
            );
        }
        assert_eq!(
            clip(&cx, larger).is_rrect(),
            Some(rrect(Rect::new(20.0, 20.0, 120.0, 100.0), [6.0; 4]))
        );
    }

    #[test]
    fn square_band_across_rounded_panel_keeps_the_panel_corners_it_covers() {
        let mut cx = Context::default();
        cx.cache.set_bounds(Entity::root(), BoundingBox { x: 0.0, y: 0.0, w: 1000.0, h: 1000.0 });
        let panel = rounded(
            &mut cx,
            Entity::root(),
            BoundingBox { x: 0.0, y: 0.0, w: 100.0, h: 80.0 },
            8.0,
        );
        let top_band =
            rounded(&mut cx, panel, BoundingBox { x: 0.0, y: 0.0, w: 100.0, h: 40.0 }, 0.0);
        let cross_band =
            rounded(&mut cx, panel, BoundingBox { x: 10.0, y: -10.0, w: 80.0, h: 100.0 }, 0.0);
        cx.style.needs_reclip(Entity::root());
        clipping_system(&mut cx);

        assert_eq!(
            clip(&cx, top_band).is_rrect(),
            Some(rrect(Rect::new(0.0, 0.0, 100.0, 40.0), [8.0, 8.0, 0.0, 0.0]))
        );
        assert_eq!(
            clip(&cx, cross_band).is_rect().map(|(rect, _, _)| rect),
            Some(Rect::new(10.0, 0.0, 90.0, 80.0))
        );
    }

    #[test]
    fn crossing_rounded_bands_compose_per_corner() {
        // Each band's rounded corners that lie inside the other band survive; the corners where
        // the bands cross are square.
        let tall = rrect_path(Rect::new(20.0, 0.0, 100.0, 100.0), [10.0; 4]);
        let wide = rrect_path(Rect::new(0.0, 20.0, 200.0, 200.0), [10.0; 4]);
        let composed = intersect_clip_paths(&tall, &wide).unwrap();

        assert_eq!(
            composed.is_rrect(),
            Some(rrect(Rect::new(20.0, 20.0, 100.0, 100.0), [0.0, 0.0, 10.0, 10.0]))
        );
    }

    #[test]
    fn nonuniform_scale_keeps_the_exact_region_of_a_rounded_clip() {
        let mut cx = Context::default();
        cx.cache.set_bounds(Entity::root(), BoundingBox { x: 0.0, y: 0.0, w: 1000.0, h: 1000.0 });
        let panel = rounded(
            &mut cx,
            Entity::root(),
            BoundingBox { x: 0.0, y: 0.0, w: 100.0, h: 80.0 },
            8.0,
        );
        // Corners become elliptical, which a rounded rectangle of circular corners cannot hold.
        cx.cache.local_transform.insert(panel, Matrix::scale_translate((0.5, 0.25), (0.0, 0.0)));
        reclip(&mut cx, &[Entity::root(), panel]);

        let actual = clip(&cx, panel);
        assert_eq!(*actual.bounds(), Rect::new(0.0, 0.0, 50.0, 20.0));
        assert!(actual.contains((1.0, 1.0)));
        assert!(actual.contains((25.0, 10.0)));
        assert!(!actual.contains((0.5, 0.5)));
        assert!(!actual.contains((51.0, 10.0)));
    }

    #[test]
    fn overlaps_without_a_rounded_rect_intersection_are_intersected_exactly() {
        let panel = rrect_path(Rect::new(0.0, 0.0, 100.0, 80.0), [6.0; 4]);
        let rotated = rect_path(Rect::new(0.0, 0.0, 100.0, 80.0))
            .make_transform(&Matrix::rotate_deg_pivot(45.0, (50.0, 40.0)));
        // A square clip inset by a pixel meets the panel's corners on a curve, a thin band
        // crosses the panel's corner arcs, a band with rounded lower corners is cut by the panel's
        // upper corners, and two panels round adjacent corners so far that their radii would be
        // scaled.
        let inset = rect_path(Rect::new(1.0, 1.0, 99.0, 79.0));
        let thin_band = rect_path(Rect::new(0.0, 0.0, 100.0, 2.0));
        let rounded_band = rrect_path(Rect::new(0.0, 0.0, 100.0, 8.0), [0.0, 0.0, 6.0, 6.0]);
        let left = rrect_path(Rect::new(0.0, 0.0, 100.0, 100.0), [80.0, 0.0, 0.0, 0.0]);
        let right = rrect_path(Rect::new(0.0, 0.0, 100.0, 100.0), [0.0, 80.0, 0.0, 0.0]);

        for (a, b, inside, outside) in [
            (&panel, &rotated, (50.0, 40.0), (50.0, -1.0)),
            (&panel, &inset, (50.0, 40.0), (1.1, 1.1)),
            (&panel, &thin_band, (50.0, 1.0), (0.5, 0.5)),
            (&panel, &rounded_band, (50.0, 4.0), (0.5, 0.5)),
            (&left, &right, (50.0, 50.0), (10.0, 10.0)),
        ] {
            let exact = a.op(b, PathOp::Intersect).unwrap();
            let actual = intersect_clip_paths(a, b).unwrap();
            assert!(!is_shape(&actual));
            assert_eq!(actual.bounds(), exact.bounds());
            assert!(actual.contains(inside));
            assert!(!actual.contains(outside));
        }
    }

    #[test]
    fn disjoint_clips_intersect_to_nothing() {
        let left = rrect_path(Rect::new(0.0, 0.0, 10.0, 10.0), [4.0; 4]);
        let right = rrect_path(Rect::new(20.0, 0.0, 30.0, 10.0), [4.0; 4]);
        let touching = rrect_path(Rect::new(10.0, 0.0, 20.0, 10.0), [4.0; 4]);

        for other in [&right, &touching] {
            let composed = intersect_clip_paths(&left, other).unwrap();
            assert!(composed.is_empty(), "{composed:?}");
        }
    }

    #[test]
    fn inverse_filled_clips_are_intersected_exactly() {
        let mut hole = PathBuilder::new_with_fill_type(PathFillType::InverseWinding);
        hole.add_rect(Rect::new(0.0, 0.0, 50.0, 50.0), None, None);
        let hole = hole.detach();
        let square = rect_path(Rect::new(0.0, 0.0, 100.0, 100.0));
        let composed = intersect_clip_paths(&hole, &square).unwrap();

        assert!(!composed.contains((25.0, 25.0)));
        assert!(composed.contains((75.0, 75.0)));
    }

    #[test]
    fn composed_clips_match_exact_intersections() {
        // Edges on a coarse grid make shared corners and shared edges common.
        let mut random = Xorshift(0x2545_f491_4f6c_dd1d);
        let mut random_clip = || {
            let left = random.below(6) as f32 * 10.0;
            let top = random.below(6) as f32 * 10.0;
            let right = left + 20.0 + random.below(6) as f32 * 10.0;
            let bottom = top + 20.0 + random.below(6) as f32 * 10.0;
            let radii = [(); 4].map(|_| [0.0, 5.0, 10.0, 20.0][random.below(4) as usize]);
            rrect_path(Rect::new(left, top, right, bottom), radii)
        };

        let mut composed_shapes = 0;
        let mut exact_fallbacks = 0;
        for _ in 0..400 {
            let a = random_clip();
            let b = random_clip();
            let exact = a.op(&b, PathOp::Intersect).unwrap();
            let composed = intersect_clip_paths(&a, &b).unwrap();
            if is_shape(&composed) {
                composed_shapes += 1;
            } else {
                exact_fallbacks += 1;
            }
            for i in 0..48 {
                for j in 0..48 {
                    let point = (i as f32 * 2.5 - 4.63, j as f32 * 2.5 - 4.63);
                    assert_eq!(
                        composed.contains(point),
                        exact.contains(point),
                        "{point:?} of {a:?} and {b:?}"
                    );
                }
            }
        }
        assert!(composed_shapes > 40, "{composed_shapes} compositions kept their shape");
        assert!(exact_fallbacks > 40, "{exact_fallbacks} compositions needed a path op");
    }
}

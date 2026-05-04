use std::f32::consts::SQRT_2;

use skia_safe::{
    PathBuilder, PathDirection, Point, RRect, Rect, path_builder::ArcSize, rrect::Corner,
};
use vizia_storage::LayoutTreeIterator;

use crate::prelude::*;

/// Computes the clipping path for each entity in the layout tree.
pub(crate) fn clipping_system(cx: &mut Context) {
    if cx.style.reclip.is_empty() {
        return;
    }

    let iter = LayoutTreeIterator::full(&cx.tree);

    for entity in iter {
        if !cx.style.reclip.contains(&entity) {
            continue;
        }

        let bounds = cx.cache.bounds.get(entity).copied().unwrap();
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

            if previous_clip_bounds != new_clip_bounds {
                for descendant in LayoutTreeIterator::subtree(&cx.tree, entity).skip(1) {
                    cx.cache.draw_bounds.remove(descendant);
                }
            }
            continue;
        }

        let parent = cx.tree.get_layout_parent(entity).unwrap_or(Entity::root());

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

        if previous_clip_bounds != new_clip_bounds {
            for descendant in LayoutTreeIterator::subtree(&cx.tree, entity).skip(1) {
                cx.cache.draw_bounds.remove(descendant);
            }
        }
    }

    cx.style.reclip.clear();
}

fn build_clip_path(cx: &Context, entity: Entity, clip_bounds: BoundingBox) -> skia_safe::Path {
    let outset = (0.0, 0.0);
    let bounds = cx.cache.bounds.get(entity).copied().unwrap_or_default();
    let scale = cx.style.scale_factor();
    let corner_top_left_radius = cx
        .style
        .corner_top_left_radius
        .get(entity)
        .map(|l| l.to_pixels(bounds.w.min(bounds.h), scale))
        .unwrap_or_default();
    let corner_top_right_radius = cx
        .style
        .corner_top_right_radius
        .get(entity)
        .map(|l| l.to_pixels(bounds.w.min(bounds.h), scale))
        .unwrap_or_default();
    let corner_bottom_right_radius = cx
        .style
        .corner_bottom_right_radius
        .get(entity)
        .map(|l| l.to_pixels(bounds.w.min(bounds.h), scale))
        .unwrap_or_default();
    let corner_bottom_left_radius = cx
        .style
        .corner_bottom_left_radius
        .get(entity)
        .map(|l| l.to_pixels(bounds.w.min(bounds.h), scale))
        .unwrap_or_default();

    let corner_top_left_shape =
        cx.style.corner_top_left_shape.get(entity).copied().unwrap_or_default();
    let corner_top_right_shape =
        cx.style.corner_top_right_shape.get(entity).copied().unwrap_or_default();
    let corner_bottom_right_shape =
        cx.style.corner_bottom_right_shape.get(entity).copied().unwrap_or_default();
    let corner_bottom_left_shape =
        cx.style.corner_bottom_left_shape.get(entity).copied().unwrap_or_default();

    let corner_top_left_smoothing =
        cx.style.corner_top_left_smoothing.get(entity).copied().unwrap_or(0.0);
    let corner_top_right_smoothing =
        cx.style.corner_top_right_smoothing.get(entity).copied().unwrap_or(0.0);
    let corner_bottom_right_smoothing =
        cx.style.corner_bottom_right_smoothing.get(entity).copied().unwrap_or(0.0);
    let corner_bottom_left_smoothing =
        cx.style.corner_bottom_left_smoothing.get(entity).copied().unwrap_or(0.0);

    let bounds = clip_bounds;

    let bounds = BoundingBox::from_min_max(0.0, 0.0, bounds.w, bounds.h);

    let rect: Rect = bounds.into();

    let mut rr = RRect::new_rect_radii(
        rect,
        &[
            Point::new(corner_top_left_radius, corner_top_left_radius),
            Point::new(corner_top_right_radius, corner_top_right_radius),
            Point::new(corner_bottom_right_radius, corner_bottom_right_radius),
            Point::new(corner_bottom_left_radius, corner_bottom_left_radius),
        ],
    );

    rr = rr.with_outset(outset);

    let x = rr.bounds().x();
    let y = rr.bounds().y();
    let width = rr.width();
    let height = rr.height();

    //TODO: Cache the path and regenerate if the bounds change
    let mut path = PathBuilder::new();

    if width == height
        && corner_bottom_left_radius == width / 2.0
        && corner_bottom_right_radius == width / 2.0
        && corner_top_left_radius == height / 2.0
        && corner_top_right_radius == height / 2.0
    {
        path.add_circle((width / 2.0, bounds.h / 2.0), width / 2.0, PathDirection::CW);
    } else if corner_top_left_radius == corner_top_right_radius
        && corner_top_right_radius == corner_bottom_right_radius
        && corner_bottom_right_radius == corner_bottom_left_radius
        && corner_top_left_smoothing == 0.0
        && corner_top_left_smoothing == corner_top_right_smoothing
        && corner_top_right_smoothing == corner_bottom_right_smoothing
        && corner_bottom_right_smoothing == corner_bottom_left_smoothing
        && corner_top_left_shape == CornerShape::Round
        && corner_top_left_shape == corner_top_right_shape
        && corner_top_right_shape == corner_bottom_right_shape
        && corner_bottom_right_shape == corner_bottom_left_shape
    {
        path.add_rrect(rr, None, None);
    } else {
        let top_right = rr.radii(Corner::UpperRight).x;

        if top_right > 0.0 {
            let (a, b, c, d, l, p, radius) = compute_smooth_corner(
                top_right,
                corner_top_right_smoothing,
                bounds.width(),
                bounds.height(),
            );

            path.move_to((f32::max(width / 2.0, width - p), 0.0));
            if corner_top_right_shape == CornerShape::Round {
                path.cubic_to(
                    (width - (p - a), 0.0),
                    (width - (p - a - b), 0.0),
                    (width - (p - a - b - c), d),
                )
                .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (l, l))
                .cubic_to(
                    (width, p - a - b),
                    (width, p - a),
                    (width, f32::min(height / 2.0, p)),
                );
            } else {
                path.line_to((width, f32::min(height / 2.0, p)));
            }
        } else {
            path.move_to((width / 2.0, 0.0)).line_to((width, 0.0)).line_to((width, height / 2.0));
        }

        let bottom_right = rr.radii(Corner::LowerRight).x;
        if bottom_right > 0.0 {
            let (a, b, c, d, l, p, radius) =
                compute_smooth_corner(bottom_right, corner_bottom_right_smoothing, width, height);

            path.line_to((width, f32::max(height / 2.0, height - p)));
            if corner_bottom_right_shape == CornerShape::Round {
                path.cubic_to(
                    (width, height - (p - a)),
                    (width, height - (p - a - b)),
                    (width - d, height - (p - a - b - c)),
                )
                .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (-l, l))
                .cubic_to(
                    (width - (p - a - b), height),
                    (width - (p - a), height),
                    (f32::max(width / 2.0, width - p), height),
                );
            } else {
                path.line_to((f32::max(width / 2.0, width - p), height));
            }
        } else {
            path.line_to((width, height)).line_to((width / 2.0, height));
        }

        let bottom_left = rr.radii(Corner::LowerLeft).x;
        if bottom_left > 0.0 {
            let (a, b, c, d, l, p, radius) =
                compute_smooth_corner(bottom_left, corner_bottom_left_smoothing, width, height);

            path.line_to((f32::min(width / 2.0, p), height));
            if corner_bottom_left_shape == CornerShape::Round {
                path.cubic_to((p - a, height), (p - a - b, height), (p - a - b - c, height - d))
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (-l, -l))
                    .cubic_to(
                        (0.0, height - (p - a - b)),
                        (0.0, height - (p - a)),
                        (0.0, f32::max(height / 2.0, height - p)),
                    );
            } else {
                path.line_to((0.0, f32::max(height / 2.0, height - p)));
            }
        } else {
            path.line_to((0.0, height)).line_to((0.0, height / 2.0));
        }

        let top_left = rr.radii(Corner::UpperLeft).x;
        if top_left > 0.0 {
            let (a, b, c, d, l, p, radius) =
                compute_smooth_corner(top_left, corner_top_left_smoothing, width, height);

            path.line_to((0.0, f32::min(height / 2.0, p)));
            if corner_top_left_shape == CornerShape::Round {
                path.cubic_to((0.0, p - a), (0.0, p - a - b), (d, p - a - b - c))
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (l, -l))
                    .cubic_to((p - a - b, 0.0), (p - a, 0.0), (f32::min(width / 2.0, p), 0.0));
            } else {
                path.line_to((f32::min(width / 2.0, p), 0.0));
            }
        } else {
            path.line_to((0.0, 0.0));
        }

        path.close();

        path.offset((x, y));
    }

    path.offset(clip_bounds.top_left());

    path.detach()
}

// Helper function for computing a rounded corner with variable smoothing
fn compute_smooth_corner(
    corner_radius: f32,
    smoothing: f32,
    width: f32,
    height: f32,
) -> (f32, f32, f32, f32, f32, f32, f32) {
    let max_p = f32::min(width, height) / 2.0;
    let corner_radius = f32::min(corner_radius, max_p);

    let p = f32::min((1.0 + smoothing) * corner_radius, max_p);

    let angle_alpha: f32;
    let angle_beta: f32;

    if corner_radius <= max_p / 2.0 {
        angle_alpha = 45.0 * smoothing;
        angle_beta = 90.0 * (1.0 - smoothing);
    } else {
        let diff_ratio = (corner_radius - max_p / 2.0) / (max_p / 2.0);

        angle_alpha = 45.0 * smoothing * (1.0 - diff_ratio);
        angle_beta = 90.0 * (1.0 - smoothing * (1.0 - diff_ratio));
    }

    let angle_theta = (90.0 - angle_beta) / 2.0;
    let dist_p3_p4 = corner_radius * (angle_theta / 2.0).to_radians().tan();

    let l = (angle_beta / 2.0).to_radians().sin() * corner_radius * SQRT_2;
    let c = dist_p3_p4 * angle_alpha.to_radians().cos();
    let d = c * angle_alpha.to_radians().tan();
    let b = (p - l - c - d) / 3.0;
    let a = 2.0 * b;

    (a, b, c, d, l, p, corner_radius)
}

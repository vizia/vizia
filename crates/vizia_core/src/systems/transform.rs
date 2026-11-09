use vizia_storage::LayoutChildIterator;

use crate::{
    animation::Interpolator, prelude::*, systems::clipping::has_own_clip,
    tree::minimal_layout_dirty_roots,
};

/// Applies transforms to the layout tree.
///
/// Callers only mark the entities whose own transform inputs changed. Starting from the topmost
/// marked entities, the system recomputes the transform of each marked entity, and of every
/// descendant whose parent transform changed.
pub(crate) fn transform_system(cx: &mut Context) {
    let _scope = crate::profiling::scope("transform_system");
    if cx.style.retransform.is_empty() {
        return;
    }

    let retransform = std::mem::take(&mut cx.style.retransform);

    // Each entry holds an entity and whether the transform of its parent changed.
    let mut stack: Vec<(Entity, bool)> = minimal_layout_dirty_roots(&cx.tree, &retransform)
        .into_iter()
        .map(|root| (root, false))
        .collect();

    while let Some((entity, parent_changed)) = stack.pop() {
        let changed =
            (parent_changed || retransform.contains(&entity)) && update_transform(cx, entity);

        if changed {
            // Entities without a clip of their own inherit the clip path of their parent, which
            // the clipping system propagates, so only entities which clip are marked.
            if has_own_clip(&cx.style, entity) {
                cx.style.needs_reclip(entity);
            }

            // The previous draw bounds of the topmost changed entity are kept so the draw system
            // can repaint the area it moved away from.
            if parent_changed {
                cx.cache.invalidate_draw_bounds(&cx.tree, entity);
            }
        }

        stack.extend(LayoutChildIterator::new(&cx.tree, entity).map(|child| (child, changed)));
    }
}

/// Recomputes the transform of an entity from the transform of its parent. Returns true if the
/// transform changed.
fn update_transform(cx: &mut Context, entity: Entity) -> bool {
    let Some(parent) = cx.tree.get_layout_parent(entity) else { return false };
    let parent_transform = cx.cache.transform.get(parent).copied().unwrap();
    let local_transform = local_transform(cx, entity);
    let Some(tx) = cx.cache.transform.get_mut(entity) else { return false };

    let new_transform = match local_transform {
        Some(transform) => parent_transform * transform,
        None => parent_transform,
    };

    let changed = *tx != new_transform;
    *tx = new_transform;
    changed
}

/// Returns the transform of an entity relative to its parent, or `None` if the entity has no
/// transform properties.
fn local_transform(cx: &Context, entity: Entity) -> Option<skia_safe::Matrix> {
    let style = &cx.style;
    if style.transform_origin.get(entity).is_none()
        && style.translate.get(entity).is_none()
        && style.rotate.get(entity).is_none()
        && style.scale.get(entity).is_none()
        && style.transform.get(entity).is_none()
    {
        return None;
    }

    let bounds = cx.cache.bounds.get(entity).copied().unwrap();
    let scale_factor = style.scale_factor();

    // Apply transform origin.
    let mut origin = style
        .transform_origin
        .get(entity)
        .map(|transform_origin| {
            let mut origin = skia_safe::Matrix::translate(bounds.top_left());
            let offset = transform_origin.as_transform(bounds, scale_factor);
            origin = offset * origin;
            origin
        })
        .unwrap_or(skia_safe::Matrix::translate(bounds.center()));
    // transform = origin * transform;
    let mut transform = origin;
    origin = origin.invert().unwrap();

    // Apply translation.
    if let Some(translate) = style.translate.get(entity) {
        transform = transform * translate.as_transform(bounds, scale_factor);
    }

    // Apply rotation.
    if let Some(rotate) = style.rotate.get(entity) {
        transform = transform * rotate.as_transform(bounds, scale_factor);
    }

    // Apply scaling.
    if let Some(scale) = style.scale.get(entity) {
        transform = transform * scale.as_transform(bounds, scale_factor);
    }

    // Apply transform functions.
    if let Some(transforms) = style.transform.get(entity) {
        // Check if the transform is currently animating
        // Get the animation state
        // Manually interpolate the value to get the overall transform for the current frame
        if let Some(animation_state) = style.transform.get_active_animation(entity) {
            if let Some(start) = animation_state.keyframes.first() {
                if let Some(end) = animation_state.keyframes.last() {
                    let start_transform = start.value.as_transform(bounds, scale_factor);
                    let end_transform = end.value.as_transform(bounds, scale_factor);
                    let t = animation_state.t;
                    let animated_transform =
                        skia_safe::Matrix::interpolate(&start_transform, &end_transform, t);
                    transform = transform * animated_transform;
                }
            }
        } else {
            transform = transform * transforms.as_transform(bounds, scale_factor);
        }
    }

    Some(transform * origin)
}
#[cfg(test)]
mod tests {
    use super::transform_system;
    use crate::prelude::*;
    use skia_safe::Matrix;

    fn element(cx: &mut Context, parent: Entity) -> Entity {
        let entity = cx.with_current(parent, |cx| Element::new(cx).entity());
        cx.cache.set_bounds(entity, BoundingBox { x: 10.0, y: 10.0, w: 100.0, h: 100.0 });
        entity
    }

    fn translate(cx: &mut Context, entity: Entity, x: f32, y: f32) {
        cx.style.translate.insert(
            entity,
            Translate {
                x: LengthOrPercentage::Length(Length::px(x)),
                y: LengthOrPercentage::Length(Length::px(y)),
            },
        );
        cx.needs_retransform(entity);
    }

    fn transform(cx: &Context, entity: Entity) -> Matrix {
        cx.cache.transform.get(entity).copied().unwrap()
    }

    /// Builds root > a > b > c and runs the transform system once.
    fn setup() -> (Context, Entity, Entity, Entity) {
        let mut cx = Context::default();
        let a = element(&mut cx, Entity::root());
        let b = element(&mut cx, a);
        let c = element(&mut cx, b);
        transform_system(&mut cx);
        cx.style.reclip.clear();
        (cx, a, b, c)
    }

    fn clip(cx: &mut Context, entity: Entity) {
        cx.style.overflowx.insert(entity, Overflow::Hidden);
        cx.style.overflowy.insert(entity, Overflow::Hidden);
    }

    #[test]
    fn transform_propagates_to_unmarked_descendants() {
        let (mut cx, a, b, c) = setup();
        clip(&mut cx, a);
        translate(&mut cx, a, 10.0, 20.0);
        transform_system(&mut cx);

        let expected = Matrix::translate((10.0, 20.0));
        assert_eq!(transform(&cx, a), expected);
        assert_eq!(transform(&cx, b), expected);
        assert_eq!(transform(&cx, c), expected);
        assert!(cx.style.retransform.is_empty());
        // Only entities which clip their content need their own clip path recomputed.
        assert_eq!(cx.style.reclip, [a].into());
    }

    #[test]
    fn nested_change_leaves_ancestors_untouched() {
        let (mut cx, a, b, c) = setup();
        clip(&mut cx, a);
        clip(&mut cx, b);
        translate(&mut cx, a, 10.0, 0.0);
        transform_system(&mut cx);
        cx.style.reclip.clear();

        translate(&mut cx, b, 0.0, 5.0);
        transform_system(&mut cx);

        assert_eq!(transform(&cx, a), Matrix::translate((10.0, 0.0)));
        assert_eq!(transform(&cx, c), Matrix::translate((10.0, 5.0)));
        assert_eq!(cx.style.reclip, [b].into());
    }

    #[test]
    fn counter_translated_descendants_are_unchanged() {
        let (mut cx, a, b, c) = setup();
        clip(&mut cx, b);

        translate(&mut cx, a, 10.0, 0.0);
        translate(&mut cx, b, -10.0, 0.0);
        transform_system(&mut cx);

        assert_eq!(transform(&cx, a), Matrix::translate((10.0, 0.0)));
        assert_eq!(transform(&cx, b), Matrix::new_identity());
        assert_eq!(transform(&cx, c), Matrix::new_identity());
        assert!(cx.style.reclip.is_empty());
    }
}

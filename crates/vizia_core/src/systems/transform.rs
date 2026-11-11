use hashbrown::HashSet;
use vizia_storage::LayoutParentIterator;

use crate::{animation::Interpolator, prelude::*};

/// Computes the local transform of each marked entity, relative to its layout parent.
///
/// Transforms in window coordinates are derived from the local transforms of an entity and its
/// ancestors when needed, so a transform change does not need to visit any descendants. Only the
/// window-space clip paths of descendants which clip their content need to be recomputed.
pub(crate) fn transform_system(cx: &mut Context) -> bool {
    let _scope = crate::profiling::scope("transform_system");
    if cx.style.retransform.is_empty() {
        return false;
    }

    let retransform = std::mem::take(&mut cx.style.retransform);

    let changed: HashSet<Entity> =
        retransform.into_iter().filter(|&entity| update_transform(cx, entity)).collect();

    if changed.is_empty() {
        return false;
    }

    let hover_affected =
        LayoutParentIterator::new(&cx.tree, cx.hovered).any(|entity| changed.contains(&entity));

    for &entity in &changed {
        // The draw system dirties the previous and new areas of redrawn entities, and recomputes
        // their draw bounds.
        cx.needs_redraw(entity);
    }

    for &entity in &cx.cache.own_clips {
        let mut current = Some(entity);
        while let Some(ancestor) = current {
            if changed.contains(&ancestor) {
                cx.style.reclip.insert(entity);
                break;
            }
            if cx.tree.is_window(ancestor) {
                break;
            }
            current = cx.tree.get_layout_parent(ancestor);
        }
    }

    hover_affected
}

/// Recomputes the local transform of an entity. Returns true if the transform changed.
fn update_transform(cx: &mut Context, entity: Entity) -> bool {
    if cx.cache.bounds.get(entity).is_none() {
        return false;
    }
    let new_transform = local_transform(cx, entity).unwrap_or_default();
    let Some(tx) = cx.cache.local_transform.get_mut(entity) else { return false };

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
        cx.cache.absolute_transform(&cx.tree, entity)
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
        cx.cache.own_clips.insert(entity);
    }

    #[test]
    fn descendants_inherit_ancestor_transforms() {
        let (mut cx, a, b, c) = setup();
        translate(&mut cx, a, 10.0, 20.0);
        transform_system(&mut cx);

        let expected = Matrix::translate((10.0, 20.0));
        assert_eq!(transform(&cx, a), expected);
        assert_eq!(transform(&cx, b), expected);
        assert_eq!(transform(&cx, c), expected);
        // Only the local transform of the changed entity is stored.
        assert!(cx.cache.local_transform.get(b).unwrap().is_identity());
        assert!(cx.style.retransform.is_empty());
    }

    #[test]
    fn only_clipping_entities_within_changed_subtrees_are_reclipped() {
        let (mut cx, a, b, c) = setup();
        let d = element(&mut cx, Entity::root());
        clip(&mut cx, b);
        clip(&mut cx, d);
        cx.style.reclip.clear();
        translate(&mut cx, a, 10.0, 0.0);
        transform_system(&mut cx);

        assert_eq!(cx.style.reclip, [b].into());
        assert_eq!(transform(&cx, c), Matrix::translate((10.0, 0.0)));
        assert!(transform(&cx, d).is_identity());
    }

    #[test]
    fn nested_transforms_combine() {
        let (mut cx, a, b, c) = setup();
        translate(&mut cx, a, 10.0, 0.0);
        translate(&mut cx, b, 0.0, 5.0);
        transform_system(&mut cx);

        assert_eq!(transform(&cx, a), Matrix::translate((10.0, 0.0)));
        assert_eq!(transform(&cx, c), Matrix::translate((10.0, 5.0)));

        translate(&mut cx, b, -10.0, 0.0);
        transform_system(&mut cx);
        assert!(transform(&cx, c).is_identity());
    }

    #[test]
    fn changed_transforms_are_redrawn() {
        let (mut cx, a, b, _) = setup();
        cx.windows.entry(Entity::root()).or_default();
        translate(&mut cx, a, 10.0, 0.0);
        transform_system(&mut cx);

        let redraw_list = &cx.windows[&Entity::root()].redraw_list;
        assert!(redraw_list.contains(&a));
        assert!(!redraw_list.contains(&b));
    }

    #[test]
    fn hovered_entity_ancestor_transform_requests_refresh() {
        let (mut cx, ancestor, _, hovered) = setup();
        cx.hovered = hovered;
        translate(&mut cx, ancestor, 10.0, 0.0);

        assert!(transform_system(&mut cx));
    }

    #[test]
    fn unrelated_descendant_transform_does_not_request_refresh() {
        let (mut cx, _, parent, hovered) = setup();
        cx.hovered = parent;
        translate(&mut cx, hovered, 10.0, 0.0);

        assert!(!transform_system(&mut cx));
    }
}

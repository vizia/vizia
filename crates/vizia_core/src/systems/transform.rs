use vizia_storage::LayoutTreeIterator;

use crate::prelude::*;

/// Applies transforms to the layout tree.
pub(crate) fn transform_system(cx: &mut Context) {
    if cx.style.retransform.is_empty() {
        return;
    }

    let iter = LayoutTreeIterator::full(&cx.tree);

    for entity in iter {
        if !cx.style.retransform.contains(&entity) {
            continue;
        }

        let bounds = cx.cache.bounds.get(entity).copied().unwrap();
        if let Some(parent) = cx.tree.get_layout_parent(entity) {
            let parent_transform = cx.cache.transform.get(parent).copied().unwrap();
            if let Some(tx) = cx.cache.transform.get_mut(entity) {
                let scale_factor = cx.style.scale_factor();

                // Apply transform origin.
                let mut origin = cx
                    .style
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
                if let Some(translate) = cx.style.translate.get(entity) {
                    transform = transform * translate.as_transform(bounds, scale_factor);
                }

                // Apply rotation.
                if let Some(rotate) = cx.style.rotate.get(entity) {
                    transform = transform * rotate.as_transform(bounds, scale_factor);
                }

                // Apply scaling.
                if let Some(scale) = cx.style.scale.get(entity) {
                    transform = transform * scale.as_transform(bounds, scale_factor);
                }

                // Apply transform functions.
                if let Some(transforms) = cx.style.transform.get(entity) {
                    // AnimatableSet::tick() already exposes the fully sampled value here,
                    // including easing, looping, direction and CSS effect composition.
                    // Re-interpolating the first/last keyframes discarded that result and
                    // made transform-list animations appear stuck after their first sample.
                    transform = transform * transforms.as_transform(bounds, scale_factor);
                }

                transform = transform * origin;

                let new_transform = parent_transform * transform;

                if *tx != new_transform {
                    cx.style.needs_reclip(entity);
                    let iter = LayoutTreeIterator::subtree(&cx.tree, entity);
                    for descendant in iter {
                        cx.style.needs_reclip(descendant);
                        // Cached draw bounds include transformed and clipped extents.
                        // Invalidate them when ancestor transforms change so dirty/intersection
                        // checks don't cull newly visible descendants.
                        if descendant != entity {
                            cx.cache.draw_bounds.remove(descendant);
                        }
                    }
                }

                *tx = new_transform;
            }
        }
    }

    cx.style.retransform.clear();
}

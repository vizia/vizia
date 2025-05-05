use vizia_storage::LayoutTreeIterator;

use crate::{animation::Interpolator, prelude::*};

/// Applies transforms to the layout tree.
pub(crate) fn transform_system(cx: &mut Context) {
    if cx.style.retransform.is_empty() {
        return;
    }

    let iter = LayoutTreeIterator::full(&cx.tree);

    for entity in iter {
        if !cx.style.retransform.contains(entity) {
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
                    // Check if the transform is currently animating
                    // Get the animation state
                    // Manually interpolate the value to get the overall transform for the current frame
                    if let Some(animation_state) = cx.style.transform.get_active_animation(entity) {
                        if let Some(start) = animation_state.keyframes.first() {
                            if let Some(end) = animation_state.keyframes.last() {
                                let start_transform =
                                    start.value.as_transform(bounds, scale_factor);
                                let end_transform = end.value.as_transform(bounds, scale_factor);
                                let t = animation_state.t;
                                let animated_transform = skia_safe::Matrix::interpolate(
                                    &start_transform,
                                    &end_transform,
                                    t,
                                );
                                transform = transform * animated_transform;
                            }
                        }
                    } else {
                        transform = transform * transforms.as_transform(bounds, scale_factor);
                    }
                }

                transform = transform * origin;

                let new_transform = parent_transform * transform;

                if *tx != new_transform {
                    cx.style.needs_reclip(entity);
                    let iter = LayoutTreeIterator::subtree(&cx.tree, entity);
                    for descendant in iter {
                        cx.style.needs_reclip(descendant);
                    }
                }

                *tx = new_transform;
            }
        }
    }

    cx.style.retransform.clear();
}

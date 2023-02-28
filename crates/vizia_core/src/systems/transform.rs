use crate::{prelude::*, style::Transform2D};
use vizia_id::GenerationalId;
use vizia_style::Transform;

// Propagates transforms down the tree
pub fn transform_system(cx: &mut Context) {
    for entity in cx.tree.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        let parent = cx.tree.get_layout_parent(entity).unwrap();

        //let parent_origin = state.data.get_origin(parent);
        let parent_transform = cx.cache.get_transform(parent);

        let bounds = cx.cache.get_bounds(entity);

        let x = bounds.x + (bounds.w / 2.0);
        let y = bounds.y + (bounds.h / 2.0);
        let mut translate = Transform2D::with_translate(x, y);

        let mut transform = parent_transform;
        // let mut transform = Transform2D::default();
        transform.premultiply(&translate);

        translate.inverse();

        if let Some(transforms) = cx.style.transform.get(entity) {
            // Check if the transform is currently animating
            // Get the animation state
            // Manually interpolate the value to get the overall transform for the current frame

            if let Some(animation_state) = cx.style.transform.get_active_animation(entity) {
                if let Some(start) = animation_state.keyframes.first() {
                    if let Some(end) = animation_state.keyframes.last() {
                        let start_transform =
                            Transform2D::from_style_transforms(&start.1, bounds, cx.scale_factor());
                        let end_transform =
                            Transform2D::from_style_transforms(&end.1, bounds, cx.scale_factor());
                        let t = animation_state.t;
                        let animated_transform =
                            Transform2D::interpolate(&start_transform, &end_transform, t);
                        transform.premultiply(&animated_transform);
                    }
                }
            } else {
                transform.premultiply(&Transform2D::from_style_transforms(
                    transforms,
                    bounds,
                    cx.scale_factor(),
                ));
            }
        }

        transform.premultiply(&translate);

        // println!("Set transform: {} {:?}", entity, transform);

        cx.cache.set_transform(entity, transform);
    }
}

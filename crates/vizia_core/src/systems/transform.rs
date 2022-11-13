use crate::{prelude::*, style::Transform2D};
use vizia_id::GenerationalId;

pub fn transform_system(cx: &mut Context, tree: &Tree<Entity>) {
    for entity in tree.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        let parent = tree.get_parent(entity).unwrap();
        //let parent_origin = state.data.get_origin(parent);
        let parent_transform = cx.cache.get_transform(parent);

        cx.cache.set_transform(entity, Transform2D::identity());

        cx.cache.set_transform(entity, parent_transform);

        let bounds = cx.cache.get_bounds(entity);

        if let Some(mut transform) = cx.style.transform.get(entity).copied() {
            // Scale translation by DPI
            transform.0[4] = transform.0[4] * cx.style.dpi_factor as f32;
            transform.0[5] = transform.0[5] * cx.style.dpi_factor as f32;

            let mut t = parent_transform;
            let x = bounds.x + (bounds.w / 2.0);
            let y = bounds.y + (bounds.h / 2.0);
            let mut translate = Transform2D::identity();
            translate.translate(x, y);
            t.premultiply(&translate).premultiply(&transform);
            translate.inverse();
            t.premultiply(&translate);
            cx.cache.set_transform(entity, t);
        }
    }
}

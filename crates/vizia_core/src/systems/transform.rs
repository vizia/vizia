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

        if let Some(transform) = cx.style.transform.get(entity).copied() {
            let mut t = parent_transform;
            let x = bounds.x + (bounds.w / 2.0);
            let y = bounds.y + (bounds.h / 2.0);
            let mut translate = Transform2D::identity();
            translate.translate(x, y);
            // t.translate(-x, -y);
            t.premultiply(&translate).premultiply(&transform);
            translate.inverse();
            t.premultiply(&translate);
            // t.translate(x, y);
            cx.cache.set_transform(entity, t);
        }

        // if let Some((tx, ty)) = cx.style.translate.get(entity).copied() {
        //     let scale = cx.style.dpi_factor as f32;
        //     cx.cache.set_translate(entity, (tx * scale, ty * scale));
        // }

        // if let Some(rotate) = cx.style.rotate.get(entity).copied() {
        //     let x = bounds.x + (bounds.w / 2.0);
        //     let y = bounds.y + (bounds.h / 2.0);
        //     cx.cache.set_translate(entity, (x, y));
        //     cx.cache.set_rotate(entity, (rotate).to_radians());
        //     cx.cache.set_translate(entity, (-x, -y));
        // }

        // if let Some((scalex, scaley)) = cx.style.scale.get(entity).copied() {
        //     let x = bounds.x + (bounds.w / 2.0);
        //     let y = bounds.y + (bounds.h / 2.0);
        //     cx.cache.set_translate(entity, (x, y));
        //     cx.cache.set_scale(entity, (scalex, scaley));
        //     cx.cache.set_translate(entity, (-x, -y));
        // }
    }
}

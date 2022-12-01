use crate::{prelude::*, style::Transform2D};
use vizia_id::GenerationalId;
use vizia_style::Transform;

// Propagates transforms down the tree
pub fn transform_system(cx: &mut Context, tree: &Tree<Entity>) {
    for entity in tree.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        let parent = tree.get_layout_parent(entity).unwrap();

        //let parent_origin = state.data.get_origin(parent);
        let parent_transform = cx.cache.get_transform(parent);

        cx.cache.set_transform(entity, Transform2D::identity());

        cx.cache.set_transform(entity, parent_transform);

        let bounds = cx.cache.get_bounds(entity);

        let x = bounds.x + (bounds.w / 2.0);
        let y = bounds.y + (bounds.h / 2.0);
        let mut translate = Transform2D::with_translate(x, y);

        let mut current_transform = parent_transform;

        current_transform.premultiply(&translate);

        translate.inverse();

        if let Some(transforms) = cx.style.transform.get(entity) {
            for transform in transforms.iter() {
                match transform {
                    Transform::Translate(translate) => {
                        let tx = translate.x.to_pixels(bounds.w);
                        let ty = translate.y.to_pixels(bounds.h);

                        let t = Transform2D::with_translate(tx, ty);
                        current_transform.premultiply(&t);
                    }

                    Transform::TranslateX(value) => {
                        let tx = value.to_pixels(bounds.w);

                        let t = Transform2D::with_translate(tx, 0.0);
                        current_transform.premultiply(&t);
                    }

                    Transform::Rotate(angle) => {
                        let t = Transform2D::with_rotate(angle.to_radians());
                        current_transform.premultiply(&t);
                    }

                    _ => {}
                }
            }
        }

        current_transform.premultiply(&translate);

        cx.cache.set_transform(entity, current_transform);

        //state.data.set_origin(entity, parent_origin);

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
        //println!("End");

        // if let Some((scalex, scaley)) = cx.style.scale.get(entity).copied() {
        //     let x = bounds.x + (bounds.w / 2.0);
        //     let y = bounds.y + (bounds.h / 2.0);
        //     cx.cache.set_translate(entity, (x, y));
        //     cx.cache.set_scale(entity, (scalex, scaley));
        //     cx.cache.set_translate(entity, (-x, -y));
        // }
    }
}

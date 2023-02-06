use crate::{cache::BoundingBox, prelude::*, systems::transform};
use vizia_id::GenerationalId;
use vizia_style::{clip, Clip};

// Iterate tree and apply clip region to nodes based on specified clip widget and parent clip region.
pub fn clipping_system(cx: &mut Context, tree: &Tree<Entity>) {
    for entity in tree.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        if tree.is_ignored(entity) {
            continue;
        }

        let parent = tree.get_layout_parent(entity).unwrap();

        let parent_clip_region = cx.cache.get_clip_region(parent);
        let mut parent_transform = cx.cache.get_transform(parent);
        let parent_bounds = cx.cache.get_bounds(parent);

        let overflowx = cx.style.overflowx.get(entity).copied().unwrap_or_default();
        let overflowy = cx.style.overflowy.get(entity).copied().unwrap_or_default();

        let bounds = cx.cache.get_bounds(entity);

        let root_bounds = cx.cache.get_bounds(Entity::root());

        // if let Some(clip_region) = cx.style.clip.get(entity) {
        //     match clip_region {
        //         Clip::Auto => bounds,
        //         Clip::Shape(rect) => parent_bounds.shrink_sides(rect.3, rect.0, rect.2, rect.1),
        //     };
        // }

        let clip_bounds = cx
            .style
            .clip
            .get(entity)
            .map(|clip| match clip {
                Clip::Auto => bounds,
                Clip::Shape(rect) => bounds.shrink_sides(
                    rect.3.to_px().unwrap() * cx.style.dpi_factor as f32,
                    rect.0.to_px().unwrap() * cx.style.dpi_factor as f32,
                    rect.1.to_px().unwrap() * cx.style.dpi_factor as f32,
                    rect.2.to_px().unwrap() * cx.style.dpi_factor as f32,
                ),
            })
            .unwrap_or(bounds);

        let mut clipping = match (overflowx, overflowy) {
            (Overflow::Visible, Overflow::Visible) => {
                cx.cache.set_clip_region(entity, parent_clip_region);
                continue;
            }
            (Overflow::Hidden, Overflow::Visible) => {
                let left = clip_bounds.left();
                let right = clip_bounds.right();
                let top = root_bounds.top();
                let bottom = root_bounds.bottom();
                BoundingBox::from_min_max(left, top, right, bottom)
            }
            (Overflow::Visible, Overflow::Hidden) => {
                let left = root_bounds.left();
                let right = root_bounds.right();
                let top = clip_bounds.top();
                let bottom = clip_bounds.bottom();
                BoundingBox::from_min_max(left, top, right, bottom)
            }
            (Overflow::Hidden, Overflow::Hidden) => clip_bounds,
        };

        let mut transform = cx.cache.get_transform(entity);
        transform.inverse();

        parent_transform[4] = parent_clip_region.center().0;
        parent_transform[5] = parent_clip_region.center().1;

        parent_transform.multiply(&transform);

        let ex = parent_clip_region.w / 2.0;
        let ey = parent_clip_region.h / 2.0;

        let tex = ex * parent_transform[0].abs() + ey * parent_transform[2].abs();
        let tey = ex * parent_transform[1].abs() + ey * parent_transform[3].abs();

        let rect = BoundingBox {
            x: parent_transform[4] - tex,
            y: parent_transform[5] - tey,
            w: tex * 2.0,
            h: tey * 2.0,
        };

        // let (left, top) =
        //     parent_transform.transform_point(parent_clip_region.x, parent_clip_region.y);
        // let (right, bottom) = parent_transform.transform_point(
        //     parent_clip_region.x + parent_clip_region.w,
        //     parent_clip_region.y + parent_clip_region.h,
        // );

        // let rect = BoundingBox::from_min_max(left, top, right, bottom);

        let res = rect.intersection(&clipping);

        // // println!("1: {} {:?}", entity, clipping);
        // // println!("r: {} bottom: {}", right, bottom);
        // clipping.x = left;
        // clipping.y = top;
        // clipping.w = right - clipping.x;
        // clipping.h = bottom - clipping.y;
        // // println!("2: {} {:?}", entity, clipping);
        // clipping = clipping.intersection(&parent_clip_region);
        // // println!("3: {} {:?}", entity, clipping);

        // transform.inverse();
        // let (left, top) = transform.transform_point(clipping.x, clipping.y);
        // let (right, bottom) =
        //     transform.transform_point(clipping.x + clipping.w, clipping.y + clipping.h);
        // println!("r: {} bottom: {}", right, bottom);
        // clipping.x = left;
        // clipping.y = top;
        // clipping.w = right - clipping.x;
        // clipping.h = bottom - clipping.y;

        // let clip_region = if overflow == Overflow::Hidden {
        //     let clip_region = cx.style.clip.get(entity).cloned().unwrap_or(entity);

        //     let clip_x = cx.cache.get_posx(clip_widget);
        //     let clip_y = cx.cache.get_posy(clip_widget);
        //     let clip_w = cx.cache.get_width(clip_widget);
        //     let clip_h = cx.cache.get_height(clip_widget);

        //     let mut intersection = BoundingBox::default();
        //     intersection.x = clip_x.max(parent_clip_region.x);
        //     intersection.y = clip_y.max(parent_clip_region.y);

        //     intersection.w = if clip_x + clip_w < parent_clip_region.x + parent_clip_region.w {
        //         clip_x + clip_w - intersection.x
        //     } else {
        //         parent_clip_region.x + parent_clip_region.w - intersection.x
        //     };

        //     intersection.h = if clip_y + clip_h < parent_clip_region.y + parent_clip_region.h {
        //         clip_y + clip_h - intersection.y
        //     } else {
        //         parent_clip_region.y + parent_clip_region.h - intersection.y
        //     };

        //     intersection.w = intersection.w.max(0.0);
        //     intersection.h = intersection.h.max(0.0);

        //     intersection
        // } else {
        //     parent_clip_region
        // };

        // Absolute positioned nodes ignore overflow hidden
        //if position_type == PositionType::SelfDirected {
        //    cx.cache().set_clip_region(entity, root_clip_region);
        //} else {
        // println!("Set clip region: {} {:?}", entity, clipping);
        cx.cache.set_clip_region(entity, res);
        //}
    }
}

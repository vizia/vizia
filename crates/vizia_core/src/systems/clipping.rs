use crate::style::SystemFlags;
use crate::{cache::BoundingBox, prelude::*};
use vizia_id::GenerationalId;

// Iterate tree and apply clip region to nodes based on specified clip widget and parent clip region.
pub fn clipping_system(cx: &mut Context) {
    if cx.style.system_flags.contains(SystemFlags::RECLIP) {
        for entity in cx.tree.into_iter() {
            if entity == Entity::root() {
                continue;
            }

            if cx.tree.is_ignored(entity) {
                continue;
            }

            let parent = cx.tree.get_layout_parent(entity).unwrap();

            let parent_clip_region = cx.cache.get_clip_region(parent);

            let overflow = cx.style.overflow.get(entity).cloned().unwrap_or_default();

            let clip_region = if overflow == Overflow::Hidden {
                let clip_widget = cx.style.clip_widget.get(entity).cloned().unwrap_or(entity);

                let clip_x = cx.cache.get_posx(clip_widget);
                let clip_y = cx.cache.get_posy(clip_widget);
                let clip_w = cx.cache.get_width(clip_widget);
                let clip_h = cx.cache.get_height(clip_widget);

                let mut intersection = BoundingBox::default();
                intersection.x = clip_x.max(parent_clip_region.x);
                intersection.y = clip_y.max(parent_clip_region.y);

                intersection.w = if clip_x + clip_w < parent_clip_region.x + parent_clip_region.w {
                    clip_x + clip_w - intersection.x
                } else {
                    parent_clip_region.x + parent_clip_region.w - intersection.x
                };

                intersection.h = if clip_y + clip_h < parent_clip_region.y + parent_clip_region.h {
                    clip_y + clip_h - intersection.y
                } else {
                    parent_clip_region.y + parent_clip_region.h - intersection.y
                };

                intersection.w = intersection.w.max(0.0);
                intersection.h = intersection.h.max(0.0);

                intersection
            } else {
                parent_clip_region
            };

            // Absolute positioned nodes ignore overflow hidden
            //if position_type == PositionType::SelfDirected {
            //    cx.cache().set_clip_region(entity, root_clip_region);
            //} else {
            cx.cache.set_clip_region(entity, clip_region);
            //}
        }

        // If clipping has changed then redraw
        cx.style.system_flags.set(SystemFlags::REDRAW, true);

        cx.style.system_flags.set(SystemFlags::RECLIP, false);
    }
}

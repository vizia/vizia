use crate::prelude::*;
use vizia_id::GenerationalId;

pub fn z_ordering_system(cx: &mut Context) {
    for entity in cx.tree.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        if cx.tree.is_ignored(entity) {
            continue;
        }

        let parent = cx.tree.get_layout_parent(entity).unwrap();

        if let Some(z_order) = cx.style.z_order.get(entity).copied() {
            cx.cache.set_z_index(entity, z_order);
        } else {
            let parent_z_order = cx.cache.get_z_index(parent);
            cx.cache.set_z_index(entity, parent_z_order);
        }
    }
}

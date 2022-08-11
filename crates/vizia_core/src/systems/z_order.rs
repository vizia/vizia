use crate::prelude::*;

pub fn z_ordering_system(cx: &mut Context, tree: &Tree) {
    for entity in tree.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        if tree.is_ignored(entity) {
            continue;
        }

        let parent = tree.get_layout_parent(entity).unwrap();

        if let Some(z_order) = cx.style.z_order.get(entity).copied() {
            cx.cache.set_z_index(entity, z_order);
        } else {
            let parent_z_order = cx.cache.get_z_index(parent);
            cx.cache.set_z_index(entity, parent_z_order);
        }
    }
}

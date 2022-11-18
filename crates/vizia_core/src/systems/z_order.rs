use crate::prelude::*;
use vizia_id::GenerationalId;

pub fn z_indexing_system(cx: &mut Context, tree: &Tree<Entity>) {
    for entity in tree.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        if tree.is_ignored(entity) {
            continue;
        }

        let parent = tree.get_layout_parent(entity).unwrap();

        if let Some(z_index) = cx.style.z_index.get(entity).copied() {
            cx.cache.set_z_index(entity, z_index);
        } else {
            let parent_z_index = cx.cache.get_z_index(parent);
            cx.cache.set_z_index(entity, parent_z_index);
        }
    }
}

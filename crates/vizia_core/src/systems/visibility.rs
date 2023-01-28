use crate::prelude::*;
use crate::style::SystemFlags;
use vizia_id::GenerationalId;

pub fn visibility_system(cx: &mut Context) {
    if cx.style.system_flags.contains(SystemFlags::REHIDE) {
        let mut draw_tree: Vec<Entity> = cx.tree.into_iter().collect();
        draw_tree.sort_by_cached_key(|entity| cx.cache.get_z_index(*entity));

        for entity in draw_tree.into_iter() {
            if entity == Entity::root() {
                continue;
            }

            if cx.tree.is_ignored(entity) {
                continue;
            }

            let parent = cx.tree.get_layout_parent(entity).unwrap();

            if cx.cache.get_visibility(parent) == Visibility::Invisible {
                cx.cache.set_visibility(entity, Visibility::Invisible);
            } else {
                if let Some(visibility) = cx.style.visibility.get(entity).copied() {
                    cx.cache.set_visibility(entity, visibility);
                } else {
                    cx.cache.set_visibility(entity, Visibility::Visible);
                }
            }

            if cx.cache.get_display(parent) == Display::None {
                cx.cache.set_display(entity, Display::None);
            } else {
                if let Some(display) = cx.style.display.get(entity).copied() {
                    cx.cache.set_display(entity, display);
                } else {
                    cx.cache.set_display(entity, Display::Flex);
                }
            }

            let parent_opacity = cx.cache.get_opacity(parent);

            let opacity = cx.style.opacity.get(entity).cloned().unwrap_or_default();

            cx.cache.set_opacity(entity, opacity.0 * parent_opacity);
        }

        cx.style.system_flags.set(SystemFlags::REHIDE, false);
    }
}

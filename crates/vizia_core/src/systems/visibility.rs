use crate::prelude::*;
use crate::style::SystemFlags;
use vizia_id::GenerationalId;
use vizia_storage::DrawIterator;

pub fn visibility_system(cx: &mut Context) {
    if cx.style.system_flags.contains(SystemFlags::REHIDE) {
        let draw_tree = DrawIterator::full(&cx.tree);

        for entity in draw_tree {
            if entity == Entity::root() {
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

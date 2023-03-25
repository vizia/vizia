use morphorm::Node;

use crate::prelude::*;
use crate::style::SystemFlags;

pub(crate) fn layout_system(cx: &mut Context) {
    // text_constraints_system(cx);

    if cx.style.system_flags.contains(SystemFlags::RELAYOUT) {
        Entity::root().layout(&mut cx.cache, &cx.tree, &cx.style, &mut cx.text_context);

        // If layout has changed then redraw
        cx.style.system_flags.set(SystemFlags::REDRAW, true);

        for entity in cx.tree.into_iter() {
            if cx.text_context.has_buffer(entity) {
                let auto_width = cx.style.width.get(entity).copied().unwrap_or_default().is_auto();
                let auto_height =
                    cx.style.height.get(entity).copied().unwrap_or_default().is_auto();
                if !auto_width && !auto_height {
                    let width = cx.cache.bounds.get(entity).unwrap().w;
                    cx.text_context.with_buffer(entity, |fs, buf| {
                        buf.set_size(fs, width.ceil(), f32::MAX);
                    });
                }
            }

            if let Some(parent) = cx.tree.get_layout_parent(entity) {
                let parent_bounds = cx.cache.get_bounds(parent);
                if let Some(bounds) = cx.cache.bounds.get_mut(entity) {
                    bounds.x += parent_bounds.x;
                    bounds.y += parent_bounds.y;
                }
            }
        }

        // Defer resetting the layout system flag to the geometry changed system
    }
}

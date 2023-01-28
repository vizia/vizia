use morphorm::layout;

use crate::prelude::*;
use crate::style::SystemFlags;

use super::text_constraints_system;

pub(crate) fn layout_system(cx: &mut Context) {
    text_constraints_system(cx);

    if cx.style.system_flags.contains(SystemFlags::RELAYOUT) {
        layout(&mut cx.cache, &cx.tree, &cx.style, &mut cx.text_context);

        // If layout has changed then reclip, retransform, and redraw
        cx.style.system_flags.set(SystemFlags::RETRANSFORM, true);
        cx.style.system_flags.set(SystemFlags::RECLIP, true);
        cx.style.system_flags.set(SystemFlags::REDRAW, true);

        for entity in cx.tree.into_iter() {
            if cx.text_context.has_buffer(entity) {
                let w = cx.cache.bounds.get(entity).unwrap().w;
                cx.text_context.with_buffer(entity, |buf| {
                    buf.set_size(w as i32, i32::MAX);
                });
            }
        }
        // Defer this to the geometry changed system for now
        // cx.style.system_flags.set(SystemFlags::RELAYOUT, false);
    }
}

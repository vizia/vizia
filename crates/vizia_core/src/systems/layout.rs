use morphorm::layout;

use crate::prelude::*;

use super::text_constraints_system;

pub(crate) fn layout_system(cx: &mut Context) {
    text_constraints_system(cx);

    if cx.style.needs_relayout {
        layout(&mut cx.cache, &cx.tree, &cx.style, &mut cx.text_context);

        cx.style.needs_relayout = false;
        cx.style.needs_redraw = true;

        for entity in cx.tree.into_iter() {
            if cx.text_context.has_buffer(entity) {
                let w = cx.cache.bounds.get(entity).unwrap().w;
                cx.text_context.with_buffer(entity, |buf| {
                    buf.set_size(w as i32, i32::MAX);
                });
            }
        }
    }
}

use morphorm::layout;

use crate::prelude::*;

use super::text_constraints_system;

pub(crate) fn layout_system(cx: &mut Context, tree: &Tree<Entity>) {
    if cx.style.needs_relayout {
        text_constraints_system(cx, tree);

        layout(&mut cx.cache, &cx.tree, &cx.style, &mut cx.cosmic_context);

        cx.style.needs_relayout = false;
    }
}

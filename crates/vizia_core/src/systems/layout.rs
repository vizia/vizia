use femtovg::TextContext;
use morphorm::layout;

use crate::{prelude::*, resource::ResourceManager, style::Style};

use super::text_constraints_system;

pub(crate) fn layout_system(cx: &mut Context, tree: &Tree<Entity>) {
    if cx.style.needs_relayout {
        text_constraints_system(cx, tree);

        // hack!
        let mut store = (Style::default(), TextContext::default(), ResourceManager::default());
        std::mem::swap(&mut store.0, &mut cx.style);
        std::mem::swap(&mut store.1, &mut cx.text_context);
        std::mem::swap(&mut store.2, &mut cx.resource_manager);

        layout(&mut cx.cache, &cx.tree, &store);
        std::mem::swap(&mut store.0, &mut cx.style);
        std::mem::swap(&mut store.1, &mut cx.text_context);
        std::mem::swap(&mut store.2, &mut cx.resource_manager);

        cx.style.needs_relayout = false;
    }
}

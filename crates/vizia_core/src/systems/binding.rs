use crate::{context::SIGNAL_REBUILDS, prelude::*};
use hashbrown::HashSet;

pub(crate) fn binding_system(cx: &mut Context) {
    // Drain reactive signal rebuild requests queued by `Binding` updater effects.
    let signal_rebuilds =
        SIGNAL_REBUILDS.with_borrow_mut(|set| set.drain().collect::<HashSet<_>>());

    if !signal_rebuilds.is_empty() {
        // Update bindings in tree order to ensure parents are updated before children.
        let ordered =
            cx.tree.into_iter().filter(|ent| signal_rebuilds.contains(ent)).collect::<Vec<_>>();

        for entity in ordered {
            if cx.entity_manager.is_alive(entity) {
                update_binding(cx, entity);
            }
        }
    }
}

fn update_binding(cx: &mut Context, observer: Entity) {
    if let Some(mut binding) = cx.bindings.remove(&observer) {
        cx.with_current(observer, |cx| {
            binding.update(cx);
        });
        cx.bindings.insert(observer, binding);
    }
}

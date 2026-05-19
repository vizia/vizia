use crate::{context::SIGNAL_REBUILDS, prelude::*};
use hashbrown::HashSet;

pub(crate) fn binding_system(cx: &mut Context) {
    // Drain reactive signal rebuild requests queued by `Binding` updater effects,
    // but only consume entries for this context. Leave other contexts' entries queued.
    let signal_rebuilds = SIGNAL_REBUILDS.with_borrow_mut(|set| {
        let mut ours = HashSet::new();
        let mut keep = HashSet::new();

        for rebuild in set.drain() {
            if rebuild.context_id == cx.context_id {
                ours.insert(rebuild.entity);
            } else {
                keep.insert(rebuild);
            }
        }

        *set = keep;
        ours
    });

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

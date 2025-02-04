use crate::{binding::StoreId, model::ModelOrView, prelude::*};
use hashbrown::{HashMap, HashSet};
use std::any::TypeId;

pub(crate) fn binding_system(cx: &mut Context) {
    let mut observers: HashMap<Entity, (Entity, TypeId, StoreId)> = HashMap::new();

    for (entity, stores) in cx.stores.iter_mut() {
        for (store_id, store) in stores.iter() {
            let model_id = store.source();

            observers.extend(
                store.observers().iter().map(|e| (*e, (*entity, model_id, store_id.clone()))),
            );
        }
    }

    if !observers.is_empty() {
        // Sort observers into tree ordering.
        let ordered_observers = cx
            .tree
            .into_iter()
            .filter_map(|ent| observers.get(&ent).map(|e| (ent, e.clone())))
            .collect::<Vec<_>>();

        let mut updated_stores: HashSet<StoreId> = HashSet::new();

        // Update observers in tree order.
        for (observer, (source, model_id, store_id)) in ordered_observers.into_iter() {
            // Skip observers that have been destroyed.
            if !cx.entity_manager.is_alive(observer) {
                continue;
            }

            if updated_stores.contains(&store_id) {
                update_binding(cx, observer);
            } else if let Some(store) =
                cx.stores.get_mut(&source).and_then(|stores| stores.get_mut(&store_id))
            {
                let view = cx
                    .views
                    .get(&source)
                    .filter(|view| view.id() == model_id)
                    .map(|view| ModelOrView::View(view.as_ref()));

                let model_or_view = if view.is_some() {
                    view
                } else {
                    cx.models
                        .get(&source)
                        .and_then(|models| models.get(&model_id))
                        .map(|model| ModelOrView::Model(model.as_ref()))
                };

                if let Some(model_or_view) = model_or_view {
                    if store.update(model_or_view) {
                        updated_stores.insert(store_id);
                        update_binding(cx, observer);
                    }
                }
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

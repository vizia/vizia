use std::collections::HashSet;

use crate::{model::ModelOrView, prelude::*};

pub(crate) fn binding_system(cx: &mut Context) {
    let mut observers: HashSet<Entity> = HashSet::new();

    for entity in cx.tree.into_iter() {
        if let Some(model_data_store) = cx.data.get_mut(entity) {
            // Determine observers of model data
            for (_, model) in model_data_store.models.iter() {
                let model = ModelOrView::Model(model.as_ref());

                for (_, store) in model_data_store.stores.iter_mut() {
                    if store.update(model) {
                        observers.extend(store.observers().iter())
                    }
                }
            }

            // Determine observers of view data
            for (_, store) in model_data_store.stores.iter_mut() {
                if let Some(view_handler) = cx.views.get(&entity) {
                    let view_model = ModelOrView::View(view_handler.as_ref());

                    if store.update(view_model) {
                        observers.extend(store.observers().iter())
                    }
                }
            }
        }
    }

    for img in cx.resource_manager.images.values_mut() {
        if img.dirty {
            observers.extend(img.observers.iter());
            img.dirty = false;
        }
    }

    let ordered_observers =
        cx.tree.into_iter().filter(|ent| observers.contains(ent)).collect::<Vec<_>>();

    // Update observers in tree order
    for observer in ordered_observers.into_iter() {
        // Skip obervers that have been destroyed
        if !cx.entity_manager.is_alive(observer) {
            continue;
        }

        if let Some(mut binding) = cx.bindings.remove(&observer) {
            cx.with_current(observer, |cx| {
                binding.update(cx);
            });
            cx.bindings.insert(observer, binding);
        }
    }
}

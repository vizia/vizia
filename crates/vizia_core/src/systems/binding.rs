use crate::{binding::StoreId, model::ModelOrView, prelude::*};
use std::{any::TypeId, collections::HashMap};

pub(crate) fn binding_system(cx: &mut Context) {
    let mut observers: HashMap<Entity, (Entity, Option<TypeId>, StoreId)> = HashMap::new();

    for (entity, model_data_store) in cx.data.iter_mut() {
        for (store_id, store) in model_data_store.stores.iter() {
            // Determine observers of model data.
            for (model_id, model) in model_data_store.models.iter() {
                let model: ModelOrView<'_> = ModelOrView::Model(model.as_ref());

                if store.contains_source(model) {
                    observers.extend(
                        store
                            .observers()
                            .iter()
                            .map(|e| (*e, (*entity, Some(*model_id), *store_id))),
                    );
                }
            }

            // Determine observers of view data.
            if let Some(view_handler) = cx.views.get(entity) {
                let view = ModelOrView::View(view_handler.as_ref());

                if store.contains_source(view) {
                    observers
                        .extend(store.observers().iter().map(|e| (*e, (*entity, None, *store_id))))
                }
            }
        }
    }

    // for img in cx.resource_manager.images.values_mut() {
    //     if img.dirty {
    //         observers.extend(img.observers.iter());
    //         img.dirty = false;
    //     }
    // }

    if !observers.is_empty() {
        // Sort observers into tree ordering.
        let ordered_observers = cx
            .tree
            .into_iter()
            .filter_map(|ent| observers.get(&ent).map(|e| (ent, *e)))
            .collect::<Vec<_>>();

        // Update observers in tree order.
        for (observer, (source, model_id, store_id)) in ordered_observers.into_iter() {
            // Skip observers that have been destroyed.
            if !cx.entity_manager.is_alive(observer) {
                continue;
            }

            if let Some(model_data_store) = cx.data.get_mut(&source) {
                if let Some(store) = model_data_store.stores.get_mut(&store_id) {
                    let model_or_view = if let Some(model_id) = model_id {
                        model_data_store
                            .models
                            .get(&model_id)
                            .map(|model| ModelOrView::Model(model.as_ref()))
                    } else {
                        cx.views.get(&source).map(|view| ModelOrView::View(view.as_ref()))
                    };

                    if let Some(model_or_view) = model_or_view {
                        if store.update(model_or_view) {
                            if let Some(mut binding) = cx.bindings.remove(&observer) {
                                cx.with_current(observer, |cx| {
                                    binding.update(cx);
                                });
                                cx.bindings.insert(observer, binding);
                            }
                        }
                    }
                }
            }
        }
    }
}

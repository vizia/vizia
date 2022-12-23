use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
};

use vizia_storage::TreeExt;

use crate::{
    context::{Context, DataContext},
    entity::Entity,
    state::{LensCache, ModelOrView, StoreId},
};

use super::{BasicStore, Data, Lens, Store};

pub trait Bindable: 'static + Clone {
    type Output;
    fn get_val2<C: DataContext>(&self, cx: &C) -> Self::Output;
    fn insert_store(self, cx: &mut Context, entity: Entity);
    fn name(&self) -> Option<&'static str>;
}

impl<L: Lens> Bindable for L
where
    L::Target: Data,
{
    type Output = L::Target;

    fn get_val2<C: DataContext>(&self, cx: &C) -> Self::Output {
        self.view(
            cx.data().expect("Failed to get data from context. Has it been built into the tree?"),
            |t| {
                t.expect("Lens failed to resolve. Do you want to use LensExt::get_fallible?")
                    .clone()
            },
        )
    }

    fn insert_store(self, cx: &mut Context, id: Entity) {
        let ancestors = cx.current().parent_iter(&cx.tree).collect::<HashSet<_>>();
        let new_ancestors = id.parent_iter(&cx.tree).collect::<Vec<_>>();

        let data = &mut cx.data;
        let storages = &mut cx.stores;

        fn insert_store<L: Lens>(
            ancestors: &HashSet<Entity>,
            stores: &mut HashMap<StoreId, Box<dyn Store>>,
            model_data: ModelOrView,
            lens: L,
            id: Entity,
        ) where
            L::Target: Data,
        {
            let key = lens.cache_key();

            if let Some(store) = stores.get_mut(&key) {
                let observers = store.observers();

                if ancestors.intersection(observers).next().is_none() {
                    store.add_observer(id);
                }
            } else {
                let mut observers = HashSet::new();
                observers.insert(id);

                let model = model_data.downcast_ref::<L::Source>().unwrap();

                let old = lens.view(model, |t| t.cloned());

                let store = Box::new(BasicStore { entity: id, lens, old, observers });

                stores.insert(key, store);
            }
        }

        for entity in new_ancestors {
            if let Some(models) = data.get_mut(entity) {
                // Check for model store for first lens
                if let Some(model_data) = models.get(&TypeId::of::<L::Source>()) {
                    if !storages.contains(entity) {
                        storages.insert(entity, HashMap::new()).unwrap();
                    }

                    if let Some(stores) = storages.get_mut(entity) {
                        insert_store(
                            &ancestors,
                            stores,
                            ModelOrView::Model(model_data.as_ref()),
                            self,
                            id,
                        );
                    }

                    break;
                }

                // Check for view store
                if let Some(view_handler) = cx.views.get(&entity) {
                    if view_handler.as_any_ref().is::<L::Source>() {
                        if !storages.contains(entity) {
                            storages.insert(entity, HashMap::new()).unwrap();
                        }

                        if let Some(stores) = storages.get_mut(entity) {
                            insert_store(
                                &ancestors,
                                stores,
                                ModelOrView::View(view_handler.as_ref()),
                                self,
                                id,
                            );
                        }

                        break;
                    }
                }
            }
        }
    }

    fn name(&self) -> Option<&'static str> {
        self.name()
    }
}

impl<L1: Lens, L2: Lens> Bindable for (L1, L2)
where
    L1::Target: Data,
    L2::Target: Data,
{
    type Output = (L1::Target, L2::Target);

    fn get_val2<C: DataContext>(&self, cx: &C) -> Self::Output {
        (self.0.get_val2(cx), self.1.get_val2(cx))
    }

    fn insert_store(self, cx: &mut Context, id: Entity) {
        self.0.insert_store(cx, id);
        self.1.insert_store(cx, id);
    }

    fn name(&self) -> Option<&'static str> {
        // TODO: It's not possible to concatenate two static str's so to return a combination of the stored names
        // we may need to change the signature of `name()` to return String or maybe CowStr.
        self.0.name()
    }
}

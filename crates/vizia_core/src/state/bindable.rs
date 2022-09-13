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

pub trait Bindable {
    fn do_something(self, cx: &mut Context, entity: Entity);
}

impl<L: Lens> Bindable for L
where
    L::Target: Data,
{
    fn do_something(self, cx: &mut Context, id: Entity) {
        println!("Create store: {}", id);

        let ancestors = cx.current().parent_iter(&cx.tree).collect::<HashSet<_>>();
        let new_ancestors = id.parent_iter(&cx.tree).collect::<Vec<_>>();

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

                println!("add store, observers: {:?}", observers);
                let store = Box::new(BasicStore { entity: id, lens, old, observers });

                stores.insert(key, store);
            }
        }

        for entity in new_ancestors {
            if let Some(model_data_store) = cx.data.get_mut(entity) {
                // Check for model store
                if let Some(model_data) = model_data_store.models.get(&TypeId::of::<L::Source>()) {
                    insert_store(
                        &ancestors,
                        &mut model_data_store.stores,
                        ModelOrView::Model(model_data.as_ref()),
                        self,
                        id,
                    );

                    break;
                }

                // Check for view store
                if let Some(view_handler) = cx.views.get(&entity) {
                    if view_handler.as_any_ref().is::<L::Source>() {
                        insert_store(
                            &ancestors,
                            &mut model_data_store.stores,
                            ModelOrView::View(view_handler.as_ref()),
                            self,
                            id,
                        );

                        break;
                    }
                }
            }
        }
    }
}

impl<L1: Lens, L2: Lens> Bindable for (L1, L2)
where
    L1::Target: Data,
{
    fn do_something(self, cx: &mut Context, id: Entity) {
        let ancestors = cx.current().parent_iter(&cx.tree).collect::<HashSet<_>>();
        let new_ancestors = id.parent_iter(&cx.tree).collect::<Vec<_>>();

        fn insert_store<L1: Lens>(
            ancestors: &HashSet<Entity>,
            stores: &mut HashMap<StoreId, Box<dyn Store>>,
            model_data: ModelOrView,
            lens: L1,
            id: Entity,
        ) where
            L1::Target: Data,
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

                let model = model_data.downcast_ref::<L1::Source>().unwrap();

                let old = lens.view(model, |t| t.cloned());

                println!("add store, observers: {:?}", observers);
                let store = Box::new(BasicStore { entity: id, lens, old, observers });

                stores.insert(key, store);
            }
        }

        // Iterate up the tree and find the store
        for entity in new_ancestors {
            if let Some(model_data_store) = cx.data.get_mut(entity) {
                // Check for model store
                if let Some(model_data) = model_data_store.models.get(&TypeId::of::<L1::Source>()) {
                    insert_store(
                        &ancestors,
                        &mut model_data_store.stores,
                        ModelOrView::Model(model_data.as_ref()),
                        self.0,
                        id,
                    );

                    break;
                }

                // Check for view store
                if let Some(view_handler) = cx.views.get(&entity) {
                    if view_handler.as_any_ref().is::<L1::Source>() {
                        insert_store(
                            &ancestors,
                            &mut model_data_store.stores,
                            ModelOrView::View(view_handler.as_ref()),
                            self.0,
                            id,
                        );

                        break;
                    }
                }
            }
        }
    }
}

pub struct DualStore<L1: Lens, L2: Lens> {
    pub entity: Entity,
    pub lens1: L1,
    pub lens2: L2,
    pub old: Option<L1::Target>,
    pub observers: HashSet<Entity>,
}

impl<L1: Lens, L2: Lens> Store for DualStore<L1, L2>
where
    L1::Target: Data,
{
    fn entity(&self) -> Entity {
        self.entity
    }

    fn update(&mut self, model: ModelOrView) -> bool {
        if let Some(data) = model.downcast_ref::<L1::Source>() {
            let result = self.lens1.view(data, |t| match (&self.old, t) {
                (Some(a), Some(b)) if a.same(b) => None,
                (None, None) => None,
                _ => Some(t.cloned()),
            });
            if let Some(new_data) = result {
                self.old = new_data;
                return true;
            }
        }

        false
    }

    fn observers(&self) -> &HashSet<Entity> {
        &self.observers
    }

    fn add_observer(&mut self, observer: Entity) {
        self.observers.insert(observer);
    }

    fn remove_observer(&mut self, observer: &Entity) {
        self.observers.remove(observer);
    }

    fn num_observers(&self) -> usize {
        self.observers.len()
    }
}

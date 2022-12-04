use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
};

use vizia_storage::TreeExt;

use crate::{
    context::Context,
    entity::Entity,
    state::{LensCache, ModelOrView, StoreId},
};

use super::{BasicStore, Data, Lens, LensExt, Store};

pub trait Bindable {
    type Output;
    fn get_val(&self, cx: &Context) -> Self::Output;
    fn insert_store(self, cx: &mut Context, entity: Entity);
    fn name(&self) -> Option<&'static str>;
}

impl<L: Lens> Bindable for L
where
    L::Target: Data,
{
    type Output = L::Target;

    fn get_val(&self, cx: &Context) -> Self::Output {
        self.get(cx)
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

    fn get_val(&self, cx: &Context) -> Self::Output {
        (self.0.get(cx), self.1.get(cx))
    }

    fn insert_store(self, cx: &mut Context, id: Entity) {
        let ancestors = cx.current().parent_iter(&cx.tree).collect::<HashSet<_>>();
        let new_ancestors = id.parent_iter(&cx.tree).collect::<Vec<_>>();

        let data = &mut cx.data;
        let storages = &mut cx.stores;

        fn insert_store<L1: Lens, L2: Lens>(
            ancestors: &HashSet<Entity>,
            stores: &mut HashMap<StoreId, Box<dyn Store>>,
            model_data: ModelOrView,
            lens1: L1,
            lens2: L2,
            id: Entity,
        ) where
            L1::Target: Data,
            L2::Target: Data,
        {
            // let key = lens1.cache_key();

            // TODO: This won't work if one of the lenses returns a UUID StoreId. Instead this should be some combination of lens1.cache_key() and lens2.cache_key().
            let key = StoreId::Type(TypeId::of::<(L1, L2)>());

            if let Some(store) = stores.get_mut(&key) {
                let observers = store.observers();

                if ancestors.intersection(observers).next().is_none() {
                    store.add_observer(id);
                }
            } else {
                let mut observers = HashSet::new();
                observers.insert(id);

                let model = model_data.downcast_ref::<L1::Source>().unwrap();
                //let model2 = model_data.downcast_ref::<L2::Source>().unwrap();

                // TODO: Instead of setting old.1 to None, find a way to get the value for both lenses.
                let old = (lens1.view(model, |t| t.cloned()), None);

                let store = Box::new(DualStore { entity: id, lens1, lens2, old, observers });

                stores.insert(key, store);
            }
        }

        // Iterate up the tree and find the first matching model/view for either of the lenses
        for entity in new_ancestors {
            if let Some(models) = data.get_mut(entity) {
                // Check for model store for first lens
                if let Some(model_data) = models.get(&TypeId::of::<L1::Source>()) {
                    if !storages.contains(entity) {
                        storages.insert(entity, HashMap::new()).unwrap();
                    }

                    if let Some(stores) = storages.get_mut(entity) {
                        insert_store(
                            &ancestors,
                            stores,
                            ModelOrView::Model(model_data.as_ref()),
                            self.0,
                            self.1,
                            id,
                        );
                    }

                    break;
                }

                // Check for model store for second lens
                if let Some(model_data) = models.get(&TypeId::of::<L2::Source>()) {
                    if !storages.contains(entity) {
                        storages.insert(entity, HashMap::new()).unwrap();
                    }

                    if let Some(stores) = storages.get_mut(entity) {
                        insert_store(
                            &ancestors,
                            stores,
                            ModelOrView::Model(model_data.as_ref()),
                            self.0,
                            self.1,
                            id,
                        );
                    }

                    break;
                }

                // Check for view store for first lens
                if let Some(view_handler) = cx.views.get(&entity) {
                    if view_handler.as_any_ref().is::<L1::Source>() {
                        if !storages.contains(entity) {
                            storages.insert(entity, HashMap::new()).unwrap();
                        }

                        if let Some(stores) = storages.get_mut(entity) {
                            insert_store(
                                &ancestors,
                                stores,
                                ModelOrView::View(view_handler.as_ref()),
                                self.0,
                                self.1,
                                id,
                            );
                        }

                        break;
                    }
                }

                // Check for view store for second lens
                if let Some(view_handler) = cx.views.get(&entity) {
                    if view_handler.as_any_ref().is::<L2::Source>() {
                        if !storages.contains(entity) {
                            storages.insert(entity, HashMap::new()).unwrap();
                        }

                        if let Some(stores) = storages.get_mut(entity) {
                            insert_store(
                                &ancestors,
                                stores,
                                ModelOrView::View(view_handler.as_ref()),
                                self.0,
                                self.1,
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
        // TODO: It's not possible to concatenate two static str's so to return a combination of the stored names
        // we may need to change the signature of `name()` to return String or maybe CowStr.
        self.0.name()
    }
}

pub(crate) struct DualStore<L1: Lens, L2: Lens> {
    pub entity: Entity,
    pub lens1: L1,
    pub lens2: L2,
    pub old: (Option<L1::Target>, Option<L2::Target>),
    pub observers: HashSet<Entity>,
}

impl<L1: Lens, L2: Lens> Store for DualStore<L1, L2>
where
    L1::Target: Data,
    L2::Target: Data,
{
    fn entity(&self) -> Entity {
        self.entity
    }

    fn update(&mut self, model: ModelOrView) -> bool {
        if let Some(data) = model.downcast_ref::<L1::Source>() {
            let result = self.lens1.view(data, |t| match (&self.old.0, t) {
                (Some(a), Some(b)) if a.same(b) => None,
                (None, None) => None,
                _ => Some(t.cloned()),
            });
            if let Some(new_data) = result {
                self.old.0 = new_data;
                return true;
            }
        }

        if let Some(data) = model.downcast_ref::<L2::Source>() {
            let result = self.lens2.view(data, |t| match (&self.old.1, t) {
                (Some(a), Some(b)) if a.same(b) => None,
                (None, None) => None,
                _ => Some(t.cloned()),
            });
            if let Some(new_data) = result {
                self.old.1 = new_data;
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

use std::any::TypeId;
use std::collections::{HashMap, HashSet};

use crate::binding::{get_storeid, BasicStore, Store, StoreId};
use crate::context::{CURRENT, MAPS, MAP_MANAGER};
use crate::model::ModelOrView;
use crate::prelude::*;

/// A view with a binding which rebuilds its contents when the observed data changes.
///
/// This view is typically used to switch between two or more views when the bound data changes. The binding view will destroy and then recreate its
/// contents whenever the bound data changes, so it is usually preferable to bind a view directly to the data (if supported) or to bind to a view modifier,
/// which will update the properties of a view without rebuilding it.
pub struct Binding<L>
where
    L: Lens,
{
    entity: Entity,
    lens: L,
    #[allow(clippy::type_complexity)]
    content: Option<Box<dyn Fn(&mut Context, L)>>,
}

impl<L> Binding<L>
where
    L: 'static + Lens,
    <L as Lens>::Source: 'static,
    <L as Lens>::Target: Data,
{
    /// Creates a new binding view.
    ///
    /// A binding view observes application data through a lens and rebuilds its contents if the data changes.
    ///
    /// # Example
    /// When the value of `AppData::some_data` changes, the label inside of the binding will be rebuilt.
    /// ```ignore
    /// Binding::new(cx, AppData::some_data, |cx, lens|{
    ///     // Retrieve the data from context
    ///     let value = *lens.get(cx);
    ///     Label::new(cx, value.to_string());
    /// });
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new<F>(cx: &mut Context, lens: L, builder: F)
    where
        F: 'static + Fn(&mut Context, L),
    {
        let id = cx.entity_manager.create();
        let current = cx.current();
        cx.tree.add(id, current).expect("Failed to add to tree");
        cx.cache.add(id);
        cx.style.add(id);
        cx.tree.set_ignored(id, true);

        let binding = Self { entity: id, lens, content: Some(Box::new(builder)) };

        CURRENT.with(|f| *f.borrow_mut() = id);

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
            let key = get_storeid(&lens);

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

                let store = Box::new(BasicStore { lens, old, observers });

                stores.insert(key, store);
            }
        }

        // Check if there's already a store with the same lens somewhere up the tree. If there is, add this binding as an observer,
        // else create a new store with this binding as an observer.
        for entity in new_ancestors {
            if let Some(model_data_store) = cx.data.get_mut(&entity) {
                // Check for model store
                if let Some(model_data) = model_data_store.models.get(&TypeId::of::<L::Source>()) {
                    insert_store(
                        &ancestors,
                        &mut model_data_store.stores,
                        ModelOrView::Model(model_data.as_ref()),
                        lens,
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
                            lens,
                            id,
                        );

                        break;
                    }
                }
            }
        }

        cx.bindings.insert(id, Box::new(binding));

        cx.with_current(id, |cx| {
            // Call the body of the binding
            if let Some(mut binding) = cx.bindings.remove(&id) {
                binding.update(cx);
                cx.bindings.insert(id, binding);
            }
        });

        let _: Handle<Self> = Handle { entity: id, p: Default::default(), cx }.ignore();
    }
}

pub(crate) trait BindingHandler {
    fn update(&mut self, cx: &mut Context);
    fn remove(&self, cx: &mut Context);
    fn debug(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
}

impl<L: 'static + Lens> BindingHandler for Binding<L> {
    fn update(&mut self, cx: &mut Context) {
        cx.remove_children(cx.current());

        let ids = MAPS.with(|f| {
            let ids = f
                .borrow()
                .iter()
                .filter(|(_, map)| map.0 == self.entity)
                .map(|(id, _)| *id)
                .collect::<Vec<_>>();
            f.borrow_mut().retain(|_, map| map.0 != self.entity);

            ids
        });

        MAP_MANAGER.with(|f| {
            for id in ids {
                f.borrow_mut().destroy(id);
            }
        });

        if let Some(builder) = &self.content {
            CURRENT.with(|f| *f.borrow_mut() = self.entity);
            (builder)(cx, self.lens);
        }
    }

    fn remove(&self, cx: &mut Context) {
        for entity in self.entity.parent_iter(&cx.tree) {
            if let Some(model_data_store) = cx.data.get_mut(&entity) {
                let key = get_storeid(&self.lens);

                // Check for model store
                if model_data_store.models.get(&TypeId::of::<L::Source>()).is_some() {
                    if let Some(store) = model_data_store.stores.get_mut(&key) {
                        store.remove_observer(&self.entity);

                        if store.num_observers() == 0 {
                            model_data_store.stores.remove(&key);
                        }
                    }

                    break;
                }

                // Check for view store
                if let Some(view_handler) = cx.views.get(&entity) {
                    if view_handler.as_any_ref().is::<L::Source>() {
                        if let Some(store) = model_data_store.stores.get_mut(&key) {
                            store.remove_observer(&self.entity);

                            if store.num_observers() == 0 {
                                model_data_store.stores.remove(&key);
                            }
                        }

                        break;
                    }
                }
            }
        }
    }

    fn debug(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.lens.fmt(f)
    }
}

impl std::fmt::Debug for dyn BindingHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.debug(f)
    }
}

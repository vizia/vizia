use std::any::TypeId;
use std::collections::{HashMap, HashSet};

use crate::prelude::*;
use crate::state::{BasicStore, LensCache, ModelOrView, Store, StoreId};

/// A binding view which rebuilds its contents when its observed data changes.
///
/// This type is part of the prelude.
pub struct Binding<L>
where
    L: Lens,
{
    lens: L,
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
    pub fn new<F>(cx: &mut Context, lens: L, builder: F)
    where
        F: 'static + Fn(&mut Context, L),
    {
        let binding = Self { lens: lens.clone(), content: Some(Box::new(builder)) };

        let id = cx.entity_manager.create();
        let current = cx.current();
        cx.tree().add(id, current).expect("Failed to add to tree");
        cx.cache().add(id).expect("Failed to add to cache");
        cx.style().add(id);

        let ancestors = cx.current().parent_iter(cx.tree()).collect::<HashSet<_>>();
        let new_ancestors = id.parent_iter(cx.tree()).collect::<Vec<_>>();

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

        // Check if there's already a store with the same lens somewhere up the tree. If there is, add this binding as an observer,
        // else create a new store with this binding as an observer.
        for entity in new_ancestors {
            if let Some(model_data_store) = cx.data.get_mut(entity) {
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
                binding.body(cx);
                cx.bindings.insert(id, binding);
            }
        });

        let _: Handle<Self> = Handle { entity: id, p: Default::default(), cx }.ignore();
    }
}

pub trait BindingHandler {
    fn body<'a>(&mut self, cx: &'a mut Context);
}

impl<L: 'static + Lens> BindingHandler for Binding<L> {
    fn body<'a>(&mut self, cx: &'a mut Context) {
        cx.remove_children(cx.current());
        if let Some(builder) = &self.content {
            (builder)(cx, self.lens.clone());
        }
    }
}

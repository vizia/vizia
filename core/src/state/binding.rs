use std::any::TypeId;
use std::collections::HashSet;
use std::rc::Rc;

use crate::prelude::*;
use crate::state::BasicStore;

/// A binding view which rebuilds its contents when its observed data changes.
///
/// This type is part of the prelude.
pub struct Binding<F, L> {
    content: F,
    bindable: L,
}

impl<F: 'static + Fn(&mut Context, L), L: Bindable> Binding<F, L> {
    /// Creates a new binding view.
    ///
    /// A binding view observes application data through a lens and rebuilds its contents if the data changes.
    ///
    /// # Example
    /// When the value of `AppData::some_data` changes, the label inside of the binding will be rebuilt.
    /// ```ignore
    /// Binding::new(cx, AppData::some_data, |cx, lens| {
    ///     // Retrieve the data from context
    ///     let value = lens.get(cx);
    ///     Label::new(cx, value.to_string());
    /// });
    pub fn new(cx: &mut Context, bindable: L, builder: F) {
        let binding = Self { content: builder, bindable: bindable.clone() };
        let id = alloc_observer(cx, bindable);
        cx.views.insert(id, Box::new(binding));

        cx.with_current(id, |cx| {
            // Call the body of the binding
            if let Some(mut view_handler) = cx.views.remove(&id) {
                view_handler.body(cx);
                cx.views.insert(id, view_handler);
            }
        });

        let _: Handle<Self> = Handle { entity: id, p: Default::default(), cx }.ignore();
    }
}

impl<F: 'static + Fn(&mut Context, L), L: Bindable> View for Binding<F, L> {
    fn body(&mut self, cx: &mut Context) {
        cx.remove_children(cx.current());
        (self.content)(cx, self.bindable.clone());
    }
}

fn alloc_observer<L: Bindable>(cx: &mut Context, bindable: L) -> Entity {
    let current = cx.current();
    let id = cx.entity_manager.create();
    cx.tree().add(id, current).expect("Failed to add to tree");
    cx.cache().add(id).expect("Failed to add to cache");
    cx.style().add(id);

    let new_ancestors = id.parent_iter(cx.tree()).collect::<Vec<_>>();

    // for each model the bindable requests...
    for store_req in bindable.requests() {
        // walk the ancestry chain child-to-parent...
        for entity in new_ancestors.iter() {
            // key the data map on the anchor entity...
            if let Some(model_data_store) = cx.data.get_mut(*entity) {
                // and then the model type...
                let model_data =
                    if let Some(model_data) = model_data_store.data.get(&store_req.model_id) {
                        ModelOrView::Model(model_data.as_ref())
                    } else if let Some(view_data) = cx.views.get(entity).and_then(|view| {
                        ((view.as_any_ref()).type_id() == store_req.model_id).then(|| view)
                    }) {
                        ModelOrView::View(view_data.as_ref())
                    } else {
                        continue;
                    };

                // and finally the store type...
                // (but only if it's dedupable)
                let store = if store_req.can_deduplicate {
                    model_data_store.lenses_dedup.entry(store_req.store_id).or_insert_with(|| {
                        bindable.make_store(model_data).expect(
                            "Bug: bindable failed to produce a store. Probably an internal error.",
                        )
                    })
                } else {
                    let idx = model_data_store.lenses_dup.len();
                    model_data_store.lenses_dup.push(bindable.make_store(model_data).expect(
                        "Bug: bindable failed to produce a store. Probably an internal error.",
                    ));
                    model_data_store.lenses_dup.get_mut(idx).unwrap()
                };

                // ...so that we can register the observer with the appropriate store.
                bindable.add_to_store(store.as_mut(), id);
            }
        }
    }

    id
}

#[derive(Copy, Clone, Debug)]
pub struct StoreId {
    pub can_deduplicate: bool,
    pub store_id: TypeId,
    pub model_id: TypeId,
}

pub trait Bindable: 'static + Clone {
    type Output;
    fn view<D: DataContext, F: FnOnce(Option<&Self::Output>) -> T, T>(
        &self,
        cx: &D,
        viewer: F,
    ) -> T;
    fn get<D: DataContext>(&self, cx: &D) -> Option<Self::Output>
    where
        Self::Output: Clone,
    {
        self.view(cx, |v| v.cloned())
    }

    fn requests(&self) -> Vec<StoreId>;
    fn make_store(&self, source: ModelOrView) -> Option<Box<dyn StoreHandler>>;
    fn add_to_store(&self, store: &mut dyn StoreHandler, entity: Entity);
}

impl<L> Bindable for L
where
    L: Lens,
    <L as Lens>::Source: 'static,
    <L as Lens>::Target: Data,
{
    type Output = L::Target;

    fn view<D: DataContext, F: FnOnce(Option<&Self::Output>) -> T, T>(
        &self,
        cx: &D,
        viewer: F,
    ) -> T {
        self.view(
            cx.data().expect(
                "Lens failed to retrieve data from context. Did you forget to build a model?",
            ),
            viewer,
        )
    }

    fn requests(&self) -> Vec<StoreId> {
        vec![StoreId {
            store_id: TypeId::of::<BasicStore<Self, <Self as Lens>::Target>>(),
            model_id: TypeId::of::<<Self as Lens>::Source>(),
            can_deduplicate: std::mem::size_of::<Self>() == 0,
        }]
    }

    fn make_store(&self, source: ModelOrView) -> Option<Box<dyn StoreHandler>> {
        source.downcast_ref::<<Self as Lens>::Source>().map(|source| -> Box<dyn StoreHandler> {
            Box::new(BasicStore {
                lens: self.clone(),
                old: self.view(source, |t| t.cloned()),
                observers: HashSet::new(),
            })
        })
    }

    fn add_to_store(&self, store: &mut dyn StoreHandler, entity: Entity) {
        if let Some(store) = store.downcast_mut::<BasicStore<Self, <Self as Lens>::Target>>() {
            store.observers.insert(entity);
        }
    }
}

pub trait BindableExt: Bindable {
    fn map_shallow<G, B: 'static>(self, get: G) -> MapShallow<Self, Self::Output, B>
    where
        G: 'static + Fn(&Self::Output) -> B,
    {
        MapShallow { child: self, mapper: Rc::new(get) }
    }
}

impl<T: Bindable> BindableExt for T {}

pub struct MapShallow<L, I, O> {
    child: L,
    mapper: Rc<dyn Fn(&I) -> O>,
}

impl<L: Clone, I, O> Clone for MapShallow<L, I, O> {
    fn clone(&self) -> Self {
        Self { child: self.child.clone(), mapper: self.mapper.clone() }
    }
}

impl<L, I, O> Bindable for MapShallow<L, I, O>
where
    L: Bindable<Output = I>,
    I: 'static,
    O: 'static,
{
    type Output = O;

    fn view<D: DataContext, F: FnOnce(Option<&Self::Output>) -> T, T>(
        &self,
        cx: &D,
        viewer: F,
    ) -> T {
        self.child.view(cx, |inner| {
            let foo = if let Some(i) = inner { Some((self.mapper)(i)) } else { None };
            viewer(foo.as_ref())
        })
    }

    fn requests(&self) -> Vec<StoreId> {
        self.child.requests()
    }

    fn make_store(&self, source: ModelOrView) -> Option<Box<dyn StoreHandler>> {
        self.child.make_store(source)
    }

    fn add_to_store(&self, store: &mut dyn StoreHandler, entity: Entity) {
        self.child.add_to_store(store, entity);
    }
}

#[derive(Clone, Debug)]
pub struct ConstantBindable<T> {
    value: T,
}

impl<T> ConstantBindable<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Clone> Bindable for ConstantBindable<T> {
    type Output = T;

    fn view<D: DataContext, F: FnOnce(Option<&Self::Output>) -> T, T>(
        &self,
        _cx: &D,
        viewer: F,
    ) -> T {
        viewer(Some(&self.value))
    }

    fn requests(&self) -> Vec<StoreId> {
        vec![]
    }

    fn make_store(&self, _source: ModelOrView) -> Option<Box<dyn StoreHandler>> {
        panic!("You should never call this function");
    }

    fn add_to_store(&self, _store: &mut dyn StoreHandler, _entity: Entity) {
        panic!("You should never call this function");
    }
}

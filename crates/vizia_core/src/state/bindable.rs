use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    marker::PhantomData,
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
    fn view<'a, C: DataContext, O, F: FnOnce(Option<&Self::Output>) -> O>(
        &self,
        cx: &'a C,
        map: F,
    ) -> O;
    fn get<C: DataContext>(&self, cx: &C) -> Self::Output;
    fn get_fallible<C: DataContext>(&self, cx: &C) -> Option<Self::Output>;
    fn insert_store(self, cx: &mut Context, entity: Entity);
    fn name(&self) -> Option<&'static str>;
}

impl<L: Lens> Bindable for L
where
    L::Target: Data,
{
    type Output = L::Target;

    fn view<'a, C: DataContext, O, F: FnOnce(Option<&Self::Output>) -> O>(
        &self,
        cx: &'a C,
        map: F,
    ) -> O {
        // Not sure why there's an error here. The lifetime of `t` should be tied to the lifetime of `cx` which is `'a`.
        (map)(self.view(
            cx.data().expect("Failed to get data from context. Has it been built into the tree?"),
            |t| t,
        ))
    }

    fn get<C: DataContext>(&self, cx: &C) -> Self::Output {
        self.view(
            cx.data().expect("Failed to get data from context. Has it been built into the tree?"),
            |t| {
                t.expect("Lens failed to resolve. Do you want to use LensExt::get_fallible?")
                    .clone()
            },
        )
    }

    fn get_fallible<C: DataContext>(&self, cx: &C) -> Option<Self::Output> {
        self.view(
            cx.data().expect("Failed to get data from context. Has it been built into the tree?"),
            |t| t.cloned().map(|v| v),
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

    fn view<'a, C: DataContext, O, F: FnOnce(Option<&Self::Output>) -> O>(
        &self,
        cx: &'a C,
        map: F,
    ) -> O {
        todo!()
        // (map)(Some(&(
        //     self.0
        //         .view(
        //             cx.data().expect(
        //                 "Failed to get data from context. Has it been built into the tree?",
        //             ),
        //             |t| t.unwrap(),
        //         )
        //         .clone(),
        //     self.1
        //         .view(
        //             cx.data().expect(
        //                 "Failed to get data from context. Has it been built into the tree?",
        //             ),
        //             |t| t.unwrap(),
        //         )
        //         .clone(),
        // )))
    }

    fn get<C: DataContext>(&self, cx: &C) -> Self::Output {
        (self.0.get(cx), self.1.get(cx))
    }

    fn get_fallible<C: DataContext>(&self, cx: &C) -> Option<Self::Output> {
        Some((self.0.get_fallible(cx)?, self.1.get_fallible(cx)?))
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

#[derive(Clone)]
pub struct BindMap<B, F, T> {
    b: B,
    map: F,
    p: PhantomData<T>,
}

impl<B, F, T> BindMap<B, F, T> {
    pub fn new(b: B, map: F) -> Self
    where
        B: Bindable + Clone,
        F: Fn(&B::Output) -> T,
    {
        Self { b, map, p: PhantomData::default() }
    }
}

impl<B: Copy, F: Copy, T: Clone> Copy for BindMap<B, F, T> {}

impl<B, F, T: Clone + 'static> Bindable for BindMap<B, F, T>
where
    B: 'static + Bindable + Clone,
    F: 'static + Clone + Fn(&B::Output) -> T,
{
    type Output = T;

    fn view<'a, C: DataContext, O, G: FnOnce(Option<&Self::Output>) -> O>(
        &self,
        cx: &'a C,
        map: G,
    ) -> O {
        (map)(Some(&(self.map)(self.b.view(cx, |t| t.expect("Failed")))))
    }

    fn get<C: DataContext>(&self, cx: &C) -> Self::Output {
        (self.map)(&self.b.get(cx))
    }

    fn get_fallible<C: DataContext>(&self, cx: &C) -> Option<Self::Output> {
        Some((self.map)(&self.b.get_fallible(cx)?))
    }

    fn insert_store(self, cx: &mut Context, entity: Entity) {
        self.b.insert_store(cx, entity);
    }

    fn name(&self) -> Option<&'static str> {
        self.b.name()
    }
}

#[derive(Clone)]
pub struct BindIndex<B, T> {
    b: B,
    index: usize,
    p: PhantomData<T>,
}

impl<B, T> BindIndex<B, T> {
    pub fn new(b: B, index: usize) -> Self {
        Self { b, index, p: PhantomData::default() }
    }
}

impl<B: Copy, T: Clone> Copy for BindIndex<B, T> {}

impl<B, T: 'static> Bindable for BindIndex<B, T>
where
    B: Bindable,
    <B as Bindable>::Output: std::ops::Deref<Target = [T]>,
    T: Clone,
{
    type Output = T;
    fn view<'a, C: DataContext, O, F: FnOnce(Option<&Self::Output>) -> O>(
        &self,
        cx: &'a C,
        map: F,
    ) -> O {
        (map)(self.b.view(cx, |t| t.expect("Failed").get(self.index)))
    }
    fn get<C: DataContext>(&self, cx: &C) -> Self::Output {
        if let Some(val) = self.b.get(cx).get(self.index) {
            val.clone()
        } else {
            panic!("Failed");
        }
    }

    fn get_fallible<C: DataContext>(&self, cx: &C) -> Option<Self::Output> {
        if let Some(val) = self.b.get_fallible(cx)?.get(self.index) {
            Some(val.clone())
        } else {
            panic!("Failed");
        }
    }

    fn insert_store(self, cx: &mut Context, entity: Entity) {
        self.b.insert_store(cx, entity);
    }

    fn name(&self) -> Option<&'static str> {
        self.b.name()
    }
}

#[derive(Clone)]
pub struct BindThen<B, L, T> {
    b: B,
    l: L,
    p: PhantomData<T>,
}

impl<B: Copy, L: Copy, T: Clone> Copy for BindThen<B, L, T> {}

impl<B, L, T> BindThen<B, L, T> {
    pub fn new(b: B, l: L) -> Self {
        Self { b, l, p: PhantomData::default() }
    }
}

// impl<B: Copy, T: Clone> Copy for BindIndex<B, T> {}

impl<B, L, T: 'static> Bindable for BindThen<B, L, T>
where
    B: Bindable,
    L: Lens<Source = B::Output, Target = T>,
    T: Clone,
{
    type Output = L::Target;

    fn view<'a, C: DataContext, O, F: FnOnce(Option<&Self::Output>) -> O>(
        &self,
        cx: &'a C,
        map: F,
    ) -> O {
        (map)(self.l.view(self.b.view(cx, |t| t.expect("Failed")), |t| t))
    }

    fn get<C: DataContext>(&self, cx: &C) -> Self::Output {
        self.l.view(&self.b.get(cx), |t| t.cloned().expect("Failed")).clone()
    }

    fn get_fallible<C: DataContext>(&self, cx: &C) -> Option<Self::Output> {
        Some(self.l.view(&self.b.get_fallible(cx)?, |t| t.cloned().expect("Failed")).clone())
    }

    fn insert_store(self, cx: &mut Context, entity: Entity) {
        self.b.insert_store(cx, entity);
    }

    fn name(&self) -> Option<&'static str> {
        self.b.name()
    }
}
// pub trait BindableExt: Clone {
//     type Map<S, F, T>;
//     type Output;
//     fn map<F, T: 'static>(self, f: F) -> Self::Map<Self, F, T>
//     where
//         F: 'static + Clone + Fn(&Self::Output) -> T;
// }

impl<B1, L1, T1, B2, L2, T2> Bindable for (BindThen<B1, L1, T1>, BindThen<B2, L2, T2>)
where
    B1: Bindable,
    B2: Bindable,
    L1: Lens<Source = B1::Output, Target = T1>,
    L2: Lens<Source = B2::Output, Target = T2>,
    T1: 'static + Clone,
    T2: 'static + Clone,
{
    type Output = (L1::Target, L2::Target);

    fn view<'a, C: DataContext, O, F: FnOnce(Option<&Self::Output>) -> O>(
        &self,
        cx: &'a C,
        map: F,
    ) -> O {
        todo!()
        // (map)(Some(&(
        //     self.0.view(cx, |t| t.expect("Failed")).clone(),
        //     self.1.view(cx, |t| t.expect("Failed")).clone(),
        // )))
    }

    fn get<C: DataContext>(&self, cx: &C) -> Self::Output {
        (self.0.get(cx), self.1.get(cx))
    }

    fn get_fallible<C: DataContext>(&self, cx: &C) -> Option<Self::Output> {
        Some((self.0.get_fallible(cx)?, self.1.get_fallible(cx)?))
    }

    fn insert_store(self, cx: &mut Context, entity: Entity) {
        self.0.insert_store(cx, entity);
        self.1.insert_store(cx, entity);
    }

    fn name(&self) -> Option<&'static str> {
        todo!()
    }
}

pub trait BindableExt: Bindable + Clone {
    fn map<F, T: 'static>(self, f: F) -> BindMap<Self, F, T>
    where
        F: 'static + Clone + Fn(&Self::Output) -> T,
    {
        BindMap::new(self, f)
    }

    fn index<T: 'static>(self, index: usize) -> BindIndex<Self, T> {
        BindIndex::new(self, index)
    }

    fn then<L: Lens, T: 'static>(self, lens: L) -> BindThen<Self, L, T> {
        BindThen::new(self, lens)
    }
}

impl<B: Bindable + Clone> BindableExt for B {}

use std::collections::HashSet;

use crate::{Context, Data, Entity, Lens, ModelData};

pub struct Store<T> {
    pub data: T,
    pub observers: HashSet<Entity>,
    pub dirty: bool,
}

impl<T> Store<T> {
    pub fn new(data: T) -> Self {
        Self { data, observers: HashSet::new(), dirty: false }
    }

    pub fn insert_observer(&mut self, entity: Entity) {
        self.observers.insert(entity);
    }

    pub fn remove_observer(&mut self, entity: Entity) {
        self.observers.remove(&entity);
    }

    // pub fn needs_update(&mut self) {
    //     self.dirty = true;
    // }

    pub fn update_observers(&mut self, cx: &mut Context) {
        if self.dirty {
            for observer in self.observers.iter() {
                if let Some(mut view) = cx.views.remove(observer) {
                    let prev = cx.current;
                    cx.current = *observer;
                    view.body(cx);
                    cx.current = prev;

                    cx.views.insert(*observer, view);
                }
            }

            self.dirty = false;
        }
    }
}

// impl<T: 'static> View for Store<T> {
//     fn event(&mut self, cx: &mut Context, event: &mut Event) {
//         self.update(cx, event);
//     }
// }

pub trait LensWrap {
    fn update(&mut self, model: &Box<dyn ModelData>) -> bool;
    fn observers(&self) -> &HashSet<Entity>;
    fn add_observer(&mut self, observer: Entity);
    fn entity(&self) -> Entity;
}

pub struct StateStore<L: Lens, T> {
    pub entity: Entity,
    pub lens: L,
    pub old: T,
    pub observers: HashSet<Entity>,
}

impl<L: Lens, T> LensWrap for StateStore<L, T>
where
    L: Lens<Target = T>,
    <L as Lens>::Target: Data,
{
    fn entity(&self) -> Entity {
        self.entity
    }

    fn update(&mut self, model: &Box<dyn ModelData>) -> bool {
        if let Some(store) = model.downcast_ref::<Store<L::Source>>() {
            let state = self.lens.view(&store.data);
            if !state.same(&self.old) {
                self.old = state.clone();
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
}

use std::collections::HashSet;

use crate::{Data, Entity, Lens, ModelData};

pub trait LensWrap {
    fn update(&mut self, model: &Box<dyn ModelData>) -> bool;
    fn observers(&self) -> &HashSet<Entity>;
    fn add_observer(&mut self, observer: Entity);
    fn entity(&self) -> Entity;
}

pub struct StateStore<L: Lens, T> {
    // The entity which declared the binding
    pub entity: Entity,
    pub lens: L,
    pub old: Option<T>,
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
        if let Some(data) = model.downcast_ref::<L::Source>() {
            let state = self.lens.view(data);
            match (state, &self.old) {
                (None, None) => false,
                (Some(a), Some(b)) if a.same(b) => false,
                _ => {
                    self.old = state.cloned();
                    true
                }
            }
        } else {
            false
        }
    }

    fn observers(&self) -> &HashSet<Entity> {
        &self.observers
    }

    fn add_observer(&mut self, observer: Entity) {
        self.observers.insert(observer);
    }
}

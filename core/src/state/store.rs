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
        if let Some(data) = model.downcast_ref::<L::Source>() {
            let result = self.lens.view(data, |t| {
                if t.same(&self.old) {
                    None
                } else {
                    Some(t.clone())
                }
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
}

use std::{any::TypeId, collections::HashSet};

use crate::prelude::*;

use super::ModelOrView;

use std::sync::atomic::{AtomicU64, Ordering};

// Generates a unique ID
pub(crate) fn next_uuid() -> u64 {
    static UUID: AtomicU64 = AtomicU64::new(0);
    UUID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum StoreId {
    Type(TypeId),
    UUID(u64),
}

pub(crate) trait Store {
    fn update(&mut self, model: ModelOrView) -> bool;
    fn observers(&self) -> &HashSet<Entity>;
    fn add_observer(&mut self, observer: Entity);
    fn remove_observer(&mut self, observer: &Entity);
    fn num_observers(&self) -> usize;
    fn entity(&self) -> Entity;
}

pub(crate) struct BasicStore<L: Lens, T> {
    // The entity which declared the binding
    pub entity: Entity,
    pub lens: L,
    pub old: Option<T>,
    pub observers: HashSet<Entity>,
}

impl<L: Lens, T> Store for BasicStore<L, T>
where
    L: Lens<Target = T>,
    <L as Lens>::Target: Data,
{
    fn entity(&self) -> Entity {
        self.entity
    }

    fn update(&mut self, model: ModelOrView) -> bool {
        if let Some(data) = model.downcast_ref::<L::Source>() {
            let result = self.lens.view(data, |t| match (&self.old, t) {
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

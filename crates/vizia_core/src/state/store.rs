use std::{any::TypeId, borrow::Borrow, collections::HashSet};

use crate::prelude::*;

use super::ModelOrView;

use std::sync::atomic::{AtomicU64, Ordering};

// Generates a unique ID
pub fn next_uuid() -> u64 {
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

// TODO skye: what's the role of owned vs borrowed from the perspective of a store?
// TODO skye: should the models downcast to Source or SourceOwned?
impl<L: Lens, T> Store for BasicStore<L, T>
where
    L: Lens<TargetOwned = T>,
    <L as Lens>::TargetOwned: Data,
    <L as Lens>::Target: Data,
    T: Borrow<L::Target>,
    L::Target: ToOwned<Owned=L::TargetOwned>,
{
    fn entity(&self) -> Entity {
        self.entity
    }

    fn update(&mut self, model: ModelOrView) -> bool {
        let Some(data) = model.downcast_ref::<L::SourceOwned>() else {
            return false
        };
        let Some(new_data) = self.lens.view(LensValue::Borrowed(data.borrow())) else { return false };

        if matches!(&self.old, Some(old) if old.borrow().same(new_data.as_ref().borrow())) {
            return false;
        }

        self.old = Some(new_data.into_owned());

        true
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

use hashbrown::HashSet;
use std::any::TypeId;
use std::hash::Hash;

use crate::{model::ModelOrView, prelude::*};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum StoreId {
    Source(u64),
    Map(u64),
}

pub(crate) trait Store {
    /// Updates the model data, returning true if the data changed.
    fn update(&mut self, model: ModelOrView) -> bool;
    /// Returns the set of observers for the store.
    fn observers(&self) -> &HashSet<Entity>;
    /// Adds an observer to the store.
    fn add_observer(&mut self, observer: Entity);
    /// Removes an observer from the store.
    fn remove_observer(&mut self, observer: &Entity);
    /// Returns the number of obersers for the store.
    fn num_observers(&self) -> usize;
    /// Returns true if the model or view is the source of the store.
    fn source(&self) -> TypeId;

    #[cfg(debug_assertions)]
    fn name(&self) -> String;
}

pub(crate) struct BasicStore<L: Lens, T> {
    pub lens: L,
    pub old: Option<T>,
    pub observers: HashSet<Entity>,
}

impl<L> Store for BasicStore<L, L::Target>
where
    L: Lens<Target: Data>,
{
    fn source(&self) -> TypeId {
        TypeId::of::<L::Source>()
    }

    fn update(&mut self, model: ModelOrView) -> bool {
        let Some(data) = model.downcast_ref::<L::Source>() else { return false };
        let Some(new_data) = self.lens.view(data) else { return false };

        if matches!(&self.old, Some(old) if old.same(&new_data)) {
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

    #[cfg(debug_assertions)]
    fn name(&self) -> String {
        format!("{:?}", self.lens)
    }
}

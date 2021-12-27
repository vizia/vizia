use std::{
    collections::HashSet,
};

use crate::{Data, Entity, Lens, ModelData, Store};

// Bindings take a lens
// Use lens as a key to some database of observers and add the binding to the set
// During update, iterate the lenses and check if the data has changed
// If it has changed then update the observers

// pub struct State<T>(T,usize);

// impl<T> Default for State<T>
//     where T: Default
// {
//     /// Constructs new [`Versioned<T>`](struct.Versioned.html) wrapper
//     /// containing default value for type `T`
//     /// and version set to [`INITIAL_VERSION`](constant.INITIAL_VERSION.html).
//     fn default() -> Self {
//         Self::new(T::default())
//     }
// }

// impl<T> Clone for State<T>
//     where T: Clone
// {
//     /// Clones [`Versioned<T>`](struct.Versioned.html).
//     /// The clone has its version set to [`INITIAL_VERSION`](constant.INITIAL_VERSION.html).
//     fn clone(&self) -> Self {
//         Self::new(self.0.clone())
//     }
// }

// impl<T> Copy for State<T>
//     where T: Copy
// {
//     // Empty
// }

// impl<T> Deref for State<T> {
//     type Target = T;

//     /// Dereferences the value. Does not increment version.
//     #[must_use]
//     fn deref(&self) -> &Self::Target {
//         self.as_ref_impl()
//     }
// }

// impl<T> DerefMut for State<T> {
//     /// Mutably dereferences the value. Increments version.
//     #[must_use = "mutation will be counted even if mutable dereference result is not actually used"]
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self.as_mut_impl()
//     }
// }

// impl<T> AsRef<T> for State<T> {
//     /// Returns reference to the value. Does not increment version.
//     #[must_use]
//     fn as_ref(&self) -> &T {
//         self.as_ref_impl()
//     }
// }

// impl<T> AsMut<T> for State<T> {
//     /// Returns mutable reference to the value. Increments version.
//     #[must_use = "mutation will be counted even if mutable reference returned from as_mut() is not actually used"]
//     fn as_mut(&mut self) -> &mut T {
//         self.as_mut_impl()
//     }
// }

// impl<T> State<T> {
//     //
//     // Interface
//     //

//     /// Constructs new [`Versioned<T>`](struct.Versioned.html) wrapper
//     /// with version set to [`INITIAL_VERSION`](constant.INITIAL_VERSION.html).
//     pub fn new(value: T) -> Self {
//         Self::with_version(value, 0)
//     }

//     /// Constructs new [`Versioned<T>`](struct.Versioned.html) wrapper
//     /// with the given version.
//     pub fn with_version(value: T, version: usize) -> Self {
//         Self(value, version)
//     }

//     /// Returns current version.
//     pub fn version(&self) -> usize {
//         self.1
//     }

//     //
//     // Service
//     //

//     fn as_ref_impl(&self) -> &T {
//         &self.0
//     }

//     fn as_mut_impl(&mut self) -> &mut T {
//         self.1 += 1;

//         &mut self.0
//     }
// }

// impl<T> State<T>
//     where T: Default
// {
//     /// Constructs new [`Versioned<T>`](struct.Versioned.html) wrapper
//     /// containing default value for type `T`
//     /// and the given version.
//     pub fn default_with_version(version: usize) -> Self {
//         Self::with_version(T::default(), version)
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

use crate::binding::{Data, Res, ResGet, SignalBinding};
use crate::context::EventContext;
use crate::entity::Entity;
use crate::prelude::ToStringLocalized;

use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::marker::PhantomData;

// Unique identifier for state nodes
#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
pub struct NodeId(u64);

impl NodeId {
    pub fn new(id: u64) -> Self {
        NodeId(id)
    }
}
// Separate tracking context with interior mutability
struct TrackingContext {
    // Current selector being tracked
    current_selector: Option<NodeId>,
    // Collected dependencies for the current selector
    dependencies: HashSet<NodeId>,
}

// RAII-based dependency tracker
pub struct DependencyTracker<'a> {
    store: &'a Store,
    prev_selector: Option<NodeId>,
}

impl<'a> DependencyTracker<'a> {
    pub fn new(store: &'a Store, selector_id: NodeId) -> Self {
        let prev_selector = store.begin_tracking(selector_id);
        Self { store, prev_selector }
    }

    pub fn dependencies(self) -> HashSet<NodeId> {
        self.store.end_tracking(self.prev_selector)
    }
}

impl<'a> Drop for DependencyTracker<'a> {
    fn drop(&mut self) {
        self.store.end_tracking(self.prev_selector);
    }
}

// Store with minimal interior mutability
pub struct Store {
    // Core state with regular mutability
    values: HashMap<NodeId, Box<dyn Any>>,

    // Maps entities to their signals
    entity_signals: HashMap<Entity, HashSet<NodeId>>,

    dependencies: HashMap<NodeId, HashSet<NodeId>>,
    dependents: HashMap<NodeId, HashSet<NodeId>>,

    // Observers for each node - maps NodeId to a set of entities observing it
    pub observers: HashMap<NodeId, HashSet<Entity>>,
    pub node_needs_update: HashMap<NodeId, bool>,

    // Compute functions for selectors
    compute_fns: HashMap<NodeId, Box<dyn Fn(&Store) -> Box<dyn Any>>>,

    // Only place where interior mutability is truly needed
    tracking: RefCell<TrackingContext>,

    id_counter: usize,
}

impl Store {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
            entity_signals: HashMap::new(),
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
            observers: HashMap::new(),
            node_needs_update: HashMap::new(),
            compute_fns: HashMap::new(),
            tracking: RefCell::new(TrackingContext {
                current_selector: None,
                dependencies: HashSet::new(),
            }),
            id_counter: 0,
        }
    }

    // Get a unique ID for the next node
    fn get_next_id(&mut self) -> NodeId {
        let id = NodeId::new(self.id_counter as u64);
        self.id_counter += 1;
        id
    }

    // Begin tracking dependencies - only modifies the tracking context
    fn begin_tracking(&self, selector_id: NodeId) -> Option<NodeId> {
        let mut tracking = self.tracking.borrow_mut();
        let prev = tracking.current_selector;
        tracking.current_selector = Some(selector_id);
        tracking.dependencies.clear();
        prev
    }

    // End tracking and return collected dependencies - only accesses tracking context
    fn end_tracking(&self, prev: Option<NodeId>) -> HashSet<NodeId> {
        let mut tracking = self.tracking.borrow_mut();
        let deps = std::mem::take(&mut tracking.dependencies);
        tracking.current_selector = prev;
        deps
    }

    // Record dependency in tracking context - only modifies tracking context
    fn record_dependency(&self, node_id: NodeId) {
        let mut tracking = self.tracking.borrow_mut();
        if tracking.current_selector.is_some() {
            tracking.dependencies.insert(node_id);
        }
    }

    // Combined get function that handles both atoms and selectors
    fn get<'a, T: 'static>(&'a self, id: &NodeId) -> Option<&'a T> {
        // Record dependency if we're in a tracking context
        self.record_dependency(*id);

        // Get the value from the store
        self.values.get(id).and_then(|boxed| boxed.downcast_ref::<T>())
    }

    fn get_mut<'a, T: 'static>(&'a mut self, id: &NodeId) -> Option<&'a mut T> {
        // Record dependency if we're in a tracking context
        self.record_dependency(*id);

        // Get the value from the store
        self.values.get_mut(id).and_then(|boxed| boxed.downcast_mut::<T>())
    }

    // Updated set method to detect batch mode
    fn set<T: 'static>(&mut self, id: &NodeId, value: T) {
        // Update the value
        self.values.insert(*id, Box::new(value));

        self.update_dependents(id);
    }

    // Register a dependency - mutates state
    fn register_dependency(&mut self, selector_id: &NodeId, dependency_id: &NodeId) {
        // Record that selector depends on dependency
        self.dependencies.entry(*selector_id).or_insert_with(HashSet::new).insert(*dependency_id);

        // Record that dependency has selector as dependent
        self.dependents.entry(*dependency_id).or_insert_with(HashSet::new).insert(*selector_id);
    }

    // Fix update_dependents to handle both selectors and effects
    fn update_dependents(&mut self, id: &NodeId) {
        self.node_needs_update.insert(*id, true);
        println!("Updating dependents for ID: {:?}", id);
        let initial_capacity = self.dependents.get(id).map_or(0, |deps| deps.len());
        let mut queue = Vec::with_capacity(initial_capacity);
        let mut visited = HashSet::with_capacity(initial_capacity * 2);

        // Start with direct dependents
        if let Some(deps) = self.dependents.get(id) {
            for dep_id in deps {
                queue.push(*dep_id);
                visited.insert(*dep_id);
            }
        }

        // Process queue iteratively
        while let Some(dependent_id) = queue.pop() {
            // It's a selector - recompute it as before
            self.recompute_selector(&dependent_id);

            // Add subsequent dependents
            if let Some(next_deps) = self.dependents.get(&dependent_id) {
                for next_dep in next_deps {
                    if !visited.contains(next_dep) {
                        queue.push(*next_dep);
                        visited.insert(*next_dep);
                    }
                }
            }
        }
    }

    // Simplify dependency update logic
    fn recompute_selector(&mut self, id: &NodeId) {
        self.node_needs_update.insert(*id, true);
        if let Some(compute_fn) = &self.compute_fns.get(id) {
            let old_deps =
                self.dependencies.get(id).map(|deps| deps.clone()).unwrap_or_else(HashSet::new);

            let tracker = DependencyTracker::new(self, *id);
            let result_any = compute_fn(self);
            let new_deps = tracker.dependencies();

            // Replace complex dependency comparison with a single operation
            if old_deps != new_deps {
                // Update dependencies
                for dep_id in old_deps.difference(&new_deps) {
                    if let Some(deps_set) = self.dependents.get_mut(dep_id) {
                        deps_set.remove(id);
                    }
                }

                for dep_id in new_deps.difference(&old_deps) {
                    self.register_dependency(id, dep_id);
                }

                // Update the dependencies map
                if new_deps.is_empty() {
                    self.dependencies.remove(id);
                } else {
                    self.dependencies.insert(*id, new_deps);
                }
            }

            self.values.insert(*id, result_any);
        }
    }

    // Clear dependencies - mutates state
    fn clear_dependencies(&mut self, selector_id: &NodeId) {
        // First gather dependencies to remove with pre-allocated capacity
        let deps_to_remove = if let Some(deps) = self.dependencies.get(selector_id) {
            // Pre-allocate a vector with the exact capacity needed
            let deps_count = deps.len();
            let mut deps_vec = Vec::with_capacity(deps_count);
            deps_vec.extend(deps.iter().copied());
            deps_vec
        } else {
            Vec::new()
        };

        // Remove this selector from dependents lists
        for dep_id in deps_to_remove {
            if let Some(deps_set) = self.dependents.get_mut(&dep_id) {
                deps_set.remove(selector_id);

                // Clean up empty dependents sets
                if deps_set.is_empty() {
                    self.dependents.remove(&dep_id);
                }
            }
        }

        // Clear the dependencies for this selector
        self.dependencies.remove(selector_id);
    }

    // Register a compute function - mutates state
    fn register_compute_fn<T, F>(&mut self, id: NodeId, compute_fn: F)
    where
        T: 'static,
        F: Fn(&Store) -> T + 'static,
    {
        let boxed_fn =
            Box::new(move |store: &Store| -> Box<dyn Any> { Box::new(compute_fn(store)) });

        self.compute_fns.insert(id, boxed_fn);
    }

    fn remove_signal(&mut self, id: &NodeId) {
        // Remove the signal value
        self.values.remove(id);

        // Clean up dependencies
        self.clear_dependencies(id);

        // Remove compute function if it exists
        self.compute_fns.remove(id);

        // Remove from node_needs_update
        self.node_needs_update.remove(id);

        // Clean up observers for this signal
        self.observers.remove(id);

        // Remove this signal from any dependents lists
        self.dependents.remove(id);
    }

    pub fn entity_destroyed(&mut self, entity: Entity) {
        // Remove all signals associated with this entity
        if let Some(signal_ids) = self.entity_signals.remove(&entity) {
            for id in signal_ids {
                self.remove_signal(&id);
            }
        }
    }
}

// Signal represents a piece of state
pub struct Signal<T: 'static> {
    id: NodeId,
    ty: PhantomData<T>,
}

impl<T: 'static> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self { id: self.id, ty: PhantomData }
    }
}

impl<T: 'static> Copy for Signal<T> {}

impl<T: 'static> Signal<T> {
    fn new(store: &mut Store, entity: Entity, default: T) -> Self {
        let id = store.get_next_id();
        store.entity_signals.entry(entity).or_default().insert(id);

        store.set(&id, default);

        Self { id, ty: PhantomData::default() }
    }

    fn derived(store: &mut Store, entity: Entity, compute: impl 'static + Fn(&Store) -> T) -> Self {
        let id = store.get_next_id();
        store.entity_signals.entry(entity).or_default().insert(id);

        store.register_compute_fn(id, compute);

        // Compute initial value
        store.recompute_selector(&id);

        Self { id, ty: PhantomData }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    // Read-only operation, but records dependency
    pub fn get<'a>(&self, store: &'a Store) -> &'a T {
        store.get::<T>(&self.id).unwrap()
    }

    fn get_mut<'a>(&self, store: &'a mut Store) -> &'a mut T {
        store.get_mut::<T>(&self.id).unwrap()
    }

    // Mutations require mutable store access
    pub fn set(&self, store: &mut Store, value: T) {
        store.set(&self.id, value);
    }

    // Update takes a function that works with references
    pub fn update<F: FnOnce(&mut T)>(&self, store: &mut EventContext, updater: F) {
        let old_value = self.get_mut(store.data.get_store_mut());
        updater(old_value);
        store.data.get_store_mut().update_dependents(&self.id);
    }

    pub fn observe(&self, store: &mut Store, entity: Entity) {
        store.observers.entry(self.id).or_default().insert(entity);
    }
}

// Root container
pub struct RecoilRoot {
    store: Store,
}

impl RecoilRoot {
    pub fn new() -> Self {
        Self { store: Store::new() }
    }

    pub fn state<T: 'static>(&mut self, entity: Entity, default: T) -> Signal<T> {
        Signal::new(&mut self.store, entity, default)
    }

    pub fn derived<T, F>(&mut self, entity: Entity, compute: F) -> Signal<T>
    where
        T: 'static,
        F: Fn(&Store) -> T + 'static,
    {
        Signal::derived(&mut self.store, entity, compute)
    }

    // Immutable store access for reading
    pub fn get_store(&self) -> &Store {
        &self.store
    }

    // Mutable store access for updates
    pub fn get_store_mut(&mut self) -> &mut Store {
        &mut self.store
    }
}

impl<T: Clone + ToStringLocalized> ToStringLocalized for Signal<T> {
    fn to_string_local(&self, cx: &impl crate::prelude::DataContext) -> String {
        if let Some(lc) = cx.localization_context() {
            return self.get(lc.data.get_store()).to_string_local(cx);
        }

        String::new()
    }
}

impl<T: Clone> ResGet<T> for Signal<T> {
    fn get_ref<'a>(
        &'a self,
        cx: &'a impl crate::prelude::DataContext,
    ) -> Option<crate::prelude::LensValue<'a, T>> {
        if let Some(lc) = cx.localization_context() {
            return Some(crate::binding::LensValue::Borrowed(self.get(lc.data.get_store())));
        }

        panic!("No localization context available for Signal.");
    }

    fn get(&self, cx: &impl crate::prelude::DataContext) -> T {
        self.get_ref(cx).unwrap().into_owned()
    }
}

impl<T: Clone> Res<T> for Signal<T> {
    fn set_or_bind<F>(
        self,
        cx: &mut crate::prelude::Context,
        entity: crate::prelude::Entity,
        closure: F,
    ) where
        Self: Sized,
        F: 'static + Fn(&mut crate::prelude::Context, Self),
    {
        println!("Setting or binding signal: {} {:?}", entity, self.id);

        SignalBinding::new(cx, self.clone(), move |cx| {
            cx.with_current(entity, |cx| {
                closure(cx, self.clone());
            });
        });
    }
}

impl<T: Clone> Data for Signal<T> {
    fn same(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

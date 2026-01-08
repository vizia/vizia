mod async_state;
mod persistence;
mod undo;

pub use async_state::{
    Async, AsyncCompletionEvent, AsyncHandle, AsyncOptions, AsyncSignalExt,
};
pub(crate) use async_state::run_async_load;
pub use persistence::{PersistenceError, PersistenceManager};
pub use undo::{HistoryEntry, SignalChange, UndoEntry, UndoManager};
use undo::SignalSnapshot;

use crate::binding::Data;
use crate::context::{Context, DataContext, EventContext};
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
    finished: bool,
}

impl<'a> DependencyTracker<'a> {
    pub fn new(store: &'a Store, selector_id: NodeId) -> Self {
        let prev_selector = store.begin_tracking(selector_id);
        Self { store, prev_selector, finished: false }
    }

    pub fn dependencies(mut self) -> HashSet<NodeId> {
        self.finished = true;
        self.store.end_tracking(self.prev_selector)
    }
}

impl<'a> Drop for DependencyTracker<'a> {
    fn drop(&mut self) {
        if !self.finished {
            self.store.end_tracking(self.prev_selector);
        }
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

    // Prevent recursive dependent updates by queueing
    updating_dependents: bool,
    pending_updates: Vec<NodeId>,
    pending_set: HashSet<NodeId>,

    // Async load tracking - maps signal NodeId to current load ID
    async_load_ids: HashMap<NodeId, u64>,

    // Async load timestamps - maps signal NodeId to when data was last loaded
    async_load_timestamps: HashMap<NodeId, web_time::Instant>,

    // Undo/redo manager
    undo_manager: UndoManager,

    // State persistence manager
    persistence_manager: PersistenceManager,

    id_counter: usize,
}

impl Store {
    pub(crate) fn has_value(&self, id: &NodeId) -> bool {
        self.values.contains_key(id)
    }

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
            updating_dependents: false,
            pending_updates: Vec::new(),
            pending_set: HashSet::new(),
            async_load_ids: HashMap::new(),
            async_load_timestamps: HashMap::new(),
            undo_manager: UndoManager::default(),
            persistence_manager: PersistenceManager::new(),
            id_counter: 0,
        }
    }

    // ========================================================================
    // Undo/Redo Methods
    // ========================================================================

    /// Get a reference to the undo manager.
    pub fn undo_manager(&self) -> &UndoManager {
        &self.undo_manager
    }

    /// Get a mutable reference to the undo manager.
    pub fn undo_manager_mut(&mut self) -> &mut UndoManager {
        &mut self.undo_manager
    }

    /// Perform undo - restores the previous state.
    pub fn undo(&mut self) -> bool {
        if !self.undo_manager.can_undo() {
            return false;
        }

        self.undo_manager.is_undoing = true;

        if let Some(entry) = self.undo_manager.undo_stack.pop() {
            // Create redo entry with current values
            let mut redo_entry = UndoEntry {
                description: entry.description.clone(),
                snapshots: Vec::with_capacity(entry.snapshots.len()),
                timestamp: entry.timestamp,
            };

            // Restore old values and save current for redo
            for snapshot in entry.snapshots {
                // Save current value for redo (using the snapshot's clone_fn)
                if self.values.contains_key(&snapshot.signal_id) {
                    // Actually get current value
                    if let Some(current) = self.values.remove(&snapshot.signal_id) {
                        redo_entry.snapshots.push(SignalSnapshot {
                            signal_id: snapshot.signal_id,
                            value: current,
                            clone_fn: snapshot.clone_fn,
                            debug_fn: snapshot.debug_fn,
                        });
                    }
                }

                // Restore the old value
                self.values.insert(snapshot.signal_id, snapshot.value);
                self.update_dependents(&snapshot.signal_id);
            }

            self.undo_manager.redo_stack.push(redo_entry);
        }

        self.undo_manager.is_undoing = false;

        // Update version signal to notify reactive can_undo/can_redo
        self.undo_manager.bump_version();
        self.update_undo_version_signal();

        true
    }

    /// Perform redo - restores the next state.
    pub fn redo(&mut self) -> bool {
        if !self.undo_manager.can_redo() {
            return false;
        }

        self.undo_manager.is_undoing = true;

        if let Some(entry) = self.undo_manager.redo_stack.pop() {
            // Create undo entry with current values
            let mut undo_entry = UndoEntry {
                description: entry.description.clone(),
                snapshots: Vec::with_capacity(entry.snapshots.len()),
                timestamp: entry.timestamp,
            };

            // Restore redo values and save current for undo
            for snapshot in entry.snapshots {
                // Save current value for undo
                if let Some(current) = self.values.remove(&snapshot.signal_id) {
                    undo_entry.snapshots.push(SignalSnapshot {
                        signal_id: snapshot.signal_id,
                        value: current,
                        clone_fn: snapshot.clone_fn,
                        debug_fn: snapshot.debug_fn,
                    });
                }

                // Restore the redo value
                self.values.insert(snapshot.signal_id, snapshot.value);
                self.update_dependents(&snapshot.signal_id);
            }

            self.undo_manager.undo_stack.push(undo_entry);
        }

        self.undo_manager.is_undoing = false;

        // Update version signal to notify reactive can_undo/can_redo
        self.undo_manager.bump_version();
        self.update_undo_version_signal();

        true
    }

    /// Update the internal undo version signal to trigger dependent updates.
    pub fn update_undo_version_signal(&mut self) {
        if let Some(version_id) = self.undo_manager.version_signal_id() {
            let version = self.undo_manager.version();
            self.values.insert(version_id, Box::new(version));
            self.update_dependents(&version_id);
        }
    }

    /// Initialize the undo version signal (called once during setup).
    pub fn init_undo_version_signal(&mut self) -> NodeId {
        let id = self.get_next_id();
        self.values.insert(id, Box::new(0u64));
        self.undo_manager.set_version_signal_id(id);
        id
    }

    /// Get the undo version signal ID, initializing if needed.
    pub fn get_or_init_undo_version_signal(&mut self) -> NodeId {
        if let Some(id) = self.undo_manager.version_signal_id() {
            id
        } else {
            self.init_undo_version_signal()
        }
    }

    // ========================================================================
    // Persistence Methods
    // ========================================================================

    /// Get a reference to the persistence manager.
    pub fn persistence_manager(&self) -> &PersistenceManager {
        &self.persistence_manager
    }

    /// Get a mutable reference to the persistence manager.
    pub fn persistence_manager_mut(&mut self) -> &mut PersistenceManager {
        &mut self.persistence_manager
    }

    /// Flush any pending persistence saves to disk.
    pub fn flush_persistence(&mut self) {
        self.persistence_manager.flush_pending(&self.values);
    }

    /// Check if there are pending persistence saves.
    pub fn has_pending_persistence(&self) -> bool {
        self.persistence_manager.has_pending_saves()
    }

    /// Check if the debounce delay has passed and we should flush persistence.
    pub fn should_flush_persistence(&self) -> bool {
        self.persistence_manager.should_flush()
    }

    /// Flush persistence if the debounce delay has passed.
    /// Returns true if a flush was performed.
    pub fn maybe_flush_persistence(&mut self) -> bool {
        if self.persistence_manager.should_flush() {
            self.persistence_manager.flush_pending(&self.values);
            true
        } else {
            false
        }
    }

    // ========================================================================
    // Time Travel Methods
    // ========================================================================

    /// Check if currently in time travel mode.
    pub fn is_ttrvl(&self) -> bool {
        self.undo_manager.is_ttrvl()
    }

    /// Get the current time travel position (None = at present).
    pub fn ttrvl_position(&self) -> Option<usize> {
        self.undo_manager.ttrvl_position()
    }

    /// Get the timeline for time travel UI.
    pub fn ttrvl_history(&self) -> Vec<HistoryEntry> {
        self.undo_manager.timeline()
    }

    /// Navigate to a specific position in the timeline.
    pub fn ttrvl_to(&mut self, target: usize) {
        let present_index = self.undo_manager.present_index();
        let timeline_len = self.undo_manager.timeline_len();

        // Clamp target to valid range
        let target = target.min(timeline_len.saturating_sub(1));

        // If already at target, nothing to do
        if self.undo_manager.ttrvl_position == Some(target) {
            return;
        }

        // If going to present
        if target == present_index {
            self.ttrvl_exit();
            return;
        }

        // Save present state on first navigation
        if self.undo_manager.ttrvl_saved_state.is_none() {
            let mut saved = HashMap::new();
            for signal_id in &self.undo_manager.undoable_signals.clone() {
                if let Some(value) = self.values.get(signal_id) {
                    if let Some(clone_fn) = self.undo_manager.get_clone_fn(signal_id) {
                        saved.insert(*signal_id, clone_fn(&**value));
                        self.undo_manager.ttrvl_saved_clone_fns.insert(*signal_id, clone_fn);
                    }
                }
            }
            self.undo_manager.ttrvl_saved_state = Some(saved);
        }

        // Start from saved state (present)
        if let Some(saved_state) = &self.undo_manager.ttrvl_saved_state {
            for (signal_id, clone_fn) in &self.undo_manager.ttrvl_saved_clone_fns.clone() {
                if let Some(value) = saved_state.get(signal_id) {
                    self.values.insert(*signal_id, clone_fn(&**value));
                }
            }
        }

        // Navigate to target by applying snapshots
        if target < present_index {
            // Go backward from present - apply undo snapshots in reverse
            for i in (target..present_index).rev() {
                if let Some(entry) = self.undo_manager.undo_stack.get(i) {
                    for snapshot in &entry.snapshots {
                        self.values.insert(snapshot.signal_id, snapshot.clone_value());
                    }
                }
            }
        } else {
            // Go forward from present - apply redo snapshots
            let redo_start = target - present_index - 1;
            for i in (0..=redo_start).rev() {
                let redo_index = self.undo_manager.redo_stack.len().saturating_sub(1).saturating_sub(i);
                if let Some(entry) = self.undo_manager.redo_stack.get(redo_index) {
                    for snapshot in &entry.snapshots {
                        self.values.insert(snapshot.signal_id, snapshot.clone_value());
                    }
                }
            }
        }

        self.undo_manager.ttrvl_position = Some(target);

        // Trigger updates for all undoable signals
        let signal_ids: Vec<_> = self.undo_manager.undoable_signals.iter().copied().collect();
        for signal_id in signal_ids {
            self.update_dependents(&signal_id);
        }

        self.undo_manager.bump_version();
        self.update_undo_version_signal();
    }

    /// Step backward in time travel.
    pub fn ttrvl_back(&mut self) {
        let current = self.undo_manager.ttrvl_position
            .unwrap_or_else(|| self.undo_manager.present_index());
        if current > 0 {
            self.ttrvl_to(current - 1);
        }
    }

    /// Step forward in time travel.
    pub fn ttrvl_forward(&mut self) {
        let current = self.undo_manager.ttrvl_position
            .unwrap_or_else(|| self.undo_manager.present_index());
        let max = self.undo_manager.timeline_len().saturating_sub(1);
        if current < max {
            self.ttrvl_to(current + 1);
        }
    }

    /// Exit time travel mode and return to present.
    pub fn ttrvl_exit(&mut self) {
        if self.undo_manager.ttrvl_saved_state.is_none() {
            self.undo_manager.ttrvl_position = None;
            return;
        }

        // Restore present state
        if let Some(saved_state) = self.undo_manager.ttrvl_saved_state.take() {
            for (signal_id, value) in saved_state {
                self.values.insert(signal_id, value);
                self.update_dependents(&signal_id);
            }
        }

        self.undo_manager.ttrvl_saved_clone_fns.clear();
        self.undo_manager.ttrvl_position = None;
        self.undo_manager.bump_version();
        self.update_undo_version_signal();
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

    // Updated set method with auto-snapshot for undoable signals and auto-persist
    fn set<T: 'static>(&mut self, id: &NodeId, value: T) {
        // Auto-snapshot for undoable signals
        self.auto_snapshot_if_needed(id);

        // Update the value
        self.values.insert(*id, Box::new(value));

        // Schedule auto-persist if this signal is persistent
        self.auto_persist_if_needed(id);

        self.update_dependents(id);
    }

    /// Schedule a persist save if the signal is registered for persistence.
    fn auto_persist_if_needed(&mut self, id: &NodeId) {
        if self.persistence_manager.is_persistent(id) {
            self.persistence_manager.schedule_save(*id);
        }
    }

    /// Auto-snapshot a signal if it's undoable and we're in an undo group.
    fn auto_snapshot_if_needed(&mut self, id: &NodeId) {
        // Check conditions
        if self.undo_manager.is_undoing {
            return;
        }
        if self.undo_manager.current_group.is_none() {
            return;
        }
        if !self.undo_manager.is_undoable(id) {
            return;
        }
        if self.undo_manager.current_group_signals.contains(id) {
            return;
        }

        // Get clone function and current value
        let clone_fn = match self.undo_manager.get_clone_fn(id) {
            Some(f) => f,
            None => return,
        };

        let debug_fn = match self.undo_manager.get_debug_fn(id) {
            Some(f) => f,
            None => return,
        };

        let current_value = match self.values.get(id) {
            Some(v) => v,
            None => return,
        };

        // Create snapshot
        let snapshot = SignalSnapshot {
            signal_id: *id,
            value: clone_fn(&**current_value),
            clone_fn,
            debug_fn,
        };

        // Add to current group
        if let Some(ref mut group) = self.undo_manager.current_group {
            group.snapshots.push(snapshot);
            self.undo_manager.current_group_signals.insert(*id);
        }
    }

    /// Set a value by NodeId - used internally for async state updates.
    pub(crate) fn set_by_id<T: 'static>(&mut self, id: &NodeId, value: T) {
        self.set(id, value);
    }

    /// Get a value by NodeId - used internally for async state checks.
    pub(crate) fn get_by_id<T: 'static>(&self, id: &NodeId) -> Option<&T> {
        self.values.get(id).and_then(|boxed| boxed.downcast_ref::<T>())
    }

    /// Track a signal as a dependency without getting its value.
    /// Used for reactive signals that need to observe internal state changes.
    pub fn track(&self, id: &NodeId) {
        self.record_dependency(*id);
    }

    /// Set the current async load ID for a signal.
    pub(crate) fn set_async_load_id(&mut self, signal_id: &NodeId, load_id: u64) {
        self.async_load_ids.insert(*signal_id, load_id);
    }

    /// Get the current async load ID for a signal.
    pub(crate) fn get_async_load_id(&self, signal_id: &NodeId) -> Option<u64> {
        self.async_load_ids.get(signal_id).copied()
    }

    /// Set the timestamp when data was loaded for a signal.
    pub(crate) fn set_async_load_timestamp(&mut self, signal_id: &NodeId) {
        self.async_load_timestamps.insert(*signal_id, web_time::Instant::now());
    }

    /// Get the timestamp when data was last loaded.
    pub(crate) fn get_async_load_timestamp(&self, signal_id: &NodeId) -> Option<web_time::Instant> {
        self.async_load_timestamps.get(signal_id).copied()
    }

    // Register a dependency - mutates state
    fn register_dependency(&mut self, selector_id: &NodeId, dependency_id: &NodeId) {
        // Record that selector depends on dependency
        self.dependencies.entry(*selector_id).or_default().insert(*dependency_id);

        // Record that dependency has selector as dependent
        self.dependents.entry(*dependency_id).or_default().insert(*selector_id);
    }

    // Fix update_dependents to handle both selectors and effects
    fn update_dependents(&mut self, id: &NodeId) {
        if self.pending_set.insert(*id) {
            self.pending_updates.push(*id);
        }

        if self.updating_dependents {
            return;
        }

        self.updating_dependents = true;

        while let Some(source_id) = self.pending_updates.pop() {
            self.pending_set.remove(&source_id);
            self.node_needs_update.insert(source_id, true);

            let initial_capacity = self.dependents.get(&source_id).map_or(0, |deps| deps.len());
            let mut queue = Vec::with_capacity(initial_capacity);
            let mut visited = HashSet::with_capacity(initial_capacity * 2);

            // Start with direct dependents
            if let Some(deps) = self.dependents.get(&source_id) {
                for dep_id in deps {
                    if visited.insert(*dep_id) {
                        queue.push(*dep_id);
                    }
                }
            }

            // Process queue iteratively
            while let Some(dependent_id) = queue.pop() {
                // It's a selector - recompute it as before
                self.recompute_selector(&dependent_id);

                // Add subsequent dependents
                if let Some(next_deps) = self.dependents.get(&dependent_id) {
                    for next_dep in next_deps {
                        if visited.insert(*next_dep) {
                            queue.push(*next_dep);
                        }
                    }
                }
            }
        }

        self.updating_dependents = false;
    }

    // Simplify dependency update logic
    fn recompute_selector(&mut self, id: &NodeId) {
        self.node_needs_update.insert(*id, true);
        if let Some(compute_fn) = &self.compute_fns.get(id) {
            let old_deps = self.dependencies.get(id).cloned().unwrap_or_default();

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

/// A reactive state primitive that stores a value of type `T`.
///
/// Signals are the core building block of Vizia's reactive system. They hold
/// values that can be read and updated, automatically triggering UI updates
/// when changed.
///
/// # Type Requirements
///
/// Signal values must implement `Clone` for several operations:
///
/// - **Reading via `.get()`**: Returns a reference, but derived signals
///   and bindings may need to clone the value for computations.
/// - **Updates via `.set()` / `.upd()`**: The value is cloned when storing.
/// - **Undo/redo**: Snapshots clone the value to preserve history.
/// - **Persistence**: Values are cloned for serialization.
///
/// This `Clone` bound is a deliberate design choice that enables:
/// - Simple, predictable ownership semantics
/// - Efficient derived signal caching
/// - Zero-copy reads in most cases (references are returned)
///
/// For large data structures, consider:
/// - Using `Arc<T>` to share data cheaply
/// - Storing indices/IDs and keeping data in a separate collection
/// - Using derived signals to project smaller views of the data
///
/// # Example
///
/// ```ignore
/// // Create a signal
/// let count = cx.state(0i32);
///
/// // Read the value
/// let value = count.get(store);
///
/// // Update the value
/// count.set(cx, 42);
/// count.upd(cx, |v| *v += 1);
///
/// // Create a derived signal
/// let doubled = count.drv(cx, |v, _| v * 2);
/// ```
pub struct Signal<T: 'static> {
    id: NodeId,
    ty: PhantomData<T>,
}

impl<T: 'static> Copy for Signal<T> {}

impl<T: 'static> Clone for Signal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static> Signal<T> {
    fn new(store: &mut Store, entity: Entity, default: T) -> Self {
        let id = store.get_next_id();
        store.entity_signals.entry(entity).or_default().insert(id);

        store.set(&id, default);

        Self { id, ty: PhantomData }
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
    pub fn get<'a>(&self, store: &'a impl DataContext) -> &'a T {
        let store_ref = store.store();
        if let Some(value) = store_ref.get::<T>(&self.id) {
            return value;
        }

        let has_value = store_ref.values.contains_key(&self.id);
        let owner = store_ref
            .entity_signals
            .iter()
            .find_map(|(entity, ids)| ids.contains(&self.id).then_some(*entity));

        if has_value {
            panic!(
                "Signal({:?}) type mismatch. Requested {}, but stored value has a different type. Owner: {:?}.",
                self.id,
                std::any::type_name::<T>(),
                owner
            );
        }

        panic!(
            "Signal({:?}) missing value for {}. Owner: {:?}.",
            self.id,
            std::any::type_name::<T>(),
            owner
        );
    }

    /// Returns the signal's value if it exists, or `None` if the signal was destroyed.
    ///
    /// Use this when accessing a signal that may have been invalidated (e.g., its owning
    /// entity was destroyed). For normal usage, prefer `get()` which panics with debug info.
    pub fn try_get<'a>(&self, store: &'a impl DataContext) -> Option<&'a T> {
        store.store().get::<T>(&self.id)
    }

    fn get_mut<'a>(&self, store: &'a mut Store) -> &'a mut T {
        store.get_mut::<T>(&self.id).unwrap()
    }

    // Mutations require mutable store access
    pub fn set(&self, store: &mut EventContext, value: T) {
        store.data.get_store_mut().set(&self.id, value);
    }

    // Update takes a function that works with references
    pub fn upd<F: FnOnce(&mut T)>(&self, store: &mut EventContext, updater: F) {
        let s = store.data.get_store_mut();

        // Auto-snapshot for undo before mutation
        s.auto_snapshot_if_needed(&self.id);

        // Mutate the value in place
        let old_value = self.get_mut(s);
        updater(old_value);

        // Schedule persistence save if registered
        s.auto_persist_if_needed(&self.id);

        s.update_dependents(&self.id);
    }

    pub fn observe(&self, store: &mut Store, entity: Entity) {
        store.observers.entry(self.id).or_default().insert(entity);
    }

    /// Creates a derived signal by mapping this signal's value.
    ///
    /// This is a convenience method that reduces boilerplate when creating
    /// derived signals. The closure receives both the value (`v`) and the store (`s`),
    /// allowing access to other signals if needed.
    ///
    /// The parameter names `v` and `s` are conventions - you can use any names
    /// that make sense for your use case (e.g., `|count, _|`, `|selected, store|`).
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Simple single-signal derivation:
    /// let doubled = self.count.drv(cx, |v, _| v * 2);
    ///
    /// // Multi-signal derivation (access other signals via store):
    /// let selected_text = self.selected.drv(cx, move |v, s| {
    ///     let items = self.items.get(s);
    ///     items.get(*v).cloned().unwrap_or_default()
    /// });
    /// ```
    pub fn drv<U, F>(&self, cx: &mut Context, f: F) -> Signal<U>
    where
        T: Clone,
        U: 'static + Clone,
        F: Fn(&T, &Store) -> U + 'static,
    {
        let signal = *self;
        cx.derived(move |s| f(signal.get(s), s))
    }

    /// Make this signal participate in undo/redo.
    ///
    /// Short form of `.undoable()`. Chain after `cx.state()`:
    /// ```ignore
    /// let count = cx.state(0).u(cx);
    /// ```
    pub fn u(self, cx: &mut Context) -> Self
    where
        T: Clone + Send + std::fmt::Debug,
    {
        cx.data.get_store_mut().undo_manager_mut().register_undoable::<T>(self.id);
        self
    }

    /// Make this signal participate in undo/redo.
    ///
    /// Chain after `cx.state()`:
    /// ```ignore
    /// let count = cx.state(0).undoable(cx);
    /// ```
    pub fn undoable(self, cx: &mut Context) -> Self
    where
        T: Clone + Send + std::fmt::Debug,
    {
        self.u(cx)
    }

    /// Make this signal persist to disk.
    ///
    /// Loads existing value from disk if available, and auto-saves on changes.
    /// Short form of `.persists()`. Chain after `cx.state()`:
    /// ```ignore
    /// let settings = cx.state(Settings::default()).p(cx, "settings");
    /// ```
    pub fn p(self, cx: &mut Context, key: impl Into<String>) -> Self
    where
        T: Clone + serde::Serialize + serde::de::DeserializeOwned,
    {
        let key = key.into();

        // Load from disk and set if exists
        if let Some(loaded) = cx.data.get_store_mut().persistence_manager_mut().load::<T>(&key) {
            cx.data.get_store_mut().set(&self.id, loaded);
        }

        // Register for auto-save
        cx.data.get_store_mut().persistence_manager_mut().register::<T>(self.id, key);

        self
    }

    /// Make this signal persist to disk.
    ///
    /// Loads existing value from disk if available, and auto-saves on changes.
    /// Chain after `cx.state()`:
    /// ```ignore
    /// let settings = cx.state(Settings::default()).persists(cx, "settings");
    /// ```
    pub fn persists(self, cx: &mut Context, key: impl Into<String>) -> Self
    where
        T: Clone + serde::Serialize + serde::de::DeserializeOwned,
    {
        self.p(cx, key)
    }
}

// Root container
pub struct RecoilRoot {
    store: Store,
}

impl Default for RecoilRoot {
    fn default() -> Self {
        Self::new()
    }
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
            // return self.get(lc.data.get_store()).to_string_local(cx);
            return lc.data.get_store().get::<T>(&self.id).unwrap().to_string_local(cx);
        }

        String::new()
    }
}

impl<T: Clone> Data for Signal<T> {
    fn same(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Signal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Signal({:?})", self.id)
    }
}

impl DataContext for Store {
    fn data<T: 'static>(&self) -> Option<&T> {
        None
    }

    fn store(&self) -> &crate::recoil::Store {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vizia_id::GenerationalId;

    #[test]
    fn test_signal_create_and_get() {
        let mut root = RecoilRoot::new();
        let signal = root.state(Entity::root(), 42i32);

        assert_eq!(*signal.get(root.get_store()), 42);
    }

    #[test]
    fn test_signal_set() {
        let mut root = RecoilRoot::new();
        let signal = root.state(Entity::root(), 10i32);

        root.get_store_mut().set(&signal.id(), 20i32);
        assert_eq!(*signal.get(root.get_store()), 20);
    }

    #[test]
    fn test_signal_update() {
        let mut root = RecoilRoot::new();
        let signal = root.state(Entity::root(), 5i32);

        // Use get_mut to update in place
        *signal.get_mut(root.get_store_mut()) += 10;
        assert_eq!(*signal.get(root.get_store()), 15);
    }

    #[test]
    fn test_derived_signal() {
        let mut root = RecoilRoot::new();
        let base = root.state(Entity::root(), 3i32);
        let doubled = root.derived(Entity::root(), move |s| {
            *base.get(s) * 2
        });

        assert_eq!(*doubled.get(root.get_store()), 6);

        root.get_store_mut().set(&base.id(), 5i32);
        assert_eq!(*doubled.get(root.get_store()), 10);
    }

    #[test]
    fn test_derived_chain() {
        let mut root = RecoilRoot::new();
        let a = root.state(Entity::root(), 2i32);
        let b = root.derived(Entity::root(), move |s| *a.get(s) + 1);
        let c = root.derived(Entity::root(), move |s| *b.get(s) * 2);

        assert_eq!(*a.get(root.get_store()), 2);
        assert_eq!(*b.get(root.get_store()), 3);
        assert_eq!(*c.get(root.get_store()), 6);

        root.get_store_mut().set(&a.id(), 10i32);
        assert_eq!(*b.get(root.get_store()), 11);
        assert_eq!(*c.get(root.get_store()), 22);
    }

    #[test]
    fn test_signal_try_get() {
        let mut root = RecoilRoot::new();
        let signal = root.state(Entity::root(), 42i32);

        assert_eq!(signal.try_get(root.get_store()), Some(&42));
    }

    #[test]
    fn test_node_id_equality() {
        let id1 = NodeId::new(1);
        let id2 = NodeId::new(1);
        let id3 = NodeId::new(2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_signal_copy() {
        let mut root = RecoilRoot::new();
        let signal1 = root.state(Entity::root(), 42i32);
        let signal2 = signal1; // Copy

        assert_eq!(signal1.id(), signal2.id());
        assert_eq!(*signal1.get(root.get_store()), *signal2.get(root.get_store()));
    }

    #[test]
    fn test_undo_manager_basic() {
        let manager = UndoManager::new(10);
        assert!(!manager.can_undo());
        assert!(!manager.can_redo());
    }

    #[test]
    fn test_undo_manager_max_history() {
        let mut manager = UndoManager::new(5);
        assert_eq!(manager.max_history(), 5);

        manager.set_max_history(10);
        assert_eq!(manager.max_history(), 10);
    }

    #[test]
    fn test_recoil_root_default() {
        let root = RecoilRoot::default();
        assert!(root.get_store().undo_manager().undo_history().is_empty());
    }

    #[test]
    fn test_multiple_signals() {
        let mut root = RecoilRoot::new();
        let sig_a = root.state(Entity::root(), 1i32);
        let sig_b = root.state(Entity::root(), 2i32);
        let sig_c = root.state(Entity::root(), 3i32);

        assert_eq!(*sig_a.get(root.get_store()), 1);
        assert_eq!(*sig_b.get(root.get_store()), 2);
        assert_eq!(*sig_c.get(root.get_store()), 3);

        root.get_store_mut().set(&sig_b.id(), 20i32);
        assert_eq!(*sig_a.get(root.get_store()), 1);
        assert_eq!(*sig_b.get(root.get_store()), 20);
        assert_eq!(*sig_c.get(root.get_store()), 3);
    }

    #[test]
    fn test_derived_with_multiple_deps() {
        let mut root = RecoilRoot::new();
        let x = root.state(Entity::root(), 2i32);
        let y = root.state(Entity::root(), 3i32);
        let sum = root.derived(Entity::root(), move |s| {
            *x.get(s) + *y.get(s)
        });

        assert_eq!(*sum.get(root.get_store()), 5);

        root.get_store_mut().set(&x.id(), 10i32);
        assert_eq!(*sum.get(root.get_store()), 13);

        root.get_store_mut().set(&y.id(), 7i32);
        assert_eq!(*sum.get(root.get_store()), 17);
    }
}

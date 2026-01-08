//! Undo/Redo and Time Travel infrastructure for signals.

use super::NodeId;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use web_time::Instant;

/// A snapshot of a single signal's value for undo/redo.
/// We store a clone function alongside the value to enable cloning.
pub(crate) struct SignalSnapshot {
    pub signal_id: NodeId,
    pub(crate) value: Box<dyn Any>,
    /// Function to clone the value (type-erased)
    pub(crate) clone_fn: fn(&dyn Any) -> Box<dyn Any>,
    /// Function to format the value for debug display (type-erased)
    pub(crate) debug_fn: fn(&dyn Any) -> String,
}

impl SignalSnapshot {
    /// Create a new snapshot with the given value.
    pub fn new<T: 'static + Clone + Send + std::fmt::Debug>(signal_id: NodeId, value: &T) -> Self {
        Self {
            signal_id,
            value: Box::new(value.clone()),
            clone_fn: |any| {
                let typed = any.downcast_ref::<T>().expect("Type mismatch in snapshot clone");
                Box::new(typed.clone())
            },
            debug_fn: |any| {
                if let Some(typed) = any.downcast_ref::<T>() {
                    format!("{:?}", typed)
                } else {
                    "<unknown>".to_string()
                }
            },
        }
    }

    /// Clone this snapshot's value.
    pub fn clone_value(&self) -> Box<dyn Any> {
        (self.clone_fn)(&*self.value)
    }

    /// Format the value for debug display.
    pub fn debug_value(&self) -> String {
        (self.debug_fn)(&*self.value)
    }
}

/// An entry in the undo/redo stack representing a group of changes.
pub struct UndoEntry {
    /// Human-readable description of the action (e.g., "Add Circle")
    pub description: String,
    /// Snapshots of signal values before the change
    pub(crate) snapshots: Vec<SignalSnapshot>,
    /// When this entry was created
    pub timestamp: Instant,
}

impl Default for UndoEntry {
    fn default() -> Self {
        Self {
            description: String::new(),
            snapshots: Vec::new(),
            timestamp: Instant::now(),
        }
    }
}

impl std::fmt::Debug for UndoEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UndoEntry")
            .field("description", &self.description)
            .field("snapshot_count", &self.snapshots.len())
            .field("timestamp", &self.timestamp)
            .finish()
    }
}

// ============================================================================
// Time Travel Debugging Types
// ============================================================================

/// Represents a change to a signal's value for time travel inspection.
#[derive(Clone, Debug)]
pub struct SignalChange {
    /// The signal that changed
    pub signal_id: NodeId,
    /// The old value (debug formatted)
    pub old_value: String,
    /// The new value (debug formatted)
    pub new_value: String,
}

/// An entry in the time travel history timeline.
#[derive(Clone, Debug)]
pub struct HistoryEntry {
    /// Position in the timeline (0 = oldest, higher = more recent)
    pub index: usize,
    /// Human-readable description of the action
    pub description: String,
    /// When this entry was created
    pub timestamp: Instant,
    /// Which signals changed and their values
    pub changes: Vec<SignalChange>,
    /// Whether this is the "present" marker
    pub is_present: bool,
}

/// Manages undo/redo stacks for the application.
pub struct UndoManager {
    /// Stack of undo entries (most recent at end)
    pub(crate) undo_stack: Vec<UndoEntry>,
    /// Stack of redo entries (most recent at end)
    pub(crate) redo_stack: Vec<UndoEntry>,
    /// Set of signal IDs that are tracked for undo
    pub(crate) undoable_signals: HashSet<NodeId>,
    /// Clone functions for undoable signals (type-erased)
    clone_fns: HashMap<NodeId, fn(&dyn Any) -> Box<dyn Any>>,
    /// Debug functions for undoable signals (type-erased)
    debug_fns: HashMap<NodeId, fn(&dyn Any) -> String>,
    /// Maximum number of undo entries to keep
    max_history: usize,
    /// Current undo group being recorded (None if not in a group)
    pub(crate) current_group: Option<UndoEntry>,
    /// Signals modified in the current group (to avoid duplicate snapshots)
    pub(crate) current_group_signals: HashSet<NodeId>,
    /// Whether we're currently performing an undo/redo (to skip recording)
    pub(crate) is_undoing: bool,
    /// Version counter that increments on any undo state change (for reactive signals)
    version: u64,
    /// NodeId of the internal version signal (set by Store)
    version_signal_id: Option<NodeId>,

    // Time travel state
    /// Current position in time travel mode (None = at present)
    pub(crate) ttrvl_position: Option<usize>,
    /// Saved present state when entering time travel mode
    pub(crate) ttrvl_saved_state: Option<HashMap<NodeId, Box<dyn Any>>>,
    /// Clone functions for saved state restoration
    pub(crate) ttrvl_saved_clone_fns: HashMap<NodeId, fn(&dyn Any) -> Box<dyn Any>>,
}

impl Default for UndoManager {
    fn default() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            undoable_signals: HashSet::new(),
            clone_fns: HashMap::new(),
            debug_fns: HashMap::new(),
            max_history: 100,
            current_group: None,
            current_group_signals: HashSet::new(),
            is_undoing: false,
            version: 0,
            version_signal_id: None,
            ttrvl_position: None,
            ttrvl_saved_state: None,
            ttrvl_saved_clone_fns: HashMap::new(),
        }
    }
}

impl UndoManager {
    /// Create a new undo manager with specified max history.
    pub fn new(max_history: usize) -> Self {
        Self { max_history, ..Default::default() }
    }

    /// Set the maximum number of undo entries to keep.
    pub fn set_max_history(&mut self, max: usize) {
        self.max_history = max;
        // Trim if needed
        while self.undo_stack.len() > self.max_history {
            self.undo_stack.remove(0);
        }
    }

    /// Get the current max history setting.
    pub fn max_history(&self) -> usize {
        self.max_history
    }

    /// Register a signal as undoable with its clone and debug functions.
    pub fn register_undoable<T: 'static + Clone + Send + std::fmt::Debug>(&mut self, signal_id: NodeId) {
        self.undoable_signals.insert(signal_id);
        self.clone_fns.insert(signal_id, |any| {
            let typed = any.downcast_ref::<T>().expect("Type mismatch in undo clone");
            Box::new(typed.clone())
        });
        self.debug_fns.insert(signal_id, |any| {
            if let Some(typed) = any.downcast_ref::<T>() {
                format!("{:?}", typed)
            } else {
                "<unknown>".to_string()
            }
        });
    }

    /// Get the debug function for a signal.
    pub fn get_debug_fn(&self, signal_id: &NodeId) -> Option<fn(&dyn Any) -> String> {
        self.debug_fns.get(signal_id).copied()
    }

    /// Clear all undo/redo history.
    pub fn clear_history(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.current_group = None;
        self.current_group_signals.clear();
        self.bump_version();
    }

    /// Set the version signal ID (called by Store during init).
    pub fn set_version_signal_id(&mut self, id: NodeId) {
        self.version_signal_id = Some(id);
    }

    /// Get the version signal ID.
    pub fn version_signal_id(&self) -> Option<NodeId> {
        self.version_signal_id
    }

    /// Get the current version.
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Bump the version (signals state change).
    pub(super) fn bump_version(&mut self) {
        self.version = self.version.wrapping_add(1);
    }

    /// Get the clone function for a signal.
    pub fn get_clone_fn(&self, signal_id: &NodeId) -> Option<fn(&dyn Any) -> Box<dyn Any>> {
        self.clone_fns.get(signal_id).copied()
    }

    /// Check if a signal is undoable.
    pub fn is_undoable(&self, signal_id: &NodeId) -> bool {
        self.undoable_signals.contains(signal_id)
    }

    /// Check if we can undo.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if we can redo.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the undo history for UI display.
    pub fn undo_history(&self) -> Vec<&str> {
        self.undo_stack.iter().rev().map(|e| e.description.as_str()).collect()
    }

    /// Get the redo history for UI display.
    pub fn redo_history(&self) -> Vec<&str> {
        self.redo_stack.iter().rev().map(|e| e.description.as_str()).collect()
    }

    /// Begin an undo group. All changes until end_group are grouped together.
    pub fn begin_group(&mut self, description: impl Into<String>) {
        if self.is_undoing {
            return;
        }
        self.current_group = Some(UndoEntry {
            description: description.into(),
            snapshots: Vec::new(),
            timestamp: Instant::now(),
        });
        self.current_group_signals.clear();
    }

    /// End the current undo group and push it to the stack.
    pub fn end_group(&mut self) {
        if self.is_undoing {
            return;
        }
        if let Some(group) = self.current_group.take() {
            if !group.snapshots.is_empty() {
                self.undo_stack.push(group);
                // Clear redo stack when new changes are made
                self.redo_stack.clear();
                // Trim history if needed
                while self.undo_stack.len() > self.max_history {
                    self.undo_stack.remove(0);
                }
                // Bump version to notify reactive signals
                self.bump_version();
            }
        }
        self.current_group_signals.clear();
    }

    /// Snapshot a signal's current value before it changes.
    /// Only records if the signal is undoable and we're in an undo group.
    pub fn snapshot_before_change<T: 'static + Clone + Send + std::fmt::Debug>(
        &mut self,
        signal_id: NodeId,
        current_value: &T,
    ) {
        if self.is_undoing || !self.undoable_signals.contains(&signal_id) {
            return;
        }

        // Only snapshot once per signal per group
        if self.current_group_signals.contains(&signal_id) {
            return;
        }

        if let Some(ref mut group) = self.current_group {
            group.snapshots.push(SignalSnapshot::new(signal_id, current_value));
            self.current_group_signals.insert(signal_id);
        }
    }

    // ========================================================================
    // Time Travel Methods
    // ========================================================================

    /// Check if currently in time travel mode.
    pub fn is_ttrvl(&self) -> bool {
        self.ttrvl_position.is_some()
    }

    /// Get the current time travel position (None = at present).
    pub fn ttrvl_position(&self) -> Option<usize> {
        self.ttrvl_position
    }

    /// Get the total number of entries in the timeline.
    pub fn timeline_len(&self) -> usize {
        // undo_stack + present + redo_stack
        self.undo_stack.len() + 1 + self.redo_stack.len()
    }

    /// Get the index that represents "present" in the timeline.
    pub fn present_index(&self) -> usize {
        self.undo_stack.len()
    }

    /// Build the full history timeline for time travel UI.
    pub fn timeline(&self) -> Vec<HistoryEntry> {
        let mut entries = Vec::with_capacity(self.timeline_len());

        // Past entries (undo stack, oldest first)
        for (i, entry) in self.undo_stack.iter().enumerate() {
            entries.push(HistoryEntry {
                index: i,
                description: entry.description.clone(),
                timestamp: entry.timestamp,
                changes: entry.snapshots.iter().map(|s| SignalChange {
                    signal_id: s.signal_id,
                    old_value: s.debug_value(),
                    new_value: String::new(), // Old value is what's stored
                }).collect(),
                is_present: false,
            });
        }

        // Present marker
        entries.push(HistoryEntry {
            index: self.undo_stack.len(),
            description: "Present".to_string(),
            timestamp: Instant::now(),
            changes: vec![],
            is_present: true,
        });

        // Future entries (redo stack, reversed so oldest undone is first)
        for (i, entry) in self.redo_stack.iter().rev().enumerate() {
            entries.push(HistoryEntry {
                index: self.undo_stack.len() + 1 + i,
                description: entry.description.clone(),
                timestamp: entry.timestamp,
                changes: entry.snapshots.iter().map(|s| SignalChange {
                    signal_id: s.signal_id,
                    old_value: s.debug_value(),
                    new_value: String::new(),
                }).collect(),
                is_present: false,
            });
        }

        entries
    }

    /// Get the description at a specific timeline position.
    pub fn description_at(&self, index: usize) -> String {
        let present = self.undo_stack.len();
        if index < present {
            self.undo_stack.get(index).map(|e| e.description.clone()).unwrap_or_default()
        } else if index == present {
            "Present".to_string()
        } else {
            let redo_index = self.redo_stack.len().saturating_sub(index - present);
            self.redo_stack.get(redo_index).map(|e| e.description.clone()).unwrap_or_default()
        }
    }
}

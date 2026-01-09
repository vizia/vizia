//! State persistence for signals.
//!
//! This module provides automatic save/load functionality for signals,
//! allowing state to persist across application restarts.
//!
//! # Security
//!
//! - Files are created with restrictive permissions (0600 on Unix)
//! - Data is stored in the platform's local app data directory
//! - Each app has its own isolated storage directory
//!
//! # Versioning
//!
//! Data is stored with a version number to support migrations:
//! ```json
//! {"v": 1, "data": {...}}
//! ```

use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use super::NodeId;

/// Current persistence format version.
const CURRENT_VERSION: u32 = 1;

/// Debounce delay for persistence saves (500ms).
const DEBOUNCE_DELAY: Duration = Duration::from_millis(500);

/// Errors that can occur during persistence operations.
#[derive(Debug, Clone)]
pub enum PersistenceError {
    /// App name not configured - call `cx.configure_persistence()` first.
    AppNameNotConfigured,
    /// Failed to create storage directory.
    DirectoryCreationFailed(String),
    /// Failed to read persisted file.
    ReadFailed { key: String, error: String },
    /// Failed to write persisted file.
    WriteFailed { key: String, error: String },
    /// Failed to deserialize data (possibly version mismatch).
    DeserializeFailed { key: String, error: String },
    /// Failed to serialize data.
    SerializeFailed { key: String, error: String },
    /// Data version is newer than supported.
    VersionTooNew { key: String, found: u32, max_supported: u32 },
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AppNameNotConfigured => {
                write!(f, "Persistence not configured. Call cx.configure_persistence(\"app_name\") first.")
            }
            Self::DirectoryCreationFailed(e) => {
                write!(f, "Failed to create persistence directory: {}", e)
            }
            Self::ReadFailed { key, error } => {
                write!(f, "Failed to read '{}': {}", key, error)
            }
            Self::WriteFailed { key, error } => {
                write!(f, "Failed to write '{}': {}", key, error)
            }
            Self::DeserializeFailed { key, error } => {
                write!(f, "Failed to deserialize '{}': {}", key, error)
            }
            Self::SerializeFailed { key, error } => {
                write!(f, "Failed to serialize '{}': {}", key, error)
            }
            Self::VersionTooNew { key, found, max_supported } => {
                write!(
                    f,
                    "Data for '{}' has version {} but max supported is {}",
                    key, found, max_supported
                )
            }
        }
    }
}

impl std::error::Error for PersistenceError {}

/// Wrapper for versioned persistent data.
#[derive(serde::Serialize, serde::Deserialize)]
struct VersionedData<T> {
    /// Format version number.
    v: u32,
    /// The actual data.
    data: T,
}

/// Manages persistence of signal values to disk.
///
/// Signals registered with the persistence manager will:
/// - Load their initial value from disk (if available)
/// - Auto-save when their value changes (debounced)
/// - Save pending changes on application exit
///
/// # Setup
///
/// Before using persistent signals, configure the app name:
/// ```ignore
/// cx.configure_persistence("my_app_name");
/// let settings = cx.persists("settings", Settings::default());
/// ```
pub struct PersistenceManager {
    /// Application name (used in storage path).
    app_name: Option<String>,

    /// Maps signal IDs to their persistence keys.
    keys: HashMap<NodeId, String>,

    /// Maps signal IDs to serialize functions (type-erased).
    serialize_fns: HashMap<NodeId, fn(&dyn Any) -> Option<String>>,

    /// Pending saves (for debouncing).
    pending_saves: HashSet<NodeId>,

    /// When the last change was scheduled (for debouncing).
    last_change_time: Option<Instant>,

    /// Recent errors (kept for inspection).
    recent_errors: Vec<PersistenceError>,

    /// Whether persistence is enabled (requires app name).
    enabled: bool,
}

impl Default for PersistenceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PersistenceManager {
    /// Creates a new PersistenceManager.
    ///
    /// Note: Persistence is disabled until `configure()` is called with an app name.
    pub fn new() -> Self {
        Self {
            app_name: None,
            keys: HashMap::new(),
            serialize_fns: HashMap::new(),
            pending_saves: HashSet::new(),
            last_change_time: None,
            recent_errors: Vec::new(),
            enabled: false,
        }
    }

    /// Configures persistence with the given app name.
    ///
    /// This must be called before using `.p()` / `.persists()` on signals.
    /// The app name is used to create an isolated storage directory.
    ///
    /// # Example
    /// ```ignore
    /// cx.configure_persistence("my_app");
    /// ```
    pub fn configure(&mut self, app_name: impl Into<String>) {
        let name = app_name.into();
        // Sanitize app name for filesystem safety
        let sanitized: String = name
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
            .collect();
        self.app_name = Some(sanitized);
        self.enabled = true;
    }

    /// Returns whether persistence is configured and enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns the configured app name.
    pub fn app_name(&self) -> Option<&str> {
        self.app_name.as_deref()
    }

    /// Returns the storage path for this app's persistent data.
    ///
    /// Format: `{data_local_dir}/{app_name}/signals/`
    pub fn storage_path(&self) -> Option<PathBuf> {
        self.app_name.as_ref().map(|name| {
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(name)
                .join("signals")
        })
    }

    /// Returns recent errors that occurred during persistence operations.
    pub fn recent_errors(&self) -> &[PersistenceError] {
        &self.recent_errors
    }

    /// Clears the recent errors list.
    pub fn clear_errors(&mut self) {
        self.recent_errors.clear();
    }

    /// Records an error.
    fn record_error(&mut self, error: PersistenceError) {
        log::error!("Persistence error: {}", error);
        // Keep max 10 recent errors
        if self.recent_errors.len() >= 10 {
            self.recent_errors.remove(0);
        }
        self.recent_errors.push(error);
    }

    /// Registers a signal for persistence with the given key.
    ///
    /// The key is used as the filename (with .json extension) in the storage directory.
    pub fn register<T>(&mut self, signal_id: NodeId, key: String)
    where
        T: serde::Serialize + 'static,
    {
        if !self.enabled {
            self.record_error(PersistenceError::AppNameNotConfigured);
            return;
        }

        self.keys.insert(signal_id, key);

        self.serialize_fns.insert(signal_id, |any| {
            any.downcast_ref::<T>().and_then(|v| {
                let versioned = VersionedData { v: CURRENT_VERSION, data: v };
                serde_json::to_string_pretty(&versioned).ok()
            })
        });
    }

    /// Loads a value from disk for the given key.
    ///
    /// Returns `None` if:
    /// - Persistence is not configured
    /// - The file doesn't exist
    /// - The data cannot be parsed
    /// - The data version is incompatible
    pub fn load<T>(&mut self, key: &str) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let storage_path = match self.storage_path() {
            Some(p) => p,
            None => {
                self.record_error(PersistenceError::AppNameNotConfigured);
                return None;
            }
        };

        let path = storage_path.join(format!("{}.json", key));

        // Read file
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // File doesn't exist - not an error, just no saved data
                return None;
            }
            Err(e) => {
                self.record_error(PersistenceError::ReadFailed {
                    key: key.to_string(),
                    error: e.to_string(),
                });
                return None;
            }
        };

        // Parse versioned wrapper
        let versioned: VersionedData<T> = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                self.record_error(PersistenceError::DeserializeFailed {
                    key: key.to_string(),
                    error: e.to_string(),
                });
                return None;
            }
        };

        // Check version
        if versioned.v > CURRENT_VERSION {
            self.record_error(PersistenceError::VersionTooNew {
                key: key.to_string(),
                found: versioned.v,
                max_supported: CURRENT_VERSION,
            });
            return None;
        }

        // Future: Add migration logic here for older versions
        // if versioned.v < CURRENT_VERSION { ... }

        Some(versioned.data)
    }

    /// Checks if a signal is registered for persistence.
    pub fn is_persistent(&self, signal_id: &NodeId) -> bool {
        self.keys.contains_key(signal_id)
    }

    /// Schedules a save for the given signal.
    ///
    /// The actual save is debounced - multiple rapid changes will be batched.
    pub fn schedule_save(&mut self, signal_id: NodeId) {
        if !self.enabled {
            return;
        }
        self.pending_saves.insert(signal_id);
        self.last_change_time = Some(Instant::now());
    }

    /// Returns true if there are pending saves.
    pub fn has_pending_saves(&self) -> bool {
        !self.pending_saves.is_empty()
    }

    /// Returns true if the debounce delay has passed since the last change.
    ///
    /// This should be checked periodically in the event loop to trigger saves.
    pub fn should_flush(&self) -> bool {
        if !self.enabled || self.pending_saves.is_empty() {
            return false;
        }

        match self.last_change_time {
            Some(time) => time.elapsed() >= DEBOUNCE_DELAY,
            None => false,
        }
    }

    /// Flushes all pending saves to disk.
    ///
    /// This is called by the debounce timer and on application exit.
    pub fn flush_pending(&mut self, values: &HashMap<NodeId, Box<dyn Any>>) {
        if !self.enabled || self.pending_saves.is_empty() {
            return;
        }

        let storage_path = match self.storage_path() {
            Some(p) => p,
            None => return,
        };

        // Ensure storage directory exists with secure permissions
        if let Err(e) = Self::create_secure_dir(&storage_path) {
            self.record_error(PersistenceError::DirectoryCreationFailed(e.to_string()));
            return;
        }

        // Collect pending saves to avoid borrow conflict
        let pending: Vec<NodeId> = self.pending_saves.drain().collect();
        let mut errors: Vec<PersistenceError> = Vec::new();

        for signal_id in pending {
            let key = match self.keys.get(&signal_id) {
                Some(k) => k.clone(),
                None => continue,
            };

            let serialize = match self.serialize_fns.get(&signal_id) {
                Some(f) => *f,
                None => continue,
            };

            let value = match values.get(&signal_id) {
                Some(v) => v,
                None => continue,
            };

            let json = match serialize(value.as_ref()) {
                Some(j) => j,
                None => {
                    errors.push(PersistenceError::SerializeFailed {
                        key: key.clone(),
                        error: "Serialization returned None".to_string(),
                    });
                    continue;
                }
            };

            let path = storage_path.join(format!("{}.json", key));

            if let Err(e) = Self::write_secure_file(&path, &json) {
                errors.push(PersistenceError::WriteFailed {
                    key,
                    error: e.to_string(),
                });
            } else {
                log::debug!("Persisted signal '{}' to {:?}", key, path);
            }
        }

        // Record all errors after the loop
        for error in errors {
            self.record_error(error);
        }

        self.last_change_time = None;
    }

    /// Creates a directory with secure permissions.
    fn create_secure_dir(path: &PathBuf) -> std::io::Result<()> {
        std::fs::create_dir_all(path)?;

        // Set directory permissions on Unix (owner only: rwx)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o700);
            std::fs::set_permissions(path, perms)?;
        }

        Ok(())
    }

    /// Writes a file with secure permissions.
    fn write_secure_file(path: &PathBuf, content: &str) -> std::io::Result<()> {
        std::fs::write(path, content)?;

        // Set file permissions on Unix (owner only: rw)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o600);
            std::fs::set_permissions(path, perms)?;
        }

        Ok(())
    }

    /// Clears all persistence data for a signal.
    pub fn unregister(&mut self, signal_id: &NodeId) {
        if let Some(key) = self.keys.remove(signal_id) {
            self.serialize_fns.remove(signal_id);
            self.pending_saves.remove(signal_id);

            // Delete the file
            if let Some(storage_path) = self.storage_path() {
                let path = storage_path.join(format!("{}.json", key));
                let _ = std::fs::remove_file(&path);
            }
        }
    }

    /// Deletes all persisted data for this app.
    pub fn clear_all(&mut self) -> std::io::Result<()> {
        if let Some(storage_path) = self.storage_path() {
            if storage_path.exists() {
                std::fs::remove_dir_all(&storage_path)?;
            }
        }
        self.keys.clear();
        self.serialize_fns.clear();
        self.pending_saves.clear();
        self.last_change_time = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
    struct TestData {
        name: String,
        value: i32,
    }

    #[test]
    fn test_not_configured() {
        let manager = PersistenceManager::new();
        assert!(!manager.is_enabled());
        assert!(manager.storage_path().is_none());
    }

    #[test]
    fn test_configure() {
        let mut manager = PersistenceManager::new();
        manager.configure("my_app");
        assert!(manager.is_enabled());
        assert_eq!(manager.app_name(), Some("my_app"));
        assert!(manager.storage_path().is_some());
    }

    #[test]
    fn test_sanitize_app_name() {
        let mut manager = PersistenceManager::new();
        manager.configure("my app/with:special\\chars");
        assert_eq!(manager.app_name(), Some("my_app_with_special_chars"));
    }

    #[test]
    fn test_versioned_format() {
        let data = TestData { name: "test".to_string(), value: 42 };
        let versioned = VersionedData { v: CURRENT_VERSION, data: &data };
        let json = serde_json::to_string(&versioned).unwrap();

        assert!(json.contains("\"v\":1"));
        assert!(json.contains("\"data\""));
    }
}

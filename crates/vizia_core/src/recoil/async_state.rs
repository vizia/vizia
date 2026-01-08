//! Async state management for signals.
//!
//! Provides comprehensive async data loading with:
//! - Cancellation support
//! - Deduplication (skip if already loading)
//! - Stale-while-revalidate pattern
//! - Timeout support
//! - Retry with exponential backoff
//!
//! # Example
//!
//! ```ignore
//! // Basic usage
//! let users: Signal<Async<Vec<User>, String>> = cx.state(Async::Idle);
//! cx.load_async(users, || fetch_users());
//!
//! // With cancellation
//! let handle = cx.load_async_cancelable(users, || fetch_users());
//! handle.cancel();
//!
//! // With options (timeout, retry, etc.)
//! cx.load_async_with(users, AsyncOptions::default()
//!     .timeout(Duration::from_secs(30))
//!     .retry(3), || fetch_users());
//!
//! // Refresh (reload even if data exists, shows stale data while loading)
//! cx.refresh_async(users, || fetch_users());
//! ```

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::context::{DataContext, EventContext};

use super::{NodeId, Signal};

// Global counter for generating unique load IDs
static LOAD_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

fn next_load_id() -> u64 {
    LOAD_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// Represents the state of an asynchronous operation.
///
/// States:
/// - `Idle` - No operation started
/// - `Loading` - First load in progress (no prior data)
/// - `Ready(T)` - Data loaded successfully
/// - `Reloading(T)` - Refreshing with stale data available
/// - `Error(E)` - Operation failed
/// - `Stale(T, E)` - Has old data but last refresh failed
/// - `Timeout` - Operation timed out
/// - `Retrying(attempt, max_attempts, last_error)` - Retry in progress after failure
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Async<T, E = String> {
    /// Initial state before any operation has started.
    #[default]
    Idle,
    /// First load in progress, no prior data available.
    Loading,
    /// Operation completed successfully with data.
    Ready(T),
    /// Refreshing with stale data still available for display.
    Reloading(T),
    /// Operation failed with an error.
    Error(E),
    /// Has stale data from previous success, but last refresh failed.
    Stale(T, E),
    /// Operation timed out.
    Timeout,
    /// Retry in progress after a failed attempt (current attempt, max attempts, last error).
    Retrying(u32, u32, E),
}


impl<T, E> Async<T, E> {
    /// Returns `true` if the state is `Idle`.
    pub fn is_idle(&self) -> bool {
        matches!(self, Async::Idle)
    }

    /// Returns `true` if any loading is in progress (`Loading`, `Reloading`, or `Retrying`).
    pub fn is_loading(&self) -> bool {
        matches!(self, Async::Loading | Async::Reloading(_) | Async::Retrying(_, _, _))
    }

    /// Returns `true` if this is the first load (no prior data).
    pub fn is_first_load(&self) -> bool {
        matches!(self, Async::Loading)
    }

    /// Returns `true` if refreshing with stale data available.
    pub fn is_reloading(&self) -> bool {
        matches!(self, Async::Reloading(_))
    }

    /// Returns `true` if data is available (`Ready`, `Reloading`, or `Stale`).
    pub fn is_ready(&self) -> bool {
        matches!(self, Async::Ready(_) | Async::Reloading(_) | Async::Stale(_, _))
    }

    /// Returns `true` if in an error state (`Error` or `Stale`).
    pub fn is_error(&self) -> bool {
        matches!(self, Async::Error(_) | Async::Stale(_, _))
    }

    /// Returns `true` if has fresh data (not stale).
    pub fn is_fresh(&self) -> bool {
        matches!(self, Async::Ready(_))
    }

    /// Returns `true` if has stale data.
    pub fn is_stale(&self) -> bool {
        matches!(self, Async::Stale(_, _) | Async::Reloading(_))
    }

    /// Returns `true` if the operation timed out.
    pub fn is_timeout(&self) -> bool {
        matches!(self, Async::Timeout)
    }

    /// Returns `true` if retrying after a failed attempt.
    pub fn is_retrying(&self) -> bool {
        matches!(self, Async::Retrying(_, _, _))
    }

    /// Returns retry progress info (current_attempt, max_attempts, last_error) if retrying.
    pub fn retry_info(&self) -> Option<(u32, u32, &E)> {
        match self {
            Async::Retrying(attempt, max, err) => Some((*attempt, *max, err)),
            _ => None,
        }
    }

    /// Returns the data if available (from `Ready`, `Reloading`, or `Stale`).
    pub fn data(&self) -> Option<&T> {
        match self {
            Async::Ready(data) | Async::Reloading(data) | Async::Stale(data, _) => Some(data),
            _ => None,
        }
    }

    /// Returns the data only if fresh (from `Ready` only).
    pub fn fresh_data(&self) -> Option<&T> {
        match self {
            Async::Ready(data) => Some(data),
            _ => None,
        }
    }

    /// Returns the error if in error state.
    pub fn error(&self) -> Option<&E> {
        match self {
            Async::Error(err) | Async::Stale(_, err) => Some(err),
            _ => None,
        }
    }

    /// Maps the data type using the provided function.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Async<U, E> {
        match self {
            Async::Idle => Async::Idle,
            Async::Loading => Async::Loading,
            Async::Ready(data) => Async::Ready(f(data)),
            Async::Reloading(data) => Async::Reloading(f(data)),
            Async::Error(err) => Async::Error(err),
            Async::Stale(data, err) => Async::Stale(f(data), err),
            Async::Timeout => Async::Timeout,
            Async::Retrying(attempt, max, err) => Async::Retrying(attempt, max, err),
        }
    }

    /// Maps the error type using the provided function.
    pub fn map_err<F2, G: FnOnce(E) -> F2>(self, f: G) -> Async<T, F2> {
        match self {
            Async::Idle => Async::Idle,
            Async::Loading => Async::Loading,
            Async::Ready(data) => Async::Ready(data),
            Async::Reloading(data) => Async::Reloading(data),
            Async::Error(err) => Async::Error(f(err)),
            Async::Stale(data, err) => Async::Stale(data, f(err)),
            Async::Timeout => Async::Timeout,
            Async::Retrying(attempt, max, err) => Async::Retrying(attempt, max, f(err)),
        }
    }

    /// Converts to `Option<&T>`, prioritizing any available data.
    pub fn as_option(&self) -> Option<&T> {
        self.data()
    }

    /// Converts to `Result<&T, &E>` based on current state.
    /// Returns `Err` only if in pure error state with no data.
    pub fn as_result(&self) -> Result<Option<&T>, &E> {
        match self {
            Async::Error(err) => Err(err),
            _ => Ok(self.data()),
        }
    }
}

impl<T: Clone, E: Clone> Async<T, E> {
    /// Extracts owned data if available.
    pub fn into_data(self) -> Option<T> {
        match self {
            Async::Ready(data) | Async::Reloading(data) | Async::Stale(data, _) => Some(data),
            _ => None,
        }
    }
}

/// Handle for canceling an async operation.
#[derive(Clone)]
pub struct AsyncHandle {
    cancelled: Arc<AtomicBool>,
    load_id: u64,
}

impl AsyncHandle {
    /// Create a new handle (internal use).
    pub(crate) fn new_internal() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
            load_id: next_load_id(),
        }
    }

    /// Create an already-cancelled handle (for deduplication).
    pub(crate) fn new_cancelled() -> Self {
        let handle = Self::new_internal();
        handle.cancelled.store(true, Ordering::SeqCst);
        handle
    }

    #[cfg(test)]
    fn new() -> Self {
        Self::new_internal()
    }

    /// Cancel the async operation.
    /// The operation will stop at the next cancellation check point.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    /// Check if the operation was cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    /// Get the unique ID for this load operation.
    pub fn load_id(&self) -> u64 {
        self.load_id
    }
}

impl std::fmt::Debug for AsyncHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncHandle")
            .field("cancelled", &self.is_cancelled())
            .field("load_id", &self.load_id)
            .finish()
    }
}

/// Options for configuring async loading behavior.
#[derive(Clone, Debug)]
pub struct AsyncOptions {
    /// Timeout duration. `None` means no timeout.
    pub timeout: Option<Duration>,
    /// Number of retry attempts (0 = no retries).
    pub retries: u32,
    /// Base delay for exponential backoff between retries.
    pub retry_delay: Duration,
    /// Maximum delay between retries.
    pub max_retry_delay: Duration,
    /// Whether to deduplicate (skip if already loading).
    pub dedupe: bool,
    /// Whether to preserve existing data while loading (stale-while-revalidate).
    /// If true, goes to `Reloading(data)`. If false, goes to `Loading`.
    pub preserve_data: bool,
}

impl Default for AsyncOptions {
    fn default() -> Self {
        Self {
            timeout: None,
            retries: 0,
            retry_delay: Duration::from_millis(100),
            max_retry_delay: Duration::from_secs(10),
            dedupe: true,
            preserve_data: false, // Default: fresh load discards old data
        }
    }
}

impl AsyncOptions {
    /// Create new options with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set timeout duration.
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    /// Set number of retry attempts.
    pub fn retry(mut self, attempts: u32) -> Self {
        self.retries = attempts;
        self
    }

    /// Set retry with custom delays.
    pub fn retry_with_delay(mut self, attempts: u32, base_delay: Duration) -> Self {
        self.retries = attempts;
        self.retry_delay = base_delay;
        self
    }

    /// Set maximum retry delay for exponential backoff.
    pub fn max_retry_delay(mut self, duration: Duration) -> Self {
        self.max_retry_delay = duration;
        self
    }

    /// Enable or disable deduplication.
    pub fn dedupe(mut self, enabled: bool) -> Self {
        self.dedupe = enabled;
        self
    }

    /// Enable or disable data preservation (stale-while-revalidate).
    /// When enabled, existing data is kept visible during reload.
    pub fn preserve_data(mut self, enabled: bool) -> Self {
        self.preserve_data = enabled;
        self
    }

    /// Preset: quick timeout, no retries.
    pub fn quick() -> Self {
        Self::default().timeout(Duration::from_secs(5))
    }

    /// Preset: patient loading with retries.
    pub fn patient() -> Self {
        Self::default()
            .timeout(Duration::from_secs(30))
            .retry(3)
    }

    /// Preset: aggressive retries for unreliable operations.
    pub fn resilient() -> Self {
        Self::default()
            .timeout(Duration::from_secs(60))
            .retry(5)
            .retry_with_delay(5, Duration::from_millis(500))
    }
}

/// Extension trait for `Signal<Async<T, E>>` providing convenience methods.
pub trait AsyncSignalExt<T, E> {
    /// Returns `true` if the state is `Idle`.
    fn is_idle(&self, cx: &impl DataContext) -> bool;

    /// Returns `true` if any loading is in progress.
    fn is_loading(&self, cx: &impl DataContext) -> bool;

    /// Returns `true` if this is the first load (no prior data).
    fn is_first_load(&self, cx: &impl DataContext) -> bool;

    /// Returns `true` if data is available (fresh or stale).
    fn is_ready(&self, cx: &impl DataContext) -> bool;

    /// Returns `true` if in an error state.
    fn is_error(&self, cx: &impl DataContext) -> bool;

    /// Returns `true` if has fresh (non-stale) data.
    fn is_fresh(&self, cx: &impl DataContext) -> bool;

    /// Returns `true` if data is stale.
    fn is_stale(&self, cx: &impl DataContext) -> bool;

    /// Returns `true` if the operation timed out.
    fn is_timeout(&self, cx: &impl DataContext) -> bool;

    /// Returns `true` if retrying after a failed attempt.
    fn is_retrying(&self, cx: &impl DataContext) -> bool;

    /// Returns retry progress info (current_attempt, max_attempts, last_error) if retrying.
    fn retry_info<'a>(&self, cx: &'a impl DataContext) -> Option<(u32, u32, &'a E)>;

    /// Returns the data if available (fresh or stale).
    fn data<'a>(&self, cx: &'a impl DataContext) -> Option<&'a T>;

    /// Returns the data only if fresh.
    fn fresh_data<'a>(&self, cx: &'a impl DataContext) -> Option<&'a T>;

    /// Returns the error if in error state.
    fn error<'a>(&self, cx: &'a impl DataContext) -> Option<&'a E>;

    /// Sets the state to `Loading`.
    fn set_loading(&self, cx: &mut EventContext);

    /// Sets the state to `Ready` with the provided data.
    fn set_ready(&self, cx: &mut EventContext, data: T);

    /// Sets the state to `Error` with the provided error.
    fn set_error(&self, cx: &mut EventContext, error: E);

    /// Resets the state to `Idle`.
    fn reset(&self, cx: &mut EventContext);

    /// Returns how long ago the data was loaded, if available.
    fn age(&self, cx: &impl DataContext) -> Option<Duration>;

    /// Returns `true` if the data is older than the given TTL.
    /// Returns `false` if no data is loaded or no timestamp exists.
    fn is_expired(&self, cx: &impl DataContext, ttl: Duration) -> bool;

    /// Returns `true` if data exists and is within the TTL (not expired).
    fn is_fresh_within(&self, cx: &impl DataContext, ttl: Duration) -> bool;
}

impl<T: 'static + Clone, E: 'static + Clone> AsyncSignalExt<T, E> for Signal<Async<T, E>> {
    fn is_idle(&self, cx: &impl DataContext) -> bool {
        self.get(cx).is_idle()
    }

    fn is_loading(&self, cx: &impl DataContext) -> bool {
        self.get(cx).is_loading()
    }

    fn is_first_load(&self, cx: &impl DataContext) -> bool {
        self.get(cx).is_first_load()
    }

    fn is_ready(&self, cx: &impl DataContext) -> bool {
        self.get(cx).is_ready()
    }

    fn is_error(&self, cx: &impl DataContext) -> bool {
        self.get(cx).is_error()
    }

    fn is_fresh(&self, cx: &impl DataContext) -> bool {
        self.get(cx).is_fresh()
    }

    fn is_stale(&self, cx: &impl DataContext) -> bool {
        self.get(cx).is_stale()
    }

    fn is_timeout(&self, cx: &impl DataContext) -> bool {
        self.get(cx).is_timeout()
    }

    fn is_retrying(&self, cx: &impl DataContext) -> bool {
        self.get(cx).is_retrying()
    }

    fn retry_info<'a>(&self, cx: &'a impl DataContext) -> Option<(u32, u32, &'a E)> {
        self.get(cx).retry_info()
    }

    fn data<'a>(&self, cx: &'a impl DataContext) -> Option<&'a T> {
        self.get(cx).data()
    }

    fn fresh_data<'a>(&self, cx: &'a impl DataContext) -> Option<&'a T> {
        self.get(cx).fresh_data()
    }

    fn error<'a>(&self, cx: &'a impl DataContext) -> Option<&'a E> {
        self.get(cx).error()
    }

    fn set_loading(&self, cx: &mut EventContext) {
        self.set(cx, Async::Loading);
    }

    fn set_ready(&self, cx: &mut EventContext, data: T) {
        self.set(cx, Async::Ready(data));
    }

    fn set_error(&self, cx: &mut EventContext, error: E) {
        self.set(cx, Async::Error(error));
    }

    fn reset(&self, cx: &mut EventContext) {
        self.set(cx, Async::Idle);
    }

    fn age(&self, cx: &impl DataContext) -> Option<Duration> {
        let store = cx.store();
        store.get_async_load_timestamp(&self.id()).map(|t| t.elapsed())
    }

    fn is_expired(&self, cx: &impl DataContext, ttl: Duration) -> bool {
        self.age(cx).map(|age| age > ttl).unwrap_or(false)
    }

    fn is_fresh_within(&self, cx: &impl DataContext, ttl: Duration) -> bool {
        self.get(cx).is_ready() && !self.is_expired(cx, ttl)
    }
}

/// Type-erased async completion event for processing in the event loop.
pub struct AsyncCompletionEvent {
    pub(crate) handler: Box<dyn FnOnce(&mut EventContext) + Send>,
}

impl AsyncCompletionEvent {
    /// Create completion event for a successful/failed load.
    pub(crate) fn new<T, E>(signal_id: NodeId, result: Result<T, E>, load_id: u64) -> Self
    where
        T: 'static + Send + Clone,
        E: 'static + Send + Clone,
    {
        let handler = Box::new(move |cx: &mut EventContext| {
            let store = cx.data.get_store_mut();

            // Get current state to check if this is still the active load
            // and to preserve stale data on error
            let current: Option<Async<T, E>> = store
                .get_by_id::<Async<T, E>>(&signal_id)
                .cloned();

            // Check if there's a newer load in progress (stale completion)
            if let Some(current_load_id) = store.get_async_load_id(&signal_id) {
                if current_load_id > load_id {
                    // This completion is stale, ignore it
                    return;
                }
            }

            match result {
                Ok(data) => {
                    store.set_by_id(&signal_id, Async::<T, E>::Ready(data));
                    // Record when data was loaded for TTL tracking
                    store.set_async_load_timestamp(&signal_id);
                }
                Err(error) => {
                    // Preserve stale data if available
                    let new_state = match current {
                        Some(Async::Reloading(old_data)) => Async::Stale(old_data, error),
                        _ => Async::Error(error),
                    };
                    store.set_by_id(&signal_id, new_state);
                }
            }
        });
        Self { handler }
    }

    /// Create completion event for a cancelled load.
    pub(crate) fn cancelled<T, E>(signal_id: NodeId, load_id: u64) -> Self
    where
        T: 'static + Send + Clone,
        E: 'static + Send + Clone,
    {
        let handler = Box::new(move |cx: &mut EventContext| {
            let store = cx.data.get_store_mut();

            // Check if this cancellation is for the current load
            if let Some(current_load_id) = store.get_async_load_id(&signal_id) {
                if current_load_id != load_id {
                    return; // Different load, ignore
                }
            }

            // Restore to previous data state or idle
            let current: Option<Async<T, E>> = store
                .get_by_id::<Async<T, E>>(&signal_id)
                .cloned();

            let new_state = match current {
                Some(Async::Reloading(data)) => Async::Ready(data),
                Some(Async::Loading) => Async::Idle,
                other => other.unwrap_or(Async::Idle),
            };
            store.set_by_id(&signal_id, new_state);
        });
        Self { handler }
    }

    /// Create completion event for a timed out load.
    pub(crate) fn timeout<T, E>(signal_id: NodeId, load_id: u64) -> Self
    where
        T: 'static + Send + Clone,
        E: 'static + Send + Clone,
    {
        let handler = Box::new(move |cx: &mut EventContext| {
            let store = cx.data.get_store_mut();

            // Check if this timeout is for the current load
            if let Some(current_load_id) = store.get_async_load_id(&signal_id) {
                if current_load_id != load_id {
                    return; // Different load, ignore
                }
            }

            store.set_by_id(&signal_id, Async::<T, E>::Timeout);
        });
        Self { handler }
    }

    /// Create progress event for a retry attempt.
    pub(crate) fn retrying<T, E>(
        signal_id: NodeId,
        load_id: u64,
        attempt: u32,
        max_attempts: u32,
        last_error: E,
    ) -> Self
    where
        T: 'static + Send + Clone,
        E: 'static + Send + Clone,
    {
        let handler = Box::new(move |cx: &mut EventContext| {
            let store = cx.data.get_store_mut();

            // Check if this is still the current load
            if let Some(current_load_id) = store.get_async_load_id(&signal_id) {
                if current_load_id != load_id {
                    return; // Different load, ignore
                }
            }

            store.set_by_id(&signal_id, Async::<T, E>::Retrying(attempt, max_attempts, last_error));
        });
        Self { handler }
    }

    pub(crate) fn apply(self, cx: &mut EventContext) {
        (self.handler)(cx);
    }
}

impl std::fmt::Debug for AsyncCompletionEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncCompletionEvent").finish_non_exhaustive()
    }
}

/// Message sent from retry thread to main async loader.
enum RetryMessage<T, E> {
    /// Retry in progress: (attempt, max_attempts, last_error)
    Retrying(u32, u32, E),
    /// Final result
    Complete(Result<T, E>),
}

/// Execute loader with retry logic, sending progress updates.
fn execute_with_retry<T, E, F>(
    loader: &F,
    cancelled: &Arc<AtomicBool>,
    retries: u32,
    retry_delay: Duration,
    max_retry_delay: Duration,
    progress_tx: std::sync::mpsc::Sender<RetryMessage<T, E>>,
) where
    F: Fn() -> Result<T, E>,
    E: Clone,
{
    let max_attempts = retries.saturating_add(1);
    let mut delay = retry_delay;

    for attempt in 0..max_attempts {
        // Check cancellation before each attempt
        if cancelled.load(Ordering::SeqCst) {
            return;
        }

        match loader() {
            Ok(data) => {
                let _ = progress_tx.send(RetryMessage::Complete(Ok(data)));
                return;
            }
            Err(e) => {
                // Don't send progress or sleep after the last attempt
                if attempt + 1 < max_attempts {
                    // Send retry progress
                    let _ = progress_tx.send(RetryMessage::Retrying(
                        attempt + 2, // Next attempt number (1-indexed)
                        max_attempts,
                        e.clone(),
                    ));

                    thread::sleep(delay);
                    // Exponential backoff
                    delay = (delay * 2).min(max_retry_delay);
                } else {
                    // Final failure
                    let _ = progress_tx.send(RetryMessage::Complete(Err(e)));
                    return;
                }
            }
        }
    }
}

/// Internal function to run the async load with all options.
///
/// Note: The loader is `Fn` (not `FnOnce`) to support retry. If you need to
/// move owned data into the closure, wrap it in `Arc`.
pub(crate) fn run_async_load<T, E, F>(
    signal_id: NodeId,
    loader: F,
    options: AsyncOptions,
    handle: AsyncHandle,
    proxy: &mut crate::context::ContextProxy,
) where
    T: 'static + Send + Clone,
    E: 'static + Send + Clone,
    F: Fn() -> Result<T, E> + Send + 'static,
{
    let cancelled = handle.cancelled.clone();
    let load_id = handle.load_id;

    // Extract options for use in spawned thread
    let retries = options.retries;
    let retry_delay = options.retry_delay;
    let max_retry_delay = options.max_retry_delay;
    let timeout = options.timeout;

    // Create channel for progress and completion messages
    let (tx, rx) = std::sync::mpsc::channel();
    let cancelled_clone = cancelled.clone();

    // Spawn worker thread
    thread::spawn(move || {
        execute_with_retry(
            &loader,
            &cancelled_clone,
            retries,
            retry_delay,
            max_retry_delay,
            tx,
        );
    });

    // Process messages from worker thread
    let start = web_time::Instant::now();
    loop {
        // Calculate remaining timeout (if any)
        let recv_timeout = if let Some(total_timeout) = timeout {
            let elapsed = start.elapsed();
            if elapsed >= total_timeout {
                // Timeout expired
                cancelled.store(true, Ordering::SeqCst);
                let _ = proxy.emit(AsyncCompletionEvent::timeout::<T, E>(signal_id, load_id));
                return;
            }
            total_timeout - elapsed
        } else {
            // No timeout - use a long duration for recv
            Duration::from_secs(3600)
        };

        match rx.recv_timeout(recv_timeout) {
            Ok(RetryMessage::Retrying(attempt, max_attempts, error)) => {
                // Emit retry progress event
                let _ = proxy.emit(AsyncCompletionEvent::retrying::<T, E>(
                    signal_id,
                    load_id,
                    attempt,
                    max_attempts,
                    error,
                ));
                // Continue waiting for more messages
            }
            Ok(RetryMessage::Complete(result)) => {
                // Check if cancelled
                if cancelled.load(Ordering::SeqCst) {
                    let _ = proxy.emit(AsyncCompletionEvent::cancelled::<T, E>(signal_id, load_id));
                    return;
                }
                // Send completion
                let _ = proxy.emit(AsyncCompletionEvent::new::<T, E>(signal_id, result, load_id));
                return;
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                cancelled.store(true, Ordering::SeqCst);
                let _ = proxy.emit(AsyncCompletionEvent::timeout::<T, E>(signal_id, load_id));
                return;
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                // Worker thread exited without sending Complete (cancelled or panicked)
                if cancelled.load(Ordering::SeqCst) {
                    let _ = proxy.emit(AsyncCompletionEvent::cancelled::<T, E>(signal_id, load_id));
                } else {
                    // Thread panicked
                    let _ = proxy.emit(AsyncCompletionEvent::cancelled::<T, E>(signal_id, load_id));
                }
                return;
            }
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_states() {
        let idle: Async<i32, &str> = Async::Idle;
        assert!(idle.is_idle());
        assert!(!idle.is_loading());

        let loading: Async<i32, &str> = Async::Loading;
        assert!(loading.is_loading());
        assert!(loading.is_first_load());

        let ready: Async<i32, &str> = Async::Ready(42);
        assert!(ready.is_ready());
        assert!(ready.is_fresh());
        assert_eq!(ready.data(), Some(&42));

        let reloading: Async<i32, &str> = Async::Reloading(42);
        assert!(reloading.is_loading());
        assert!(!reloading.is_first_load());
        assert!(reloading.is_ready()); // Has data available
        assert!(reloading.is_stale());
        assert_eq!(reloading.data(), Some(&42));

        let error: Async<i32, &str> = Async::Error("failed");
        assert!(error.is_error());
        assert_eq!(error.error(), Some(&"failed"));
        assert_eq!(error.data(), None);

        let stale: Async<i32, &str> = Async::Stale(42, "refresh failed");
        assert!(stale.is_error());
        assert!(stale.is_ready()); // Still has data
        assert!(stale.is_stale());
        assert_eq!(stale.data(), Some(&42));
        assert_eq!(stale.error(), Some(&"refresh failed"));
    }

    #[test]
    fn test_async_map() {
        let ready: Async<i32, &str> = Async::Ready(21);
        let doubled = ready.map(|x| x * 2);
        assert_eq!(doubled.data(), Some(&42));

        let reloading: Async<i32, &str> = Async::Reloading(21);
        let doubled = reloading.map(|x| x * 2);
        assert_eq!(doubled.data(), Some(&42));
        assert!(matches!(doubled, Async::Reloading(42)));

        let stale: Async<i32, &str> = Async::Stale(21, "err");
        let doubled = stale.map(|x| x * 2);
        assert_eq!(doubled.data(), Some(&42));

        let error: Async<i32, &str> = Async::Error("oops");
        let mapped_err = error.map_err(|e| e.to_uppercase());
        assert_eq!(mapped_err.error(), Some(&"OOPS".to_string()));
    }

    #[test]
    fn test_async_handle() {
        let handle = AsyncHandle::new();
        assert!(!handle.is_cancelled());

        handle.cancel();
        assert!(handle.is_cancelled());
    }

    #[test]
    fn test_async_options_presets() {
        let quick = AsyncOptions::quick();
        assert_eq!(quick.timeout, Some(Duration::from_secs(5)));
        assert_eq!(quick.retries, 0);

        let patient = AsyncOptions::patient();
        assert_eq!(patient.timeout, Some(Duration::from_secs(30)));
        assert_eq!(patient.retries, 3);

        let resilient = AsyncOptions::resilient();
        assert_eq!(resilient.timeout, Some(Duration::from_secs(60)));
        assert_eq!(resilient.retries, 5);
    }
}

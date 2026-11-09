//! A lightweight profiler for the vizia update and draw pipeline.
//!
//! Enable the `profiling` feature to compile it in. Without the feature every function here is a
//! no-op, so the instrumentation in the systems costs nothing.
//!
//! Each system opens a [`Scope`] while it runs, which records how long the system took and how many
//! tree nodes it visited. Recording only happens between [`start_capture`] and [`stop_capture`].
//! The returned [`Capture`] implements [`Display`](std::fmt::Display).
//!
//! ```ignore
//! vizia::profiling::start_capture();
//! // ... trigger a state change and let the event loop process it ...
//! let capture = vizia::profiling::stop_capture();
//! println!("{capture}");
//! ```

use std::fmt;
use std::time::Duration;

/// Whether the profiler is compiled in (the `profiling` feature is enabled).
pub const ENABLED: bool = cfg!(feature = "profiling");

/// A single recorded scope.
#[derive(Debug, Clone)]
pub struct ScopeRecord {
    /// The name of the scope, usually the name of the system.
    pub name: &'static str,
    /// Nesting depth. Top-level passes (`update`, `draw`) have a depth of zero.
    pub depth: usize,
    /// Wall-clock time spent in the scope, including nested scopes.
    pub duration: Duration,
    /// Tree nodes visited while in the scope, including nested scopes.
    pub visited: u64,
}

/// The scopes recorded between [`start_capture`] and [`stop_capture`], in the order they started.
#[derive(Debug, Clone, Default)]
pub struct Capture {
    pub records: Vec<ScopeRecord>,
}

impl fmt::Display for Capture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !ENABLED {
            return writeln!(f, "vizia profiling is disabled; enable the `profiling` feature.");
        }

        writeln!(f, "{:<32} {:>10} {:>10}", "scope", "time (ms)", "visited")?;
        for (index, record) in self.records.iter().enumerate() {
            if record.depth == 0 && index != 0 {
                writeln!(f)?;
            }
            let name = format!("{}{}", "  ".repeat(record.depth), record.name);
            writeln!(
                f,
                "{:<32} {:>10.3} {:>10}",
                name,
                record.duration.as_secs_f64() * 1000.0,
                record.visited
            )?;
        }
        Ok(())
    }
}

#[cfg(feature = "profiling")]
mod imp {
    use super::{Capture, ScopeRecord};
    use std::cell::RefCell;
    use std::time::Instant;
    use vizia_storage::profiling::snapshot;

    struct OpenScope {
        index: usize,
        start: Instant,
        visited: u64,
    }

    #[derive(Default)]
    struct State {
        capturing: bool,
        // Incremented on every start/stop so that guards from a previous capture are ignored.
        generation: u64,
        records: Vec<ScopeRecord>,
        stack: Vec<OpenScope>,
    }

    thread_local! {
        static STATE: RefCell<State> = RefCell::default();
    }

    pub struct Scope {
        generation: Option<u64>,
    }

    pub fn scope(name: &'static str) -> Scope {
        STATE.with_borrow_mut(|state| {
            if !state.capturing {
                return Scope { generation: None };
            }
            let index = state.records.len();
            state.records.push(ScopeRecord {
                name,
                depth: state.stack.len(),
                duration: Default::default(),
                visited: 0,
            });
            state.stack.push(OpenScope { index, start: Instant::now(), visited: snapshot() });
            Scope { generation: Some(state.generation) }
        })
    }

    fn close(state: &mut State, open: OpenScope, now: Instant, visited: u64) {
        let record = &mut state.records[open.index];
        record.duration = now - open.start;
        record.visited = visited - open.visited;
    }

    impl Drop for Scope {
        fn drop(&mut self) {
            let Some(generation) = self.generation else { return };
            let now = Instant::now();
            let visited = snapshot();
            let _ = STATE.try_with(|state| {
                let mut state = state.borrow_mut();
                if state.generation != generation {
                    return;
                }
                if let Some(open) = state.stack.pop() {
                    close(&mut state, open, now, visited);
                }
            });
        }
    }

    pub fn start_capture() {
        STATE.with_borrow_mut(|state| {
            state.capturing = true;
            state.generation += 1;
            state.records.clear();
            state.stack.clear();
        });
    }

    pub fn stop_capture() -> Capture {
        let now = Instant::now();
        let visited = snapshot();
        STATE.with_borrow_mut(|state| {
            // Close any scopes that are still open so their records are usable.
            while let Some(open) = state.stack.pop() {
                close(state, open, now, visited);
            }
            state.capturing = false;
            state.generation += 1;
            Capture { records: std::mem::take(&mut state.records) }
        })
    }

    pub fn is_capturing() -> bool {
        STATE.with_borrow(|state| state.capturing)
    }
}

#[cfg(not(feature = "profiling"))]
mod imp {
    use super::Capture;

    pub struct Scope;

    #[inline(always)]
    pub fn scope(_name: &'static str) -> Scope {
        Scope
    }

    pub fn start_capture() {}

    pub fn stop_capture() -> Capture {
        Capture::default()
    }

    pub fn is_capturing() -> bool {
        false
    }
}

/// RAII guard returned by [`scope`]. The scope ends when the guard is dropped.
#[must_use = "the scope ends as soon as the guard is dropped"]
pub struct Scope(#[allow(dead_code)] imp::Scope);

/// Opens a profiling scope which lasts until the returned guard is dropped.
///
/// Scopes nest: a scope opened while another is active becomes its child.
#[inline(always)]
pub fn scope(name: &'static str) -> Scope {
    Scope(imp::scope(name))
}

/// Starts a new capture on the current thread, discarding any previously recorded scopes.
pub fn start_capture() {
    imp::start_capture()
}

/// Stops the current capture and returns everything recorded since [`start_capture`].
pub fn stop_capture() -> Capture {
    imp::stop_capture()
}

/// Returns true if a capture is in progress on the current thread.
pub fn is_capturing() -> bool {
    imp::is_capturing()
}

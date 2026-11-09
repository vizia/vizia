//! Optional instrumentation for counting how many tree nodes are visited by the tree iterators.
//!
//! When the `profiling` feature is enabled every tree iterator increments a global counter each
//! time it yields a node. Profilers can diff a [`snapshot`] taken before and after a piece of work
//! to determine how much of the tree that work walked. Without the feature [`record`] compiles to
//! nothing.

#[cfg(feature = "profiling")]
static VISITED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Records that a tree iterator yielded a node. No-op without the `profiling` feature.
#[inline(always)]
pub fn record() {
    #[cfg(feature = "profiling")]
    VISITED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

/// Returns the cumulative number of nodes yielded by tree iterators since startup.
pub fn snapshot() -> u64 {
    #[cfg(feature = "profiling")]
    return VISITED.load(std::sync::atomic::Ordering::Relaxed);
    #[cfg(not(feature = "profiling"))]
    0
}

/// Wraps an iterator result, recording a visit if a node was yielded.
#[inline(always)]
pub(crate) fn counted<T>(item: Option<T>) -> Option<T> {
    if item.is_some() {
        record();
    }
    item
}

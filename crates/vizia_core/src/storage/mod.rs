pub(crate) mod animatable_set;
pub(crate) mod animatable_var_set;
pub(crate) mod style_set;

use std::sync::atomic::{AtomicU64, Ordering};

/// Incremented whenever an entity gains or loses inline style data, which changes how inline
/// values are inherited and linked. Value updates of existing inline data do not affect it.
static INLINE_STRUCTURE_GENERATION: AtomicU64 = AtomicU64::new(0);

pub(crate) fn inline_structure_generation() -> u64 {
    INLINE_STRUCTURE_GENERATION.load(Ordering::Relaxed)
}

pub(crate) fn bump_inline_structure_generation() {
    INLINE_STRUCTURE_GENERATION.fetch_add(1, Ordering::Relaxed);
}

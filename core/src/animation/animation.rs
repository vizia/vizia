use std::cmp::{Eq, PartialEq};
use std::hash::Hash;

use crate::id::GenerationalId;

const ANIMATION_INDEX_BITS: u32 = 24;
const ANIMATION_INDEX_MASK: u32 = (1 << ANIMATION_INDEX_BITS) - 1;

const ANIMATION_GENERATION_BITS: u32 = 8;
const ANIMATION_GENERATION_MASK: u32 = (1 << ANIMATION_GENERATION_BITS) - 1;

/// An id used to reference style animations stored in context.
///
/// An animation id is returned by `cx.add_animation()` and can be used to configure animations
/// as well as to play, pause, and stop animations on entities (see [`AnimExt`]).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Animation(u32);

impl Default for Animation {
    fn default() -> Self {
        Animation::null()
    }
}

impl std::fmt::Display for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.index())
    }
}

impl std::fmt::Debug for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Animation {{index: {}, generation: {}}}", self.index(), self.generation())
    }
}

impl Animation {
    /// Creates a null animation id.
    ///
    /// A null animation can be used as a placeholder within a widget struct but cannot be used to
    /// get/set animation properties or control animation playback.
    pub fn null() -> Animation {
        Animation(std::u32::MAX)
    }

    /// Creates a new animation id with a given index and generation.
    pub(crate) fn new(index: u32, generation: u32) -> Animation {
        assert!(index < ANIMATION_INDEX_MASK);
        assert!(generation < ANIMATION_GENERATION_MASK);
        Animation(index | generation << ANIMATION_INDEX_BITS)
    }
}

impl GenerationalId for Animation {
    /// Create a new animation id with a given index and generation.
    fn new(index: usize, generation: usize) -> Self {
        Animation::new(index as u32, generation as u32)
    }

    /// Returns the index of the animation.
    ///
    /// This is used to retrieve animation data from the style storages in the context.
    fn index(&self) -> usize {
        (self.0 & ANIMATION_INDEX_MASK) as usize
    }

    /// Returns the generation of the animtion.
    ///
    /// This is used to determine whether or not the animation referred to by the id is 'alive'.
    fn generation(&self) -> u8 {
        ((self.0 >> ANIMATION_INDEX_BITS) & ANIMATION_GENERATION_MASK) as u8
    }

    /// Returns true if the animation is null.
    fn is_null(&self) -> bool {
        self.0 == std::u32::MAX
    }
}

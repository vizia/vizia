use std::cmp::{Eq, PartialEq};
use std::hash::Hash;

use crate::GenerationalId;

const ENTITY_INDEX_BITS: u32 = 24;
const ENTITY_INDEX_MASK: u32 = (1 << ENTITY_INDEX_BITS) - 1;

const ENTITY_GENERATION_BITS: u32 = 8;
const ENTITY_GENERATION_MASK: u32 = (1 << ENTITY_GENERATION_BITS) - 1;

/// An entity is an id used to reference to get/set properties in the context.
///
/// Rather than having widgets own their data, all state is stored in a single database and
/// is stored and loaded using entities.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(u32);

impl Default for Entity {
    fn default() -> Self {
        Entity::null()
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.index())
    }
}

impl std::fmt::Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity {{index: {}, generation: {}}}", self.index(), self.generation())
    }
}

impl Entity {
    /// Creates a null entity
    ///
    /// A null entity can be used as a placeholder within a widget struct but cannot be used to get/set properties
    pub fn null() -> Entity {
        Entity(std::u32::MAX)
    }

    /// Creates a root entity
    ///
    /// The root entity represents the main window and is always valid.
    /// The root entity can be used to set properties on the primary window, such as background color,
    /// as well as sending events to the window such as Restyle and Redraw events.
    pub fn root() -> Entity {
        Entity(0)
    }

    /// Creates a new entity with a given index and generation
    pub(crate) fn new(index: u32, generation: u32) -> Entity {
        assert!(index < ENTITY_INDEX_MASK);
        assert!(generation < ENTITY_GENERATION_MASK);
        Entity(index | generation << ENTITY_INDEX_BITS)
    }
}

impl GenerationalId for Entity {
    fn new(index: usize, generation: usize) -> Self {
        Entity::new(index as u32, generation as u32)
    }

    fn index(&self) -> usize {
        (self.0 & ENTITY_INDEX_MASK) as usize
    }

    fn generation(&self) -> u8 {
        ((self.0 >> ENTITY_INDEX_BITS) & ENTITY_GENERATION_MASK) as u8
    }

    fn is_null(&self) -> bool {
        self.0 == std::u32::MAX
    }
}

pub trait AsEntity {
    fn entity(&self) -> Entity;
}

impl AsEntity for Entity {
    fn entity(&self) -> Entity {
        *self
    }
}

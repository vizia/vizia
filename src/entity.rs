

use std::cmp::{Eq, PartialEq};
use std::hash::Hash;

use crate::GenerationalId;

const ENTITY_INDEX_BITS: u32 = 24;
const ENTITY_INDEX_MASK: u32  = (1<<ENTITY_INDEX_BITS)-1;

const ENTITY_GENERATION_BITS: u32 = 8;
const ENTITY_GENERATION_MASK: u32 = (1<<ENTITY_GENERATION_BITS)-1;

// const ENTITY_MAX: u32 = std::u32::MAX>>8;

// const MINIMUM_FREE_INDICES: usize = 1024;


/// An entity is an id used to reference to get/set properties in State.
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

    // /// Returns the index of the entity
    // pub fn index(&self) -> Option<usize> {
    //     if self.0 < std::u32::MAX {
    //         Some((self.0 & ENTITY_INDEX_MASK) as usize)
    //     } else {
    //         None
    //     }
    // }

    // /// Returns the generation of the entity
    // pub fn generation(&self) -> Option<u8> {
    //     if self.0 < std::u32::MAX {
    //         Some(((self.0 >> ENTITY_INDEX_BITS) & ENTITY_GENERATION_MASK) as u8)
    //     } else {
    //         None
    //     }
    // }

    // pub(crate) fn index_unchecked(&self) -> usize {
    //     (self.0 & ENTITY_INDEX_MASK) as usize
    // }


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

    /// Returns true if the entity is null
    fn is_null(&self) -> bool {
        self.0 == std::u32::MAX
    }
}

// /// The entity manager is responsibe for creating, destroying, and reusing entities as well as checking if an entity is 'alive'.
// pub(crate) struct EntityManager {
//     count: u32,
//     generation: Vec<u8>,
//     free_list: VecDeque<u32>,
// }

// impl EntityManager {
//     pub fn new() -> EntityManager {
//         EntityManager {
//             count: 0,
//             generation: Vec::new(),
//             free_list: VecDeque::with_capacity(MINIMUM_FREE_INDICES),
//         }
//     }

//     /// Creates a new entity, reusing a destroyed entity if the number of reusable entities is greater than MINIMUM_FREE_INDICES.
//     pub(crate) fn create_entity(&mut self) -> Option<Entity> {
//         let index = if self.free_list.len() > MINIMUM_FREE_INDICES {
//             self.free_list.pop_front()
//         } else {
//             self.generation.push(0);
//             let idx = (self.generation.len() - 1) as u32;
//             assert!((idx as u32) < ENTITY_MAX, "Entity index exceeds maximum allowed value");
//             Some(idx)
//         };

//         // Convert Option<u32> (index) to Option<Entity>
//         index.map(|idx| Entity::new(idx, self.generation[idx as usize] as u32))
//     }

//     /// Returns true is the entity is alive
//     pub fn is_alive(&self, entity: Entity) -> bool {
//         self.generation[entity.index_unchecked()] == entity.generation().unwrap()
//     }

//     /// Destroys an entity, adding it to the list of reusable entities
//     pub fn destroy_entity(&mut self, entity: Entity) {
//         let index = entity.index_unchecked() as u32;
//         assert!(self.generation[index as usize] <= std::u8::MAX, "Entity generation exceeds maximum allowed value");
//         self.generation[index as usize] += 1;
//         self.free_list.push_back(index);
//     }
// }

pub trait AsEntity {
    fn entity(&self) -> Entity;
}

impl AsEntity for Entity {
    fn entity(&self) -> Entity {
        *self
    }
}

impl AsEntity for (Entity, Entity) {
    fn entity(&self) -> Entity {
        self.0
    }
}

impl AsEntity for (Entity, Entity, Entity) {
    fn entity(&self) -> Entity {
        self.0
    }
}

impl AsEntity for (Entity, Entity, Entity, Entity) {
    fn entity(&self) -> Entity {
        self.0
    }
}

impl AsEntity for (Entity, Entity, Entity, Entity, Entity) {
    fn entity(&self) -> Entity {
        self.0
    }
}
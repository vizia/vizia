use crate::GenerationalId;
use std::{collections::VecDeque, marker::PhantomData};

const MINIMUM_FREE_INDICES: usize = 4096;
const IDX_MAX: u64 = u64::MAX >> 16;

/// The IdManager is responsible for allocating and destroying generational IDs.
///
/// The IdManager is generic on ID type, requiring only that the ID type implements [GenerationalId].
pub struct IdManager<I>
where
    I: GenerationalId,
{
    generation: Vec<u16>,
    free_list: VecDeque<u64>,

    p: PhantomData<I>,
}

impl<I> Default for IdManager<I>
where
    I: GenerationalId,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<I> IdManager<I>
where
    I: GenerationalId,
{
    pub fn new() -> Self {
        Self {
            generation: vec![0],
            free_list: VecDeque::with_capacity(MINIMUM_FREE_INDICES),

            p: PhantomData,
        }
    }

    pub fn reset(&mut self) {
        self.generation.clear();
        self.generation.push(0);
        self.free_list.clear();
    }

    /// Creates a new generational id.
    ///
    /// A generational id has an index, used for indexing into arrays, and a generation, used to check the alive status of the id.
    pub fn create(&mut self) -> I {
        let index = if self.free_list.len() >= MINIMUM_FREE_INDICES {
            self.free_list.pop_front().unwrap()
        } else {
            let idx = (self.generation.len()) as u64;
            self.generation.push(0);
            assert!(idx < IDX_MAX, "ID index exceeds maximum allowed value of {}", IDX_MAX);
            idx
        };

        I::new(index, self.generation[index as usize] as u64)
    }

    /// Destroys an ID returning false if the ID has already been destroyed.
    ///
    /// Destroyed IDs are reused after MINIMUM_FREE_INDICES are created for a single genration.
    pub fn destroy(&mut self, id: I) -> bool {
        if self.is_alive(id) {
            let index = id.index();
            assert!(index < self.generation.len(), "ID is invalid");
            assert!(self.generation[index] != u16::MAX, "ID generation is at maximum");
            self.generation[index] += 1;
            self.free_list.push_back(index as u64);
            true
        } else {
            false
        }
    }

    /// Checks if an id is alive.
    ///
    /// Works by comparing the id generation with an internal store of id generations.
    pub fn is_alive(&self, id: I) -> bool {
        if id.is_null() {
            return false;
        }
        self.generation[id.index()] == id.generation()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        impl_generational_id, GENERATIONAL_ID_GENERATION_MASK, GENERATIONAL_ID_INDEX_BITS,
        GENERATIONAL_ID_INDEX_MASK,
    };

    #[derive(Copy, Clone, PartialEq)]
    struct Entity(u64);

    impl_generational_id!(Entity);

    /// Test for creating a new IdManager.
    #[test]
    fn new() {
        let id_manager = IdManager::<Entity>::new();
        assert!(id_manager.generation.is_empty());
        assert!(id_manager.free_list.is_empty());
    }

    /// Test for creating a new id.
    #[test]
    fn create() {
        let mut id_manager = IdManager::<Entity>::new();
        let id = id_manager.create();
        assert_eq!(id, Entity::new(0, 0));
    }

    /// Test for creating mutiple ids.
    #[test]
    fn create_multiple() {
        let mut id_manager = IdManager::<Entity>::new();
        let id1 = id_manager.create();
        let id2 = id_manager.create();
        let id3 = id_manager.create();
        let id4 = id_manager.create();
        assert_eq!(id1, Entity::new(0, 0));
        assert_eq!(id2, Entity::new(1, 0));
        assert_eq!(id3, Entity::new(2, 0));
        assert_eq!(id4, Entity::new(3, 0));
    }

    /// Test for removing an id.
    #[test]
    fn destroy() {
        let mut id_manager = IdManager::<Entity>::new();
        let id = id_manager.create();
        let success = id_manager.destroy(id);
        assert!(success);
        assert_eq!(id_manager.generation[id.index()], 1);
        assert_eq!(*id_manager.free_list.front().unwrap(), id.index() as u64);
    }

    /// Test of removing an invalid id.
    #[test]
    #[should_panic]
    fn destroy_invalid() {
        let mut id_manager = IdManager::<Entity>::new();
        id_manager.destroy(Entity::new(5, 0));
    }

    /// Test of removing an already removed id.
    #[test]
    fn destroy_twice() {
        let mut id_manager = IdManager::<Entity>::new();
        let id = id_manager.create();
        id_manager.destroy(id);

        let success = id_manager.destroy(id);
        assert!(!success);
    }

    /// Test for reusing an id.
    #[test]
    fn resuse() {
        let mut id_manager = IdManager::<Entity>::new();
        let id1 = id_manager.create();
        id_manager.destroy(id1);
        let id2 = id_manager.create();
        assert_eq!(id2, Entity::new(1, 0));
        for _ in 0..MINIMUM_FREE_INDICES - 1 {
            let id = id_manager.create();
            id_manager.destroy(id);
        }

        let id3 = id_manager.create();
        assert_eq!(id3, Entity::new(0, 1));
    }

    /// Test the is_alive() method.
    #[test]
    fn alive() {
        let mut id_manager = IdManager::<Entity>::new();
        let id = id_manager.create();
        let alive1 = id_manager.is_alive(id);
        assert!(alive1);
        id_manager.destroy(id);
        let alive2 = id_manager.is_alive(id);
        assert!(!alive2);
    }
}

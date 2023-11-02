use crate::{Entry, SparseSetIndex};
use vizia_id::GenerationalId;

pub type SparseSet<V> = SparseSetGeneric<usize, V>;

/// A generic sparse set data structure.
#[derive(Debug, Clone)]
pub struct SparseSetGeneric<I, V>
where
    I: SparseSetIndex,
{
    pub sparse: Vec<I>,
    pub dense: Vec<Entry<I, V>>,
}

impl<I, V> Default for SparseSetGeneric<I, V>
where
    I: SparseSetIndex,
{
    fn default() -> Self {
        Self { sparse: Vec::new(), dense: Vec::new() }
    }
}

impl<I, V> SparseSetGeneric<I, V>
where
    I: SparseSetIndex,
{
    /// Create a new empty sparse set
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.sparse.clear();
        self.dense.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }

    /// Returns the index of the data associated with the key if it exists
    pub fn dense_idx<K: GenerationalId>(&self, key: K) -> Option<I> {
        if let Some(dense_index) = self.sparse.get(key.index()) {
            if let Some(entry) = self.dense.get(dense_index.index()) {
                if entry.key.index() == key.index() {
                    return Some(*dense_index);
                }
            }
        }
        None
    }

    /// Returns true if the sparse set contains data for the given key
    pub fn contains<K: GenerationalId>(&self, key: K) -> bool {
        self.dense_idx(key).is_some()
    }

    /// Returns a reference to the data for a given key if it exists
    pub fn get<K: GenerationalId>(&self, key: K) -> Option<&V> {
        self.dense_idx(key).map(|dense_idx| &self.dense[dense_idx.index()].value)
    }

    /// Returns a mutable reference to the data for a given key if it exists
    pub fn get_mut<K: GenerationalId>(&mut self, key: K) -> Option<&mut V> {
        self.dense_idx(key).map(move |dense_idx| &mut self.dense[dense_idx.index()].value)
    }

    /// Inserts data for a given key into the sparse set.
    ///
    /// Panics if the key is null.
    pub fn insert<K: GenerationalId>(&mut self, key: K, value: V) {
        if key.is_null() {
            panic!("Key is null");
        }

        if let Some(stored_value) = self.get_mut(key) {
            *stored_value = value;
            return;
        }

        let sparse_idx = key.index();

        if sparse_idx >= self.sparse.len() {
            self.sparse.resize(sparse_idx + 1, I::null());
        }

        self.sparse[sparse_idx] = I::new(self.dense.len());
        self.dense.push(Entry { key: I::new(sparse_idx), value });
    }

    /// Removes the data for a given key from the sparse set
    pub fn remove<K: GenerationalId>(&mut self, key: K) -> Option<V> {
        if self.contains(key) {
            let sparse_idx = key.index();
            let dense_idx = self.sparse[sparse_idx];
            let r = self.dense.swap_remove(dense_idx.index()).value;
            if dense_idx.index() < self.dense.len() {
                let swapped_entry = &self.dense[dense_idx.index()];
                self.sparse[swapped_entry.key.index()] = dense_idx;
            }

            self.sparse[sparse_idx] = I::null();

            Some(r)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sparse_set::SparseSetIndex;
    use vizia_id::{
        impl_generational_id, GenerationalId, GENERATIONAL_ID_GENERATION_MASK,
        GENERATIONAL_ID_INDEX_BITS, GENERATIONAL_ID_INDEX_MASK,
    };

    #[derive(Copy, Clone, PartialEq)]
    struct Entity(u64);

    impl_generational_id!(Entity);

    /// Test for creating a new sparse set
    #[test]
    fn new() {
        let sparse_set = SparseSetGeneric::<usize, usize>::new();

        assert!(sparse_set.sparse.is_empty());
        assert!(sparse_set.dense.is_empty());
        assert!(sparse_set.is_empty());
    }

    /// Test for adding data to a sparse set
    #[test]
    fn insert() {
        let mut sparse_set = SparseSetGeneric::<usize, usize>::new();

        sparse_set.insert(Entity::new(0, 0), 42);
        assert_eq!(sparse_set.sparse, [0]);
        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);
    }

    /// Test adding multiple items with different ids
    #[test]
    fn multiple_insert() {
        let mut sparse_set = SparseSetGeneric::<usize, usize>::new();

        sparse_set.insert(Entity::new(0, 0), 42);
        sparse_set.insert(Entity::new(1, 0), 69);
        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);
        assert_eq!(sparse_set.dense[1].key, 1);
        assert_eq!(sparse_set.dense[1].value, 69);
    }

    /// Test adding multiple items with the same id (i.e. update the value)
    #[test]
    fn overlapping_insert() {
        let mut sparse_set = SparseSetGeneric::<usize, usize>::new();

        sparse_set.insert(Entity::new(0, 0), 42);
        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);
        sparse_set.insert(Entity::new(0, 0), 69);
        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 69);
    }

    /// Test inserting data with a null id
    #[test]
    #[should_panic]
    fn insert_invalid() {
        let mut sparse_set = SparseSetGeneric::<usize, usize>::new();

        sparse_set.insert(Entity::null(), 42);
    }

    /// Test removing item when sparse set contains only one item
    #[test]
    fn remove_single() {
        let mut sparse_set = SparseSetGeneric::<usize, usize>::new();

        sparse_set.insert(Entity::new(0, 0), 42);
        assert_eq!(sparse_set.sparse, [0]);
        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);
        assert_eq!(sparse_set.remove(Entity::new(0, 0)), Some(42));
    }

    /// Test removing first of two items
    #[test]
    fn remove_first() {
        let mut sparse_set = SparseSetGeneric::<usize, usize>::new();

        sparse_set.insert(Entity::new(0, 0), 42);
        sparse_set.insert(Entity::new(1, 0), 69);
        assert_eq!(sparse_set.sparse, [0, 1]);
        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);
        assert_eq!(sparse_set.dense[1].key, 1);
        assert_eq!(sparse_set.dense[1].value, 69);
        assert_eq!(sparse_set.remove(Entity::new(0, 0)), Some(42));
    }

    /// Test removing last of two items
    #[test]
    fn remove_last() {
        let mut sparse_set = SparseSetGeneric::<usize, usize>::new();

        sparse_set.insert(Entity::new(0, 0), 42);
        sparse_set.insert(Entity::new(1, 0), 69);
        assert_eq!(sparse_set.sparse, [0, 1]);
        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);
        assert_eq!(sparse_set.dense[1].key, 1);
        assert_eq!(sparse_set.dense[1].value, 69);
        assert_eq!(sparse_set.remove(Entity::new(1, 0)), Some(69));
    }

    /// Test removing middle of three items
    #[test]
    fn remove_middle() {
        let mut sparse_set = SparseSetGeneric::<usize, usize>::new();

        sparse_set.insert(Entity::new(0, 0), 42);
        sparse_set.insert(Entity::new(1, 0), 69);
        sparse_set.insert(Entity::new(2, 0), 33);
        assert_eq!(sparse_set.sparse, [0, 1, 2]);
        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);
        assert_eq!(sparse_set.dense[1].key, 1);
        assert_eq!(sparse_set.dense[1].value, 69);
        assert_eq!(sparse_set.dense[2].key, 2);
        assert_eq!(sparse_set.dense[2].value, 33);
        assert_eq!(sparse_set.remove(Entity::new(1, 0)), Some(69));
    }

    /// Test removing item when the sparse array is actually sparse
    #[test]
    fn remove_sparse() {
        let mut sparse_set = SparseSetGeneric::<usize, usize>::new();

        sparse_set.insert(Entity::new(0, 0), 42);
        sparse_set.insert(Entity::new(12, 0), 69);
        sparse_set.insert(Entity::new(5, 0), 33);

        assert_eq!(
            sparse_set.sparse,
            [
                0,
                usize::null(),
                usize::null(),
                usize::null(),
                usize::null(),
                2,
                usize::null(),
                usize::null(),
                usize::null(),
                usize::null(),
                usize::null(),
                usize::null(),
                1
            ]
        );
        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);
        assert_eq!(sparse_set.dense[1].key, 12);
        assert_eq!(sparse_set.dense[1].value, 69);
        assert_eq!(sparse_set.dense[2].key, 5);
        assert_eq!(sparse_set.dense[2].value, 33);
        assert_eq!(sparse_set.remove(Entity::new(12, 0)), Some(69));
    }
}

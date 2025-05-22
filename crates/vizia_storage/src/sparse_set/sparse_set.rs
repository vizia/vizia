use std::{
    ops::{Deref, DerefMut},
    slice,
};

use crate::SparseSetIndex;
use vizia_id::GenerationalId;

pub type SparseSet<V> = SparseSetGeneric<usize, V>;
use super::entry::Entry;

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
        if key.is_null() {
            return None;
        }

        self.sparse.get(key.index()).copied().and_then(
            |idx| {
                if idx.is_null() {
                    None
                } else {
                    Some(idx)
                }
            },
        )
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
        if self.is_inherited(key) {
            return None;
        }

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

    pub fn inherit<K: GenerationalId>(&mut self, key: K, other: K) -> bool {
        if key == other {
            return false;
        }

        if key.is_null() || other.is_null() {
            panic!("Key is null");
        }

        if self.contains(other) {
            // If the key already has data, remove it
            if self.contains(key) {
                self.remove(key);
            }

            let sparse_idx = key.index();

            if sparse_idx >= self.sparse.len() {
                self.sparse.resize(sparse_idx + 1, I::null());
            }

            let dense_idx = self.sparse[sparse_idx];
            let other_dense_idx = self.sparse[other.index()];

            // Check if the key is already inherited from another key
            if dense_idx == other_dense_idx {
                return false;
            }

            // Update the sparse set to inherit the key
            self.sparse[sparse_idx] = other_dense_idx;

            return true;
        }

        false
    }

    /// Returns true if the key is inherited from another key
    pub fn is_inherited<K: GenerationalId>(&self, key: K) -> bool {
        if self.contains(key) {
            let sparse_idx = key.index();
            let dense_idx = self.sparse[sparse_idx];
            let entry = &self.dense[dense_idx.index()];
            return entry.key != dense_idx;
        }

        false
    }
}

/// Deref to a slice.
impl<I, T> Deref for SparseSetGeneric<I, T>
where
    I: SparseSetIndex,
{
    type Target = [Entry<I, T>];

    fn deref(&self) -> &Self::Target {
        &self.dense[..]
    }
}

/// Deref to a mutable slice.
impl<I, T> DerefMut for SparseSetGeneric<I, T>
where
    I: SparseSetIndex,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dense[..]
    }
}

/// Move into an interator, consuming the SparseSetGeneric.
impl<I, T> IntoIterator for SparseSetGeneric<I, T>
where
    I: SparseSetIndex,
{
    type Item = Entry<I, T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.dense.into_iter()
    }
}

/// An interator over the elements of the SparseSetGeneric.
impl<'a, I, T> IntoIterator for &'a SparseSetGeneric<I, T>
where
    I: SparseSetIndex,
{
    type Item = &'a Entry<I, T>;
    type IntoIter = slice::Iter<'a, Entry<I, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An interator over mutable elements of the SparseSetGeneric.
impl<'a, I, T> IntoIterator for &'a mut SparseSetGeneric<I, T>
where
    I: SparseSetIndex,
{
    type Item = &'a mut Entry<I, T>;
    type IntoIter = slice::IterMut<'a, Entry<I, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vizia_id::{
        impl_generational_id, GENERATIONAL_ID_GENERATION_MASK, GENERATIONAL_ID_INDEX_BITS,
        GENERATIONAL_ID_INDEX_MASK,
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

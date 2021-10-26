use crate::{GenerationalId};


pub trait DenseIndex: Copy + Clone {
    fn new(index: usize) -> Self;
    fn null() -> Self;
    fn index(&self) -> usize;

}

impl DenseIndex for usize {

    fn new(index: usize) -> Self {
        index
    }

    fn null() -> Self {
        std::usize::MAX
    }

    fn index(&self) -> usize {
        *self
    }
}


#[derive(Debug, PartialEq)]
pub enum SparseSetError {
    NullKey,
}


/// Represents an entry of a sparse set storing the value and the linked key
#[derive(Debug)]
pub(crate) struct Entry<T> {
    pub(crate) key: usize,
    pub(crate) value: T,
}

/// A sparse set
#[derive(Default, Debug)]
pub struct SparseSetGeneric<T, D: DenseIndex> {
    pub(crate) sparse: Vec<D>,
    pub(crate) dense: Vec<Entry<T>>,
}

impl<T, D: DenseIndex> SparseSetGeneric<T, D> 
where
    T: Default,
{

    /// Create a new empty sparse set
    pub(crate) fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
        }
    }

    pub(crate) fn clear(&mut self) {
        self.sparse.clear();
        self.dense.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }

    /// Returns the index of the data associated with the key if it exists
    pub(crate) fn dense_idx<I: GenerationalId>(&self, key: I) -> Option<D> {
        let sparse_idx = key.index();

        if sparse_idx < self.sparse.len() {
            let dense_idx = self.sparse[key.index()];
            if dense_idx.index() < self.dense.len() {
                let entry = &self.dense[dense_idx.index()];
                if entry.key == sparse_idx {
                    return Some(dense_idx);
                }
            }            
        }

        None
    }


    /// Returns true if the sparse set contains data for the given key
    pub(crate) fn contains<I: GenerationalId>(&self, key: I) -> bool {
        self.dense_idx(key).is_some()
    }

    /// Returns a reference to the data for a given key if it exists
    pub fn get<I: GenerationalId>(&self, key: I) -> Option<&T> {
        self.dense_idx(key).map(|dense_idx| &self.dense[dense_idx.index()].value)
    }

    /// Returns a mutable reference to the data for a given key if it exists
    pub(crate) fn get_mut<I: GenerationalId>(&mut self, key: I) -> Option<&mut T> {
        self.dense_idx(key).map(move |dense_idx| &mut self.dense[dense_idx.index()].value)
    }

    /// Inserts data for a given key into the sparse set
    pub fn insert<I: GenerationalId>(&mut self, key: I, value: T) -> Result<(), SparseSetError> {

        if key.is_null() {
            return Err(SparseSetError::NullKey);
        }
        

        if let Some(stored_value) = self.get_mut(key) {
            *stored_value = value;
            return Ok(());
        }

        let sparse_idx = key.index();

        if sparse_idx >= self.sparse.len() {
            self.sparse.resize(sparse_idx + 1, D::null());
        }

        self.sparse[sparse_idx] = D::new(self.dense.len());
        self.dense.push(Entry {
            key: sparse_idx,
            value,
        });

        Ok(())
    }

    /// Removes the data for a given key from the sparse set
    pub(crate) fn remove<I: GenerationalId>(&mut self, key: I) -> Option<T> {
        if self.contains(key) {
            let sparse_idx = key.index();
            let dense_idx = self.sparse[sparse_idx];
            let r = self.dense.swap_remove(dense_idx.index()).value;
            if dense_idx.index() < self.dense.len() {
                let swapped_entry = &self.dense[dense_idx.index()];
                self.sparse[swapped_entry.key] = dense_idx; 
            }

            self.sparse[sparse_idx] = D::null();

            Some(r)
        } else {
            None
        }
    }
}

pub type SparseSet<T> = SparseSetGeneric<T, usize>;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Entity;

    /// Test for creating a new sparse set
    #[test]
    fn new() {
        let sparse_set = SparseSet::<f32>::new();
        assert_eq!(sparse_set.sparse.is_empty(), true);
        assert_eq!(sparse_set.dense.is_empty(), true);
    }

    /// Test for adding data to a sparse set
    #[test]
    fn insert() {
        let mut sparse_set = SparseSet::new();
        let res = sparse_set.insert(Entity::root(), 42);

        assert_eq!(res, Ok(()));
        assert_eq!(sparse_set.sparse, [0]);
        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);
    }

    /// Test adding multiple items with different ids
    #[test]
    fn multiple_insert() {
        let mut sparse_set = SparseSet::new();
        let res1 = sparse_set.insert(Entity::new(0,0), 42);
        assert_eq!(res1, Ok(()));
        let res2 = sparse_set.insert(Entity::new(1, 0), 69);
        assert_eq!(res2, Ok(()));

        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);

        assert_eq!(sparse_set.dense[1].key, 1);
        assert_eq!(sparse_set.dense[1].value, 69);

    }

    /// Test adding multiple items with the same id (i.e. update the value)
    #[test]
    fn overlapping_insert() {
        let mut sparse_set = SparseSet::new();
        let res1 = sparse_set.insert(Entity::new(0,0), 42);
        assert_eq!(res1, Ok(()));

        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);

        let res2 = sparse_set.insert(Entity::new(0, 0), 69);
        assert_eq!(res2, Ok(()));

        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 69);
    }

    /// Test inserting data with a null id
    #[test]
    fn insert_invalid() {
        let mut sparse_set = SparseSet::new();
        let res = sparse_set.insert(Entity::null(), 42);
        assert_eq!(res, Err(SparseSetError::NullKey));
    }

    /// Test removing item when sparse set contains only one item
    #[test]
    fn remove_single() {
        let mut sparse_set = SparseSet::new();
        let res = sparse_set.insert(Entity::root(), 42);

        assert_eq!(res, Ok(()));
        assert_eq!(sparse_set.sparse, [0]);
        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);

        let ret = sparse_set.remove(Entity::root());
        assert_eq!(ret, Some(42));
        println!("{:?}", sparse_set);
    }

    /// Test removing first of two items
    #[test]
    fn remove_first() {
        let mut sparse_set = SparseSet::new();
        let res1 = sparse_set.insert(Entity::root(), 42);
        let res2 = sparse_set.insert(Entity::new(1,0), 69);

        assert_eq!(res1, Ok(()));
        assert_eq!(res2, Ok(()));
        assert_eq!(sparse_set.sparse, [0, 1]);
        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);
        assert_eq!(sparse_set.dense[1].key, 1);
        assert_eq!(sparse_set.dense[1].value, 69);

        let ret = sparse_set.remove(Entity::root());
        assert_eq!(ret, Some(42));
        println!("{:?}", sparse_set);
    }

    /// Test removing last of two items
    #[test]
    fn remove_last() {
        let mut sparse_set = SparseSet::new();
        let res1 = sparse_set.insert(Entity::root(), 42);
        let res2 = sparse_set.insert(Entity::new(1,0), 69);

        assert_eq!(res1, Ok(()));
        assert_eq!(res2, Ok(()));
        assert_eq!(sparse_set.sparse, [0, 1]);
        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);
        assert_eq!(sparse_set.dense[1].key, 1);
        assert_eq!(sparse_set.dense[1].value, 69);

        let ret = sparse_set.remove(Entity::new(1,0));
        assert_eq!(ret, Some(69));
        println!("{:?}", sparse_set);
    }

    /// Test removing middle of three items
    #[test]
    fn remove_middle() {
        let mut sparse_set = SparseSet::new();
        let res1 = sparse_set.insert(Entity::root(), 42);
        let res2 = sparse_set.insert(Entity::new(1,0), 69);
        let res3 = sparse_set.insert(Entity::new(2,0), 33);

        assert_eq!(res1, Ok(()));
        assert_eq!(res2, Ok(()));
        assert_eq!(res3, Ok(()));
        assert_eq!(sparse_set.sparse, [0, 1, 2]);
        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);
        assert_eq!(sparse_set.dense[1].key, 1);
        assert_eq!(sparse_set.dense[1].value, 69);
        assert_eq!(sparse_set.dense[2].key, 2);
        assert_eq!(sparse_set.dense[2].value, 33);

        let ret = sparse_set.remove(Entity::new(1,0));
        assert_eq!(ret, Some(69));
        println!("{:?}", sparse_set);
    }

    /// Test removing item when the sparse array is actually sparse
    #[test]
    fn remove_sparse() {
        let mut sparse_set = SparseSet::new();
        let res1 = sparse_set.insert(Entity::root(), 42);
        let res2 = sparse_set.insert(Entity::new(12,0), 69);
        let res3 = sparse_set.insert(Entity::new(5,0), 33);

        assert_eq!(res1, Ok(()));
        assert_eq!(res2, Ok(()));
        assert_eq!(res3, Ok(()));
        assert_eq!(sparse_set.sparse, [0, 
            std::usize::MAX, std::usize::MAX, 
            std::usize::MAX, std::usize::MAX, 
            2, 
            std::usize::MAX, 
            std::usize::MAX, 
            std::usize::MAX, 
            std::usize::MAX, 
            std::usize::MAX, 
            std::usize::MAX, 
            1]);
        assert_eq!(sparse_set.dense[0].key, 0);
        assert_eq!(sparse_set.dense[0].value, 42);
        assert_eq!(sparse_set.dense[1].key, 12);
        assert_eq!(sparse_set.dense[1].value, 69);
        assert_eq!(sparse_set.dense[2].key, 5);
        assert_eq!(sparse_set.dense[2].value, 33);

        let ret = sparse_set.remove(Entity::new(12,0));
        assert_eq!(ret, Some(69));

        sparse_set.insert(Entity::new(12, 1), 77);
        println!("{:?}", sparse_set);
    }



}
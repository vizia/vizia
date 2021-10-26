use crate::{Entity, GenerationalId};

pub enum StorageError {
    InvalidEntity,

}

pub struct DenseVec<T> {
    data: Vec<T>,
}

impl<T> DenseVec<T> 
where T: Default + Clone
{

    /// Creates a new instance of a DenseVec storage
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    /// Inserts an element at position `entity.index()`. If the index is greater than the length of the data array
    /// then the data array will be resized, filling the empty elements between with the default value.
    pub fn insert(&mut self, entity: &Entity, value: T) -> Result<(), StorageError> {
        let index = entity.index();
            
        if index >= self.data.len() {
            self.data.resize(index + 1, T::default());
        }

        self.data[index] = value;

        Ok(())

    }

    /// Returns a reference to an element
    pub fn get(&self, entity: &Entity) -> Option<&T> {
        self.data.get(entity.index())
    }

    /// Returns a mutable reference to an element
    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        self.data.get_mut(entity.index())
    }
}
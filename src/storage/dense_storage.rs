use crate::{Entity, GenerationalId};

#[derive(Clone, Default)]
pub struct DenseStorage<T> {
    pub indices: Vec<u32>,
    pub data: Vec<T>,
}

impl<T> DenseStorage<T>
where
    T: std::fmt::Debug + Default + Clone,
{
    pub fn new() -> Self {
        DenseStorage {
            indices: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, value: T) {
        let index = entity.index();
        if index >= self.indices.len() {
            self.indices.resize(index + 1, std::u32::MAX);
            self.indices[index] = self.data.len() as u32;
            self.data.push(value);
        } else {
            let data_index = self.indices[index] as usize;
            if data_index >= self.data.len() {
                self.indices[index] = self.data.len() as u32;
                self.data.push(value);
            } else {
                self.data[data_index] = value;
            }
        }
        
    }

    pub fn remove(&mut self, _entity: Entity) {}

    pub fn get(&self, entity: Entity) -> Option<&T> {
        if entity.index() >= self.indices.len() {
            return None;
        }

        let data_index = self.indices[entity.index()] as usize;

        if data_index >= self.data.len() {
            return None;
        }

        Some(&self.data[data_index])
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        if entity.index() >= self.indices.len() {
            return None;
        }

        let data_index = self.indices[entity.index()] as usize;

        if data_index >= self.data.len() {
            return None;
        }

        Some(&mut self.data[data_index])
    }

    pub fn set(&mut self, entity: Entity, value: T) {
        if entity.index() >= self.indices.len() {
            self.insert(entity, value);
            return;
        }

        let index = entity.index();
        let data_index = self.indices[index] as usize;

        if data_index >= self.data.len() {
            self.insert(entity, value);
            return;
        }

        self.data[data_index] = value;
        
    }

    pub fn size(&self) -> usize {
        return self.data.len() * std::mem::size_of::<T>()
            + self.indices.len() * std::mem::size_of::<usize>();
    }
}

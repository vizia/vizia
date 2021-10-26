use crate::{Entity, GenerationalId};

const INDEX_BITS: u32 = 20;
const INDEX_MASK: u32  = (1<<INDEX_BITS)-1;

const COUNT_BITS: u32 = 12;
const COUNT_MASK: u32 = (1<<COUNT_BITS)-1;

struct Index(u32);

// TODO - Bounds checks
impl Index {
    fn new(index: u32, count: u32) -> Self {
        Self(index | count << INDEX_BITS)
    }

    fn  null() -> Self {
        Self(std::u32::MAX)
    }

    fn is_null(&self) -> bool {
        self.0 == std::u32::MAX
    }

    fn set_index(&mut self, index: u32) {
        if self.is_null() {
            self.0 = index;
        } else {
            let count = self.count();
            self.0 = index | count << INDEX_BITS;
        }
    }

    fn index(&self) -> usize {
        if self.0 == std::u32::MAX {
            std::usize::MAX
        } else {
            (self.0 & INDEX_MASK) as usize
        }
    }

    fn count(&self) -> u32 {
        if self.0 == std::u32::MAX {
            self.0
        } else {
            ((self.0 >> INDEX_BITS) & COUNT_MASK) as u32
        }
    }
}

impl Clone for Index {
    fn clone(&self) -> Self {
        if self.is_null() {
            Self::null()
        } else {
            let count = self.count() + 1;
            Self::new(self.index() as u32, count)
        }
    }
}


pub struct SharedSet<T> {
    indices: Vec<Index>,
    pub data: Vec<T>,
}

impl<T> SharedSet<T> {
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, data: T) {
        if entity.index() >= self.indices.len() {
            self.indices.resize(entity.index() + 1, Index::null());
        }

        let data_index = self.indices[entity.index()].index();

        if data_index < self.data.len() {
            self.data[data_index] = data;
        } else {
            self.indices[entity.index()] = Index::new(self.data.len() as u32, 0);
            self.data.push(data);
        }
    }

    pub fn remove(&mut self, entity: Entity) {
        if entity.index() >= self.indices.len() {
            return;
        }

        let data_index = self.indices[entity.index()].index();

        if data_index >= self.data.len() {
            return;
        }

        let data_count = self.indices[entity.index()].count();

        if data_count == 1 {
            for item in self.indices.iter_mut() {
                if item.index() == self.data.len() - 1 {
                    item.set_index(data_index as u32);
                }
            }

            self.data.swap_remove(data_index);
            self.indices[entity.index()] = Index::null();
        } else {
            self.indices[entity.index()] = Index::new(data_index as u32, (data_count - 1) as u32);
        }
    }

    // Sets data for entity1 to be the same as entity2
    pub fn set_data_index(&mut self, entity1: Entity, entity2: Entity) {
        if entity2.index() >= self.indices.len() {
            return;
        }

        if entity1.index() >= self.indices.len() {
            self.indices.resize(entity1.index() + 1, Index::null());
        }
        
        let data_index2 = self.indices[entity2.index()].clone();

        if data_index2.index() >= self.data.len() {
            return;
        }

        self.indices[entity1.index()] = data_index2;
        
    } 

    pub fn get(&self, entity: Entity) -> Option<&T> {
        if entity.index() < self.indices.len() {
            let data_index = self.indices[entity.index()].index();
            if data_index < self.data.len() {
                return Some(&self.data[data_index]);
            }
        }

        None
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        if entity.index() < self.indices.len() {
            let data_index = self.indices[entity.index()].index();
            if data_index < self.data.len() {
                return Some(&mut self.data[data_index]);
            }
        }

        None
    }

    pub fn get_index(&self, entity: Entity) -> Option<usize> {
        if entity.index() < self.indices.len() {
            Some(self.indices[entity.index()].index())
        } else {
            None
        }
    } 
}

impl<T> Default for SharedSet<T> {
    fn default() -> Self {
        Self {
            indices: Vec::new(),
            data: Vec::new(),
        }
    }
}


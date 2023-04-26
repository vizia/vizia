use crate::prelude::*;
use crate::style::Rule;
use vizia_id::GenerationalId;

use vizia_storage::{SparseSetGeneric, SparseSetIndex};

const INDEX_MASK: u32 = u32::MAX / 4;
const INLINE_MASK: u32 = 1 << 31;
const INHERITED_MASK: u32 = 1 << 30;

/// Represents an index that can either be used to retrieve inline or shared data
///
/// Since inline data will override shared data, this allows the same index to be used
/// with a flag to indicate which data the index refers to.
/// The first bit of the u32 internal value is used to signify if the data index
/// refers to shared (default) or inline data:
/// - 0 - shared
/// - 1 - inline
#[derive(Debug, Clone, Copy, PartialEq)]
struct DataIndex(u32);

impl DataIndex {
    /// Create a new data index with the first bit set to 1, indicating that
    /// the index refers to inline data.
    pub fn inline(index: usize) -> Self {
        assert!((index as u32) < INDEX_MASK);
        let value = (index as u32) | INLINE_MASK;
        Self(value)
    }

    pub fn inherited(self) -> Self {
        let value = self.0;
        Self(value | INHERITED_MASK)
    }

    /// Create a new data index with the first bit set to 0, indicating that
    /// the index refers to shared data.
    pub fn shared(index: usize) -> Self {
        assert!((index as u32) < INDEX_MASK);
        Self(index as u32)
    }

    /// Retrieve the inline or shared data index.
    pub fn index(&self) -> usize {
        (self.0 & INDEX_MASK) as usize
    }

    /// Returns true if the data index refers to inline data.
    pub fn is_inline(&self) -> bool {
        (self.0 & INLINE_MASK).rotate_left(1) != 0
    }

    /// Returns true if the data index refers to an inherited value
    pub fn is_inherited(&self) -> bool {
        (self.0 & INHERITED_MASK).rotate_left(2) != 0
    }

    /// Create a null data index.
    ///
    /// A null data index is used to signify that the index refers to no data.
    pub fn null() -> Self {
        Self(u32::MAX >> 1)
    }
}

/// An Index is used by the AnimatableStorage and contains a data index and an animation index.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Index {
    data_index: DataIndex,
    anim_index: u32,
}

impl Default for Index {
    fn default() -> Self {
        Index { data_index: DataIndex::null(), anim_index: u32::MAX }
    }
}

impl SparseSetIndex for Index {
    fn new(index: usize) -> Self {
        Index { data_index: DataIndex::inline(index), anim_index: u32::MAX }
    }

    fn null() -> Self {
        Self::default()
    }

    fn index(&self) -> usize {
        self.data_index.index()
    }
}

/// Animatable storage is used for storing inline and shared data for entities as well as definitions for
/// animations, which can be played for entities, and transitions, which play when an entity matches a new shared style
/// rule which defines a trnasition.
///
/// Animations are moved from animations to active_animations when played. This allows the active
/// animations to be quickly iterated to update the value.
pub struct StyleSet<T> {
    /// Shared data determined by style rules
    shared_data: SparseSetGeneric<Index, T>,
    /// Inline data defined on specific entities
    inline_data: SparseSetGeneric<Index, T>,
}

impl<T> Default for StyleSet<T> {
    fn default() -> Self {
        Self { shared_data: Default::default(), inline_data: Default::default() }
    }
}

impl<T> StyleSet<T>
where
    T: 'static + std::fmt::Debug,
{
    /// Create a new empty styleset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert an inline value for an entity.
    pub fn insert(&mut self, entity: Entity, value: T) {
        self.inline_data.insert(entity, value);
    }

    /// Remove an entity and any inline data.
    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let entity_index = entity.index();

        if entity_index < self.inline_data.sparse.len() {
            let data_index = self.inline_data.sparse[entity_index].data_index;
            if data_index.is_inline() && !data_index.is_inherited() {
                self.inline_data.remove(entity)
            } else {
                self.inline_data.sparse[entity_index] = Index::null();
                None
            }
        } else {
            None
        }
    }

    pub fn inherit_inline(&mut self, entity: Entity, parent: Entity) -> bool {
        let entity_index = entity.index();
        let parent_index = parent.index();

        if parent_index < self.inline_data.sparse.len() {
            let parent_sparse_index = self.inline_data.sparse[parent_index];

            if parent_sparse_index.data_index.is_inline()
                && parent_sparse_index.data_index.index() < self.inline_data.dense.len()
            {
                if entity_index >= self.inline_data.sparse.len() {
                    self.inline_data.sparse.resize(entity_index + 1, Index::null());
                }

                let entity_sparse_index = self.inline_data.sparse[entity_index];

                if entity_sparse_index.data_index.is_inline()
                    && entity_sparse_index.data_index.index() < self.inline_data.dense.len()
                {
                    if entity_sparse_index.data_index.is_inherited() {
                        self.inline_data.sparse[entity_index] = Index {
                            data_index: DataIndex::inline(parent_sparse_index.data_index.index())
                                .inherited(),
                            anim_index: u32::MAX,
                        };
                        return true;
                    }
                } else {
                    self.inline_data.sparse[entity_index] = Index {
                        data_index: DataIndex::inline(parent_sparse_index.data_index.index())
                            .inherited(),
                        anim_index: u32::MAX,
                    };
                    return true;
                }
            }
        }

        false
    }

    pub fn inherit_shared(&mut self, entity: Entity, parent: Entity) -> bool {
        let entity_index = entity.index();
        let parent_index = parent.index();

        if parent_index < self.inline_data.sparse.len() {
            let parent_sparse_index = self.inline_data.sparse[parent_index];

            if !parent_sparse_index.data_index.is_inline()
                && parent_sparse_index.data_index.index() < self.shared_data.dense.len()
            {
                if entity_index >= self.inline_data.sparse.len() {
                    self.inline_data.sparse.resize(entity_index + 1, Index::null());
                }

                let entity_sparse_index = self.inline_data.sparse[entity_index];

                if !entity_sparse_index.data_index.is_inline()
                    && entity_sparse_index.data_index.index() < self.shared_data.dense.len()
                {
                    if entity_sparse_index.data_index.is_inherited() {
                        self.inline_data.sparse[entity_index] = Index {
                            data_index: DataIndex::shared(parent_sparse_index.data_index.index())
                                .inherited(),
                            anim_index: u32::MAX,
                        };
                        return true;
                    }
                } else {
                    if !entity_sparse_index.data_index.is_inline() {
                        self.inline_data.sparse[entity_index] = Index {
                            data_index: DataIndex::shared(parent_sparse_index.data_index.index())
                                .inherited(),
                            anim_index: u32::MAX,
                        };
                    }
                    return true;
                }
            }
        }

        false
    }

    pub fn clear_rules(&mut self) {
        // Remove transitions (TODO)
        for _index in self.shared_data.sparse.iter() {
            //let anim_index = index.anim_index as usize;
        }

        self.shared_data.clear();

        for index in self.inline_data.sparse.iter_mut() {
            if !index.data_index.is_inline() {
                index.data_index = DataIndex::null();
            }
        }
    }

    pub(crate) fn insert_rule(&mut self, rule: Rule, value: T) {
        self.shared_data.insert(rule, value);
    }

    // pub(crate) fn remove_rule(&mut self, rule: Rule) -> Option<T> {
    //     self.shared_data.remove(rule)
    // }

    /// Returns a reference to any inline data on the entity if it exists.
    pub fn get_inline(&self, entity: Entity) -> Option<&T> {
        let entity_index = entity.index();
        if entity_index < self.inline_data.sparse.len() {
            let data_index = self.inline_data.sparse[entity_index].data_index;
            if data_index.is_inline() {
                return self.inline_data.get(entity);
            }
        }

        None
    }

    /// Returns a mutable reference to any inline data on the entity if it exists.
    pub fn get_inline_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let entity_index = entity.index();
        if entity_index < self.inline_data.sparse.len() {
            let data_index = self.inline_data.sparse[entity_index].data_index;
            if data_index.is_inline() {
                return self.inline_data.get_mut(entity);
            }
        }

        None
    }

    // /// Returns a reference to any shared data for a given rule if it exists.
    // pub(crate) fn get_shared(&self, rule: Rule) -> Option<&T> {
    //     self.shared_data.get(rule)
    // }

    // /// Returns a mutable reference to any shared data for a given rule if it exists.
    // pub(crate) fn get_shared_mut(&mut self, rule: Rule) -> Option<&mut T> {
    //     self.shared_data.get_mut(rule)
    // }

    /// Get the animated, inline, or shared data value from the storage.
    pub fn get(&self, entity: Entity) -> Option<&T> {
        let entity_index = entity.index();
        if entity_index < self.inline_data.sparse.len() {
            let data_index = self.inline_data.sparse[entity_index].data_index;
            if data_index.is_inline() {
                if data_index.index() < self.inline_data.dense.len() {
                    return Some(&self.inline_data.dense[data_index.index()].value);
                }
            } else if data_index.index() < self.shared_data.dense.len() {
                return Some(&self.shared_data.dense[data_index.index()].value);
            }
        }

        None
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let entity_index = entity.index();
        if entity_index < self.inline_data.sparse.len() {
            let data_index = self.inline_data.sparse[entity_index].data_index;
            if data_index.is_inline() && data_index.index() < self.inline_data.dense.len() {
                return Some(&mut self.inline_data.dense[data_index.index()].value);
            }
        }

        None
    }

    /// Link an entity to some shared data.
    pub(crate) fn link(&mut self, entity: Entity, rules: &[Rule]) -> bool {
        let entity_index = entity.index();

        // Check if the entity already has some data
        if entity_index < self.inline_data.sparse.len() {
            let data_index = self.inline_data.sparse[entity_index].data_index;
            // If the data is inline then skip linking as inline data overrides shared data
            if data_index.is_inline() && !data_index.is_inherited() {
                return false;
            }
        }

        // Loop through matched rules and link to the first valid rule
        for rule in rules.iter() {
            if let Some(shared_data_index) = self.shared_data.dense_idx(*rule) {
                // If the entity doesn't have any previous shared data then create space for it
                if entity_index >= self.inline_data.sparse.len() {
                    self.inline_data.sparse.resize(entity_index + 1, Index::null());
                }

                let data_index = self.inline_data.sparse[entity_index].data_index;
                // Already linked
                if !data_index.is_inline() && data_index.index() == shared_data_index.index() {
                    return false;
                }

                self.inline_data.sparse[entity_index].data_index =
                    DataIndex::shared(shared_data_index.index());

                return true;
            }
        }

        // No matching rules so set if the data is shared set the index to null if not already null
        if entity_index < self.inline_data.sparse.len() {
            let data_index = self.inline_data.sparse[entity_index].data_index;
            if !data_index.is_inline()
                && !data_index.is_inherited()
                && self.inline_data.sparse[entity_index].data_index != DataIndex::null()
            {
                self.inline_data.sparse[entity_index].data_index = DataIndex::null();
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // DataIndex tests

    /// Test for creating an inline data index and retrieving the index.
    #[test]
    fn inline() {
        let data_index = DataIndex::inline(5);
        assert_eq!(data_index.0, INLINE_MASK + 5);
        assert_eq!(data_index.index(), 5);
    }

    /// Test that an invalid (too large) inline index causes a panic.
    #[test]
    #[should_panic]
    fn invalid_inline() {
        DataIndex::inline(std::usize::MAX);
    }

    /// Test for creating a shared data index and retrieving the index.
    #[test]
    fn shared() {
        let data_index = DataIndex::shared(5);
        assert_eq!(data_index.0, 5);
        assert_eq!(data_index.index(), 5);
    }

    /// Test that an invalid (too large) shared index causes a panic.
    #[test]
    #[should_panic]
    fn invalid_shared() {
        DataIndex::shared(std::usize::MAX);
    }

    /// Test of the is_inline() method.
    #[test]
    fn is_inline() {
        let data_index1 = DataIndex::inline(5);
        assert_eq!(data_index1.is_inline(), true);
        let data_index2 = DataIndex::shared(5);
        assert_eq!(data_index2.is_inline(), false);
    }

    /// Test that a null data index is the correct value #7FFFFFFF (i.e. all bits = 1 except the first bit).
    #[test]
    fn null() {
        let data_index = DataIndex::null();
        assert_eq!(data_index.0, 2147483647);
    }

    // AnimatableStorage tests

    /// Test for constructing a new empty animatable storage.
    #[test]
    fn new() {
        let animatable_storage = StyleSet::<f32>::new();
        assert_eq!(animatable_storage.inline_data.is_empty(), true);
        assert_eq!(animatable_storage.shared_data.is_empty(), true);
    }

    /// Test inserting inline data into the storage.
    #[test]
    fn insert_inline() {
        let mut animatable_storage = StyleSet::new();
        animatable_storage.insert(Entity::root(), 5.0);
        //assert_eq!(animatable_storage.entity_indices.first().unwrap().data_index, DataIndex::inline(0));
    }
}

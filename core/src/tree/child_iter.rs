use crate::{Tree, Entity, GenerationalId};

/// Iterator for iterating through the children of an entity.
pub struct ChildIterator<'a> {
    pub tree: &'a Tree,
    pub current_forward: Option<Entity>,
    pub current_backward: Option<Entity>,
}

impl<'a> Iterator for ChildIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entity) = self.current_forward {
            self.current_forward = self.tree.get_next_sibling(entity);
            return Some(entity);
        }

        None
    }
}

impl<'a> DoubleEndedIterator for ChildIterator<'a> {
    fn next_back(&mut self) -> Option<Entity> {
        if let Some(entity) = self.current_backward {
            self.current_backward = self.tree.prev_sibling[entity.index()];
            return Some(entity);
        }

        None
    }
}
use crate::{Tree, Entity};


/// Iterator for iterating through the ancestors of an entity
pub struct ParentIterator<'a> {
    pub(crate) tree: &'a Tree,
    pub(crate) current: Option<Entity>,
}

impl<'a> Iterator for ParentIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        if let Some(entity) = self.current {
            self.current = self.tree.get_parent(entity);
            return Some(entity);
        }

        None
    }
}
use crate::Tree;
use vizia_id::GenerationalId;

/// Iterator for iterating through the ancestors of an entity.
pub struct ParentIterator<'a, I>
where
    I: GenerationalId,
{
    tree: &'a Tree<I>,
    current: Option<I>,
}

impl<'a, I> ParentIterator<'a, I>
where
    I: GenerationalId,
{
    pub fn new(tree: &'a Tree<I>, current: Option<I>) -> Self {
        Self { tree, current }
    }
}

impl<I> Iterator for ParentIterator<'_, I>
where
    I: GenerationalId,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entity) = self.current {
            self.current = self.tree.get_parent(entity);
            Some(entity)
        } else {
            None
        }
    }
}

pub struct LayoutParentIterator<'a, I>
where
    I: GenerationalId,
{
    tree: &'a Tree<I>,
    current: Option<I>,
}

impl<'a, I> LayoutParentIterator<'a, I>
where
    I: GenerationalId,
{
    pub fn new(tree: &'a Tree<I>, current: I) -> Self {
        Self { tree, current: Some(current) }
    }
}

impl<I> Iterator for LayoutParentIterator<'_, I>
where
    I: GenerationalId,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entity) = self.current {
            self.current = self.tree.get_layout_parent(entity);
            Some(entity)
        } else {
            None
        }
    }
}

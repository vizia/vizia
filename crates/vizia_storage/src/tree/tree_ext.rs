use crate::{LayoutChildIterator, LayoutParentIterator, Tree, TreeIterator};
use vizia_id::GenerationalId;

/// Trait which provides methods for querying the tree.
pub trait TreeExt<I>
where
    I: GenerationalId,
{
    fn parent(&self, tree: &Tree<I>) -> Option<I>;
    fn first_child(&self, tree: &Tree<I>) -> Option<I>;
    fn is_sibling(&self, tree: &Tree<I>, entity: I) -> bool;
    fn is_child_of(&self, tree: &Tree<I>, entity: I) -> bool;
    fn is_descendant_of(&self, tree: &Tree<I>, entity: I) -> bool;

    fn parent_iter<'a>(&self, tree: &'a Tree<I>) -> LayoutParentIterator<'a, I>;
    fn child_iter<'a>(&self, tree: &'a Tree<I>) -> LayoutChildIterator<'a, I>;
    fn tree_iter<'a>(&self, tree: &'a Tree<I>) -> TreeIterator<'a, I>;
    fn branch_iter<'a>(&self, tree: &'a Tree<I>) -> TreeIterator<'a, I>;
}

impl<I> TreeExt<I> for I
where
    I: GenerationalId,
{
    fn parent(&self, tree: &Tree<Self>) -> Option<Self> {
        tree.get_parent(*self)
    }

    fn first_child(&self, tree: &Tree<Self>) -> Option<Self> {
        tree.get_first_child(*self)
    }

    fn is_sibling(&self, tree: &Tree<Self>, entity: Self) -> bool {
        tree.is_sibling(*self, entity)
    }

    fn is_child_of(&self, tree: &Tree<Self>, entity: Self) -> bool {
        if *self == Self::null() {
            return false;
        }

        if let Some(parent) = tree.get_parent(*self) {
            parent == entity
        } else {
            false
        }
    }

    fn is_descendant_of(&self, tree: &Tree<Self>, entity: Self) -> bool {
        if *self == Self::null() {
            return false;
        }

        for parent in self.parent_iter(tree) {
            if parent == entity {
                return true;
            }
        }

        false
    }

    fn parent_iter<'a>(&self, tree: &'a Tree<Self>) -> LayoutParentIterator<'a, I> {
        LayoutParentIterator::new(tree, Some(*self))
    }

    fn child_iter<'a>(&self, tree: &'a Tree<Self>) -> LayoutChildIterator<'a, I> {
        LayoutChildIterator::new(tree, *self)
    }

    // XXX(ollpu): Why is this defined on Entity when self isn't used?
    // The earlier behavior also seems illogical (start from the given entity but continue through
    // the whole tree)
    fn tree_iter<'a>(&self, tree: &'a Tree<Self>) -> TreeIterator<'a, I> {
        TreeIterator::full(tree)
    }

    fn branch_iter<'a>(&self, tree: &'a Tree<Self>) -> TreeIterator<'a, I> {
        TreeIterator::subtree(tree, *self)
    }
}

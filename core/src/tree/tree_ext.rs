use crate::prelude::*;

use super::child_iter::ChildIterator;
use super::parent_iter::ParentIterator;
use super::tree_iter::TreeIterator;

/// Trait which provides methods for querying the tree.
pub trait TreeExt {
    fn parent(&self, tree: &Tree) -> Option<Entity>;
    fn is_sibling(&self, tree: &Tree, entity: Entity) -> bool;
    fn is_child_of(&self, tree: &Tree, entity: Entity) -> bool;
    fn is_descendant_of(&self, tree: &Tree, entity: Entity) -> bool;

    fn parent_iter<'a>(&self, tree: &'a Tree) -> ParentIterator<'a>;
    fn child_iter<'a>(&self, tree: &'a Tree) -> ChildIterator<'a>;
    fn tree_iter<'a>(&self, tree: &'a Tree) -> TreeIterator<'a>;
    fn branch_iter<'a>(&self, tree: &'a Tree) -> TreeIterator<'a>;
}

impl TreeExt for Entity {
    fn parent(&self, tree: &Tree) -> Option<Entity> {
        tree.get_parent(*self)
    }

    fn is_sibling(&self, tree: &Tree, entity: Entity) -> bool {
        tree.is_sibling(*self, entity)
    }

    fn is_child_of(&self, tree: &Tree, entity: Entity) -> bool {
        if *self == Entity::null() {
            return false;
        }

        if let Some(parent) = tree.get_parent(*self) {
            parent == entity
        } else {
            false
        }
    }

    fn is_descendant_of(&self, tree: &Tree, entity: Entity) -> bool {
        if *self == Entity::null() {
            return false;
        }

        for parent in self.parent_iter(tree) {
            if parent == entity {
                return true;
            }
        }

        false
    }

    fn parent_iter<'a>(&self, tree: &'a Tree) -> ParentIterator<'a> {
        ParentIterator { tree, current: Some(*self) }
    }

    fn child_iter<'a>(&self, tree: &'a Tree) -> ChildIterator<'a> {
        ChildIterator::new(tree, *self)
    }

    // XXX(ollpu): Why is this defined on Entity when self isn't used?
    // The earlier behavior also seems illogical (start from the given entity but continue through
    // the whole tree)
    fn tree_iter<'a>(&self, tree: &'a Tree) -> TreeIterator<'a> {
        TreeIterator::full(tree)
    }

    fn branch_iter<'a>(&self, tree: &'a Tree) -> TreeIterator<'a> {
        TreeIterator::subtree(tree, *self)
    }
}

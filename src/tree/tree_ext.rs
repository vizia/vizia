use crate::{Entity, Tree};

use super::parent_iter::ParentIterator;
use super::child_iter::ChildIterator;
use super::branch_iter::BranchIterator;
use super::tree_iter::TreeIterator;

/// Trait which provides methods for qerying the tree.
pub trait TreeExt {
    fn parent(&self, tree: &Tree) -> Option<Entity>;
    fn is_sibling(&self, tree: &Tree, entity: Entity) -> bool;
    fn is_child_of(&self, tree: &Tree, entity: Entity) -> bool;
    fn is_descendant_of(&self, tree: &Tree, entity: Entity) -> bool;

    fn parent_iter<'a>(&self, tree: &'a Tree) -> ParentIterator<'a>;
    fn child_iter<'a>(&self, tree: &'a Tree) -> ChildIterator<'a>;
    fn tree_iter<'a>(&self, tree: &'a Tree) -> TreeIterator<'a>;
    fn branch_iter<'a>(&self, tree: &'a Tree) -> BranchIterator<'a>;
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
            if parent == entity {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
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
        ParentIterator {
            tree,
            current: Some(*self),
        }
    }

    fn child_iter<'a>(&self, tree: &'a Tree) -> ChildIterator<'a> {
        ChildIterator {
            tree,
            current_forward: tree.get_first_child(*self),
            current_backward: tree.get_last_child(*self),
        }
    }

    fn tree_iter<'a>(&self, tree: &'a Tree) -> TreeIterator<'a> {
        TreeIterator {
            tree,
            current_node: Some(*self),
        }
    }

    fn branch_iter<'a>(&self, tree: &'a Tree) -> BranchIterator<'a> {
        BranchIterator {
            tree,
            start_node: *self,
            current_node: Some(*self),
        }
    }
}

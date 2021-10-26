use morphorm::Hierarchy;

use crate::{Entity, TreeIterator, ChildIterator};

use std::iter::Rev;

impl<'a> Hierarchy<'a> for crate::Tree {
    type Item = Entity;
    type DownIter = TreeIterator<'a>;
    type UpIter = Rev<std::vec::IntoIter<Entity>>;
    type ChildIter = ChildIterator<'a>;

    fn down_iter(&'a self) -> Self::DownIter {
        TreeIterator {
            tree: self,
            current_node: Some(Entity::root()),
        }
    }

    fn up_iter(&'a self) -> Self::UpIter {
        let iterator = TreeIterator {
            tree: self,
            current_node: Some(Entity::root()),
        };
        iterator.collect::<Vec<_>>().into_iter().rev()
    }

    fn child_iter(&'a self, node: Self::Item) -> Self::ChildIter {
        ChildIterator {
            tree: self,
            current_forward: self.get_first_child(node),
            current_backward: self.get_last_child(node),
        }
    }

    fn is_first_child(&self, node: Self::Item) -> bool {
        self.is_first_child(node)
    }

    fn is_last_child(&self, node: Self::Item) -> bool {
        self.is_last_child(node)
    }

    fn parent(&self, node: Self::Item) -> Option<Self::Item> {
        self.get_parent(node)
    }
}
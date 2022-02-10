use morphorm::Hierarchy;

use crate::{ChildIterator, Entity, TreeIterator};

use std::iter::Rev;

impl<'a> Hierarchy<'a> for crate::Tree {
    type Item = Entity;
    type DownIter = TreeIterator<'a>;
    type UpIter = Rev<TreeIterator<'a>>;
    type ChildIter = ChildIterator<'a>;

    fn down_iter(&'a self) -> Self::DownIter {
        TreeIterator::full(self)
    }

    fn up_iter(&'a self) -> Self::UpIter {
        TreeIterator::full(self).rev()
    }

    fn child_iter(&'a self, node: Self::Item) -> Self::ChildIter {
        ChildIterator::new(self, node)
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

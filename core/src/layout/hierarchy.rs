use morphorm::Hierarchy;

use crate::prelude::*;

use crate::layout::{LayoutChildIterator, LayoutTreeIterator};
use std::iter::Rev;

impl<'a> Hierarchy<'a> for Tree {
    type Item = Entity;
    type DownIter = LayoutTreeIterator<'a>;
    type UpIter = Rev<LayoutTreeIterator<'a>>;
    type ChildIter = LayoutChildIterator<'a>;

    fn down_iter(&'a self) -> Self::DownIter {
        LayoutTreeIterator::full(self)
    }

    fn up_iter(&'a self) -> Self::UpIter {
        LayoutTreeIterator::full(self).rev()
    }

    fn child_iter(&'a self, node: Self::Item) -> Self::ChildIter {
        LayoutChildIterator::new(self, node)
    }

    fn is_first_child(&self, node: Self::Item) -> bool {
        self.is_first_child(node)
    }

    fn is_last_child(&self, node: Self::Item) -> bool {
        self.is_last_child(node)
    }

    fn parent(&self, node: Self::Item) -> Option<Self::Item> {
        self.get_layout_parent(node)
    }
}

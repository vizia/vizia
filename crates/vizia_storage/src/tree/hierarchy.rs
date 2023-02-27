use crate::{LayoutChildIterator, LayoutTreeIterator, Tree};
use morphorm::{Hierarchy, Node};
use std::iter::Rev;
use vizia_id::GenerationalId;

impl<'a, I> Hierarchy<'a> for Tree<I>
where
    I: GenerationalId + for<'w> Node<'w> + 'a,
{
    type Item = I;
    type DownIter = LayoutTreeIterator<'a, I>;
    type UpIter = Rev<LayoutTreeIterator<'a, I>>;
    type ChildIter = LayoutChildIterator<'a, I>;

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

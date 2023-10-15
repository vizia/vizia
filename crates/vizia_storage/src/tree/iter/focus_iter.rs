use crate::{DoubleEndedTreeTour, TourDirection, TourStep, Tree};
use vizia_id::GenerationalId;

/// Iterator for iterating through the tree in depth first preorder.
pub struct FocusTreeIterator<'a, I>
where
    I: GenerationalId,
{
    tree: &'a Tree<I>,
    tours: DoubleEndedTreeTour<I>,
    is_hidden: Box<dyn Fn(I) -> bool + 'a>,
}

impl<'a, I> FocusTreeIterator<'a, I>
where
    I: GenerationalId,
{
    pub fn new(
        tree: &'a Tree<I>,
        tours: DoubleEndedTreeTour<I>,
        is_hidden: impl Fn(I) -> bool + 'a,
    ) -> Self {
        Self { tree, tours, is_hidden: Box::new(is_hidden) }
    }

    pub fn full(tree: &'a Tree<I>, is_hidden: impl Fn(I) -> bool + 'a) -> Self {
        Self::subtree(tree, I::root(), is_hidden)
    }

    pub fn subtree(tree: &'a Tree<I>, root: I, is_hidden: impl Fn(I) -> bool + 'a) -> Self {
        Self {
            tree,
            tours: DoubleEndedTreeTour::new_same(Some(root)),
            is_hidden: Box::new(is_hidden),
        }
    }
}

impl<'a, I> Iterator for FocusTreeIterator<'a, I>
where
    I: GenerationalId,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.tours.next_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => {
                if (self.is_hidden)(node) {
                    (None, TourStep::LeaveCurrent)
                } else {
                    (Some(node), TourStep::EnterFirstChild)
                }
            }
            TourDirection::Leaving => (None, TourStep::EnterNextSibling),
        })
    }
}

impl<'a, I> DoubleEndedIterator for FocusTreeIterator<'a, I>
where
    I: GenerationalId,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tours.next_back_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => (None, TourStep::EnterLastChild),
            TourDirection::Leaving => {
                if (self.is_hidden)(node) {
                    (None, TourStep::EnterPrevSibling)
                } else {
                    (Some(node), TourStep::EnterPrevSibling)
                }
            }
        })
    }
}
